pub mod todo_persistence {
    use rusqlite::{Connection, params, Result};

    use crate::dto::item::item::Item;

    pub fn init_db() -> Result<()> {
        create_db()
    }

    fn create_db() -> Result<()> {
        let connection = Connection::open("todo_rust.db")?;

        println!("Database and table created successfully");

        let query = "
            CREATE TABLE IF NOT EXISTS item (
            id INTEGER PRIMARY KEY AUTOINCREMENT, done INTEGER, description TEXT);
        ";

        connection.execute(
            query,
            params![],
        )?;
        Ok(())
    }

    pub fn get_all_items() -> Result<Vec<Item>> {
        let connection = Connection::open("todo_rust.db")?;

        let mut stmt = connection.prepare("SELECT id, done, description FROM item")?;

        let item_iter = stmt.query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                done: row.get::<_, i64>(1)? != 0,
                description: row.get(2)?,
            })
        })?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }

        Ok(items)
    }

    pub fn create_item(item_to_save: Item) -> Result<i64> {
        let connection = Connection::open("todo_rust.db")?;

        connection.execute( "
        INSERT INTO item (done, description) VALUES (0, ?1);
                ",
        params![item_to_save.description],
        )?;

        Ok(connection.last_insert_rowid())
    }

    pub fn complete_item(item_to_complete_id: i64) -> Result<()> {
        let connection = Connection::open("todo_rust.db")?;

        connection.execute( "
        UPDATE item SET done = 1 WHERE id = ?1;",
                            params![item_to_complete_id],
        )?;
        Ok(())
    }
}