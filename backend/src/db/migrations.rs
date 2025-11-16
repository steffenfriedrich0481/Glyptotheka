use rusqlite::Connection;
use std::path::Path;

pub struct Migration {
    pub version: u32,
    pub description: &'static str,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema",
        sql: include_str!("../../migrations/001_initial.sql"),
    },
];

pub fn run_migrations<P: AsRef<Path>>(db_path: P) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;
    
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let current_version = get_current_version(&conn)?;

    for migration in MIGRATIONS.iter().filter(|m| m.version > current_version) {
        tracing::info!("Applying migration {}: {}", migration.version, migration.description);
        conn.execute_batch(migration.sql)?;
    }

    Ok(())
}

fn get_current_version(conn: &Connection) -> Result<u32, Box<dyn std::error::Error>> {
    let version: Result<u32, rusqlite::Error> = conn.query_row(
        "SELECT MAX(version) FROM schema_migrations",
        [],
        |row| row.get(0)
    );

    match version {
        Ok(v) => Ok(v),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(_) => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_migrations() {
        let temp_db = NamedTempFile::new().unwrap();
        run_migrations(temp_db.path()).unwrap();

        let conn = Connection::open(temp_db.path()).unwrap();
        let version: u32 = conn.query_row(
            "SELECT MAX(version) FROM schema_migrations",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(version, 1);

        let table_exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='projects'",
            [],
            |row| row.get(0)
        ).unwrap();

        assert!(table_exists);
    }
}
