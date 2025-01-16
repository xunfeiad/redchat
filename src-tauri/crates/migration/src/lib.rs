pub use sea_orm_migration::prelude::*;

mod m20250112_114132_user;
mod m20250112_115302_message;
mod m20250112_115512_user_friend;
mod m20250112_115520_group;
mod m20250112_115527_user_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250112_114132_user::Migration),
            Box::new(m20250112_115302_message::Migration),
            Box::new(m20250112_115512_user_friend::Migration),
            Box::new(m20250112_115520_group::Migration),
            Box::new(m20250112_115527_user_group::Migration),
        ]
    }
}
