use revolt_database::DummyDb;

use super::AbstractMigrations;

#[async_trait]
impl AbstractMigrations for DummyDb {
    async fn migrate_database(&self) -> Result<(), ()> {
        // DummyDb specific code
        Ok(())
    }
}
