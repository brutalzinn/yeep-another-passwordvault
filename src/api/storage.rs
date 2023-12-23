
pub mod storage {
    extern crate rusqlite;
    use rusqlite::{Connection, Result};

    pub fn init() -> Result<()> {
        let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS keypairs (
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL,
            value TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
    }
}

