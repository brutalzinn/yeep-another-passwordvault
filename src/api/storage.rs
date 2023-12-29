
 pub mod storage {
    extern crate rusqlite;
    use rusqlite::{Connection, Result};
    #[derive(Debug)]
    pub struct KeyPair {
        pub id: i32,
        pub key: String,
        pub value: String
    }
    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keypairs (
                      id INTEGER PRIMARY KEY,
                      key TEXT NOT NULL,
                      value TEXT NOT NULL)",
            [],
        )?;
        Ok(())
    }
    
    pub fn insert_keypair(conn: &Connection, key: &str, value: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO keypairs (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }
    
    pub fn read_keypair(conn: &Connection, keypair_id: i32) -> Result<Option<KeyPair>> {
        let mut stmt = conn.prepare("SELECT id, key, value FROM keypairs WHERE id = ?1")?;
        let keypair_iter = stmt.query_map(&[&keypair_id], |row| {
            Ok(KeyPair {
                id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
            })
        })?;
    
        for keypair in keypair_iter {
            return Ok(Some(keypair?));
        }
    
        Ok(None)
    }

    pub fn read_keypairs(conn: &Connection) -> Result<Vec<KeyPair>> {
        let mut stmt = conn.prepare("SELECT id, key, value FROM keypairs")?;
        let keypair_iter = stmt.query_map([], |row| {
            Ok(KeyPair {
                id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
            })
        })?;
        let mut keypairs : Vec<KeyPair> = Vec::new();

        for keypair in keypair_iter {
            match keypair {
                Ok(p) => keypairs.push(p),
                Err(e) => eprintln!("Error: {e:?}"),
            }
        }
    
        Ok(keypairs)
    }
    
    // pub fn update_keypair(conn: &Connection, id: i32, new_value: &KeyPair) -> Result<usize> {
    //     conn.execute(
    //         "UPDATE keypairs SET name = ?1, value = ?2 WHERE id = ?3",
    //         [&new_value.key, &new_value.value]
    //     )
    // }
    
    pub fn delete_keypair(conn: &Connection, id: i32) -> Result<usize> {
        conn.execute("DELETE FROM keypairs WHERE id = ?1", &[&id])
    }
    
}

