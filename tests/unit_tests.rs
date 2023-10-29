use sqlite_rust::{create_action, delete_action, update_action};
extern crate rusqlite;
use rusqlite::Connection;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_action() -> Result<(), Box<dyn Error>> {
        // Arrange
        let query = "UPDATE world_billionaires SET age = 30 WHERE id = 100";

        // Act
        update_action(query)?;

        // Assert
        let conn = Connection::open("billionaires.db")?;
        let mut stmt = conn.prepare("SELECT age FROM world_billionaires WHERE id = 100")?;
        let age: i32 = stmt.query_row([], |row| row.get(0))?;
        assert_eq!(age, 30);

        Ok(())
    }

    #[test]
    fn test_delete_action() -> Result<(), Box<dyn Error>> {
        // Arrange
        let query = "DELETE FROM world_billionaires WHERE person_name = 'Larry Page'";

        // Act
        delete_action(query)?;

        // Assert
        let conn = Connection::open("billionaires.db")?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM world_billionaires WHERE person_name = 'Larry Page';",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 0);

        Ok(())
    }

    #[test]
    fn test_create_action() -> Result<(), Box<dyn Error>> {
        // Arrange
        let query = "INSERT INTO world_billionaires (person_name, final_worth, age) VALUES ('Test', 3000000, 49)";

        // Act
        create_action(query)?;

        // Assert
        let conn = Connection::open("billionaires.db")?;
        let mut stmt = conn.prepare("SELECT person_name, final_worth, age FROM world_billionaires WHERE person_name = 'Test';")?;
        let billionaire = stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
        assert_eq!(billionaire, ("Test".to_string(), 3000000, 49));

        Ok(())
    }
}
