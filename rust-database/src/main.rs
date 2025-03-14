fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{Local, NaiveDateTime};
    use futures::TryStreamExt;
    use sqlx::{
        Connection, Error, PgConnection, Pool, Postgres, Row,
        postgres::{PgPoolOptions, PgRow},
        prelude::FromRow,
    };

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://test-user:test-passwoCrd@localhost:5432/try-rust-db";

        let connection = PgConnection::connect(url).await?;

        connection.close().await?;

        return Ok(());
    }

    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://test-user:test-password@localhost:5432/try-rust-db";

        return PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url)
            .await;
    }

    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;

        return Ok(());
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("INSERT into category(id, name, description) values('A', 'Contoh', 'Contoh');")
            .execute(&pool)
            .await?;

        pool.close().await;

        return Ok(());
    }

    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("INSERT INTO category(id, name, description) VALUES($1, $2, $3)")
            .bind("B")
            .bind("Contoh Lagi")
            .bind("Contoh Deskripsi")
            .execute(&pool)
            .await?;

        pool.close().await;

        return Ok(());
    }

    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Option<PgRow> = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");

            println!("id: {}, name: {}, description: {}", id, name, description);
        } else {
            println!("Data is not found")
        }

        return Ok(());
    }

    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: PgRow = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");
        println!("id: {}, name: {}, description: {}", id, name, description);

        return Ok(());
    }

    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("SELECT * FROM category")
            .bind("A")
            .fetch_all(&pool)
            .await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        return Ok(());
    }

    #[tokio::test]
    async fn test_fetch() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result = sqlx::query("SELECT * FROM category").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        return Ok(());
    }

    #[derive(Debug, FromRow)]
    struct Category {
        id: String,
        name: String,
        description: String,
    }

    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Category> = sqlx::query("SELECT * FROM category")
            .map(|row: PgRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }

        return Ok(());
    }

    #[tokio::test]
    async fn test_automatic_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Category> = sqlx::query_as("SELECT * FROM category")
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }

        return Ok(());
    }

    #[tokio::test]
    async fn test_insert_brand() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("INSERT INTO brands(id, name, description, created_at, updated_at) VALUES($1, $2, $3, $4, $5);").bind("A").bind("Contoh name").bind("Contoh description").bind(Local::now().naive_local()).bind(Local::now().naive_local()).execute(&pool).await?;

        return Ok(());
    }

    #[derive(Debug, FromRow)]
    struct Brand {
        id: String,
        name: String,
        description: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }

    #[tokio::test]
    async fn test_automatic_result_type_data() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Brand> = sqlx::query_as("SELECT * FROM brands")
            .fetch_all(&pool)
            .await?;

        for brand in result {
            println!("{:?}", brand);
        }

        return Ok(());
    }

    #[tokio::test]
    async fn test_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO brands(id, name, description, created_at, updated_at) VALUES($1, $2, $3, $4, $5);")
            .bind("B")
            .bind("Contoh name B")
            .bind("Contoh description B")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction)
            .await?;

        sqlx::query("INSERT INTO brands(id, name, description, created_at, updated_at) VALUES($1, $2, $3, $4, $5);")
            .bind("C")
            .bind("Contoh name C")
            .bind("Contoh description C")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    #[tokio::test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("INSERT INTO sellers(name) VALUES($1) RETURNING id;")
            .bind("Contoh name")
            .fetch_one(&pool)
            .await?;

        let id: i32 = result.get("id");
        println!("ID Seller : {}", id);

        return Ok(());
    }

    #[tokio::test]
    async fn test_auto_increment_with_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query("INSERT INTO sellers(name) VALUES($1);")
            .bind("Contoh name")
            .execute(&mut *transaction)
            .await?;

        let result = sqlx::query("SELECT lastval() as id;")
            .fetch_one(&mut *transaction)
            .await?;

        let id: i32 = result.get_unchecked("id");
        println!("ID Seller : {}", id);

        transaction.commit().await?;

        return Ok(());
    }
}
