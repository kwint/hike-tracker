use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

pub fn run_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}
