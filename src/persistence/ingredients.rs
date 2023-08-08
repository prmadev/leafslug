//! implementation of ingredient persistence for postgres

use async_trait::async_trait;
use sqlx::PgPool;

use crate::{Ingredient, CRUD};

#[async_trait]
impl CRUD<Ingredient> for PgPool {
    type Er = IngredientCRUDError;

    type ID = i32;

    /// Inserts a new item to the database.
    async fn create(&self, item: Ingredient) -> Result<Self::ID, Self::Er> {
        return Ok(match item.id {
            Some(id) => {
                #[allow(clippy::panic)] // sadly
                sqlx::query!(
                    r#"
                        INSERT INTO ingredients(id, name)
                        VALUES($1,$2)
                        RETURNING id
                    "#,
                    id,
                    item.name
                )
                .fetch_one(self)
                .await?
                .id
            }
            None => {
                #[allow(clippy::panic)] // sadly
                sqlx::query!(
                    r#"
                        INSERT INTO ingredients(name)
                        VALUES($1)
                        RETURNING id    
                    "#,
                    item.name
                )
                .fetch_one(self)
                .await?
                .id
            }
        });
    }

    /// Retrieves an item matching the Given ID.
    async fn retrieve(&self, id: Self::ID) -> Result<Ingredient, Self::Er> {
        // Fetch the ingredient
        Ok(
            #[allow(clippy::panic)] // sadly
            sqlx::query_as!(Ingredient, "SELECT * FROM ingredients WHERE id = $1", id)
                .fetch_one(self)
                .await?,
        )
    }

    /// Updates an item matching the Given ID.
    async fn update(&self, new: Ingredient) -> Result<Ingredient, Self::Er> {
        Ok(
            #[allow(clippy::panic)] // sadly
            sqlx::query_as!(
                Ingredient,
                r#"
                UPDATE ingredients
                SET name = $2
                WHERE id = $1
                RETURNING id, name
            "#,
                new.id,
                new.name
            )
            .fetch_one(self)
            .await?,
        )
    }

    /// Deletes an item matching the Given ID.
    async fn delete(&self, id: Self::ID) -> Result<(), Self::Er> {
        #[allow(clippy::panic)] // sadly
        sqlx::query_as!(Ingredient, "DELETE FROM ingredients WHERE id = $1", id)
            .execute(self)
            .await?;
        Ok(())
    }
}

/// Errors related to crud actions
#[derive(Debug, thiserror::Error)]
pub enum IngredientCRUDError {
    /// got an error when interacting with sql db
    #[error("Failed to work with sql db: {0}")]
    FailedWithThisErrorFromSQLX(#[from] sqlx::Error),
}

#[cfg(test)]
mod testing {
    #![allow(clippy::panic, clippy::expect_used)]
    use crate::Ingredient;
    use std::{assert_eq, env::var_os};

    use sqlx::{Connection, Executor, PgConnection, PgPool};

    use super::CRUD;

    async fn create_pg_pool() -> Result<PgPool, String> {
        let db_user = var_os("DATABASE_USER")
            .ok_or("could not get DATABASE_USER")?
            .into_string()
            .map_err(|_e| "could not convert to string".to_owned())?;

        let db_pass = var_os("DATABASE_PASS")
            .ok_or("could not get DATABASE_PASS")?
            .into_string()
            .map_err(|_e| "could not convert to string".to_owned())?;

        let db_address = var_os("DATABASE_ADDRESS")
            .ok_or("could not get DATABASE_ADDRESS")?
            .into_string()
            .map_err(|_e| "could not convert to string".to_owned())?;

        let db_name = time::OffsetDateTime::now_utc().unix_timestamp();
        let db_port = var_os("DATABASE_PORT")
            .ok_or("could not get DATABASE_PORT")?
            .into_string()
            .map_err(|_e| "could not convert to string".to_owned())?;

        // Create database
        let mut connection = PgConnection::connect(&format!(
            "postgres://{db_user}:{db_pass}@{db_address}:{db_port}/postgres"
        ))
        .await
        .expect("Failed to connect to Postgres");

        connection
            .execute(r#"CREATE DATABASE "{db_name}"; "#)
            .await
            .expect("Failed to create database");

        let db_url = format!("postgres://{db_user}:{db_pass}@{db_address}:{db_port}/{db_name}");

        let pool = PgPool::connect_lazy(&db_url)
            .map_err(|x| format!("got this error{x} "))
            .expect("could not get a pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate the database");
        Ok(pool)
    }

    fn basic_ingredient() -> Ingredient {
        Ingredient {
            id: None,
            name: "some name".to_owned(),
        }
    }

    #[tokio_macros::test]
    async fn testing_insert() {
        let pool = create_pg_pool().await.expect("got error when getting pool");
        let unsaved_ingredient = basic_ingredient();

        let id = pool
            .create(unsaved_ingredient.clone())
            .await
            .expect("got error when adding ingredient");

        let res = pool
            .retrieve(id)
            .await
            .expect("got error when getting ingredients");

        assert_eq!(
            id.clone(),
            res.id
                .expect("did not have id when i had to have one")
                .clone()
        );
        assert_eq!(unsaved_ingredient.name, res.name);

        let update_proposal = Ingredient {
            id: res.id,
            name: "bread".to_owned(),
        };
        let res2 = pool
            .update(update_proposal)
            .await
            .expect("must be updated but could not do it");
        assert_ne!(res2.name, res.name);
        assert_eq!(res2.name, "bread".to_owned());
        assert_eq!(res2.id, res.id);

        pool.delete(id).await.expect("got error when deleting item");
        _ = pool.retrieve(id).await.expect_err("should have got error");
    }
}
