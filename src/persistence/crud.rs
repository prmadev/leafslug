//! Simple definition for CRUD of an item

use async_trait::async_trait;

/// CRUD acts as a check list for functionality related to items in the server
#[async_trait]
pub trait CRUD<T> {
    /// Type of the error.
    type Er: std::error::Error;

    /// The type of id used in database.
    type ID;

    /// Inserts a new item to the database.
    async fn create(&self, item: T) -> Result<Self::ID, Self::Er>;

    /// Retrieves an item matching the Given ID.
    async fn retrieve(&self, id: Self::ID) -> Result<T, Self::Er>;

    /// Updates an item matching the Given ID.
    async fn update(&self, new: T) -> Result<T, Self::Er>;

    /// Deletes an item matching the Given ID.
    async fn delete(&self, id: Self::ID) -> Result<(), Self::Er>;
}
