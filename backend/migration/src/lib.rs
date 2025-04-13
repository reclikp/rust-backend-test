pub use sea_orm_migration::prelude::*;

mod m20250323_081950_create_post_table;
mod m20250325_192612_create_user_table;
mod m20250412_131251_add_user_roles;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250323_081950_create_post_table::Migration),
            Box::new(m20250325_192612_create_user_table::Migration),
            Box::new(m20250412_131251_add_user_roles::Migration),
        ]
    }
}
