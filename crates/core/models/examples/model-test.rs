use revolt_models::admin::migrations::MigrationInfo;

fn main() {
    println!("hello {:?}", MigrationInfo { id: 0, revision: 0 });
}
