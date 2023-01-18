use revolt_database::DummyDb;
use revolt_models::admin::migrations::{AbstractMigrations, MigrationInfo};

#[async_std::main]
async fn main() {
    let info = MigrationInfo { id: 0, revision: 0 };
    info.method_on_struct_itself();
    println!("hello {:?}", info);

    let db = DummyDb {};
    db.migrate_database().await.unwrap();
}
