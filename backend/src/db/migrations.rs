use crate::db::connection::DbPool;

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
    Migration {
        version: 2,
        description: "Performance indexes",
        sql: include_str!("../../migrations/002_performance_indexes.sql"),
    },
    Migration {
        version: 3,
        description: "Remove stl_thumb_path configuration",
        sql: include_str!("../../migrations/003_remove_stl_thumb_path.sql"),
    },
    Migration {
        version: 4,
        description: "Add project_previews table",
        sql: include_str!("../../migrations/004_project_previews.sql"),
    },
    Migration {
        version: 5,
        description: "Add image priority and source columns",
        sql: include_str!("../../migrations/005_stl_preview_priority.sql"),
    },
];

pub fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let current_version = get_current_version(&conn)?;

    for migration in MIGRATIONS.iter().filter(|m| m.version > current_version) {
        tracing::info!(
            "Applying migration {}: {}",
            migration.version,
            migration.description
        );
        conn.execute_batch(migration.sql)?;
    }

    Ok(())
}

fn get_current_version(conn: &rusqlite::Connection) -> Result<u32, Box<dyn std::error::Error>> {
    let version: Result<u32, rusqlite::Error> =
        conn.query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
            row.get(0)
        });

    match version {
        Ok(v) => Ok(v),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(_) => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_migrations() {
        let temp_db = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_db.path()).unwrap();

        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

        let current_version = get_current_version(&conn).unwrap();

        for migration in MIGRATIONS.iter().filter(|m| m.version > current_version) {
            conn.execute_batch(migration.sql).unwrap();
        }

        let version: u32 = conn
            .query_row("SELECT MAX(version) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();

        assert_eq!(version, 1);

        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='projects'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(table_exists);
    }
}
