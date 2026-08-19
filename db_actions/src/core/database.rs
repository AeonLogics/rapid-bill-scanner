use dirs::desktop_dir;
use std::sync::OnceLock;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv};
use tokio::runtime::Handle;

pub type LaserDatabase = Surreal<Db>;

pub static DB_CONNECTION: OnceLock<LaserDatabase> = OnceLock::new();

pub fn init_db() {
    if DB_CONNECTION.get().is_some() {
        return;
    }

    // 1. Grab Tokio's runtime handle from GPUI's environment
    let handle = Handle::current();

    // 2. Block on the async connection setup synchronously
    let conn = handle.block_on(async {
        let mut path = desktop_dir().expect("Desktop directory not found");
        path.push("laser_db");

        let db = Surreal::new::<SurrealKv>(path)
            .await
            .expect("Unable to connect to Database");

        db.use_ns("laser")
            .use_db("main")
            .await
            .expect("Failed to select namespace/database");

        db
    });

    let _ = DB_CONNECTION.set(conn);
}

pub fn get_db() -> LaserDatabase {
    if DB_CONNECTION.get().is_none() {
        init_db();
    }

    DB_CONNECTION
        .get()
        .expect("Database initialization failed")
        .clone()
}
