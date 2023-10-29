extern crate csv;
extern crate rusqlite;
use rusqlite::Connection;
use std::error::Error;
use std::fs::File;



// return either Ok()) or Err(rusqlite::Error)
fn read_csv_file(file_path: &str, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        conn.execute("INSERT INTO world_billionaires (rank, final_worth, category, person_name, age, country, city, source, self_made, gender, last_name,first_name) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", [&record[0], &record[1], &record[2], &record[3], &record[4], &record[5], &record[6], &record[7], &record[8], &record[9], &record[10], &record[11]])?;
    }
    Ok(())
}

// return either Ok(conn) or Err(rusqlite::Error)
fn create_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open("billionaires.db")?;

    conn.execute("DROP TABLE IF EXISTS world_billionaires", [])?;

    conn.execute(
        "CREATE TABLE world_billionaires (
            id INTEGER PRIMARY KEY,
            rank INTEGER,
            final_worth INTEGER,
            category TEXT,
            person_name TEXT,
            age INTEGER,
            country TEXT,
            city TEXT,
            source TEXT,
            self_made TEXT,
            gender TEXT,
            last_name TEXT,
            first_name TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn etl_process() -> Result<(), Box<dyn Error>> {
    let conn = create_database()?;
    read_csv_file("world_billionaires.csv", &conn)?;
    Ok(())
}

pub fn create_action(query: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("billionaires.db")?;
    conn.execute(query, [])?;
    println!("Create successful: {}", query);
    Ok(())
}

pub fn read_action(query: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("billionaires.db")?;
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, i32>(0)?,     // id
            row.get::<usize, i32>(1)?,     // rank
            row.get::<usize, i32>(2)?,     // finalWorth
            row.get::<usize, String>(3)?,  // category
            row.get::<usize, String>(4)?,  // personName
            row.get::<usize, i32>(5)?,     // age
            row.get::<usize, String>(6)?,  // country
            row.get::<usize, String>(7)?,  // city
            row.get::<usize, String>(8)?,  // source
            row.get::<usize, String>(9)?,  // selfMade
            row.get::<usize, String>(10)?, // gender
            row.get::<usize, String>(11)?, // lastName
            row.get::<usize, String>(12)?, // firstName
        ))
    })?;

    for row in rows {
        match row {
            Ok((
                id,
                rank,
                final_worth,
                category,
                person_name,
                age,
                country,
                city,
                source,
                self_made,
                gender,
                last_name,
                first_name,
            )) => {
                println!("Result: id={}, rank={}, finalWorth={}, category={}, personName={}, age={}, country={}, city={}, source={}, selfMade={}, gender={}, lastName={}, firstName={}",
                    id, rank, final_worth, category, person_name, age, country, city, source, self_made, gender, last_name, first_name);
            }
            Err(e) => eprintln!("Error in row: {:?}", e),
        }
    }

    Ok(())
}

pub fn update_action(query: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("billionaires.db")?;
    conn.execute(query, [])?;
    println!("Update successful: {}", query);
    Ok(())
}

pub fn delete_action(query: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("billionaires.db")?;
    conn.execute(query, [])?;
    println!("Delete successful: {}", query);
    Ok(())
}
