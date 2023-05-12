/* database.rs
 *
 * Copyright 2022-2023 Aman Kumar <akumar@gnome.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::{fs, fs::File, path::PathBuf};

use anyhow::Result;
use diesel::{prelude::*, r2d2, r2d2::ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use once_cell::sync::Lazy;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

static DB_PATH: Lazy<PathBuf> = Lazy::new(|| gtk::glib::user_data_dir().join("declutter"));
static POOL: Lazy<Pool> = Lazy::new(|| init_pool().expect("Failed to create a pool"));

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub(crate) fn connection() -> Pool {
    POOL.clone()
}

fn init_pool() -> Result<Pool> {
    fs::create_dir_all(&*DB_PATH)?;
    let db_path = DB_PATH.join("authenticator.db");
    if !db_path.exists() {
        File::create(&db_path)?;
    }
    let manager = ConnectionManager::<SqliteConnection>::new(db_path.to_str().unwrap());
    let pool = r2d2::Pool::builder().build(manager)?;

    {
        let mut db = pool.get()?;
        tracing::info!("Running DB Migrations...");
        db.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }
    tracing::info!("Database pool initialized.");
    Ok(pool)
}

pub(crate) fn run_migrations() -> Result<()> {
    let mut db = connection().get()?;
    db.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    Ok(())
}

pub(crate) fn reset_database() -> Result<()> {
    let db_path = DB_PATH.join("authenticator.db");
    if db_path.exists() {
        fs::remove_file(&db_path)?;
    }
    init_pool()?;
    Ok(())
}
