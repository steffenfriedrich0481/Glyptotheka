use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use std::path::Path;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_pool<P: AsRef<Path>>(database_path: P) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file(database_path)
        .with_flags(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE);

    let pool = Pool::builder().max_size(16).build(manager)?;

    {
        let conn = pool.get()?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA cache_size = -64000;
             PRAGMA mmap_size = 30000000000;",
        )?;
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_pool() {
        let temp_db = NamedTempFile::new().unwrap();
        let pool = create_pool(temp_db.path()).unwrap();

        let conn = pool.get().unwrap();
        let result: i64 = conn.query_row("SELECT 1", [], |row| row.get(0)).unwrap();
        assert_eq!(result, 1);
    }
}
