use sqlite_rust::{create_action, delete_action, etl_process, read_action, update_action};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let action = &args[1];

        match action.as_str() {
            "etl" => {
                if let Err(err) = etl_process() {
                    eprintln!("Error Occured: {}", err);
                } else {
                    println!("Database created successfully.");
                }
            }
            _ => {
                println!("Invalid action. Use 'etl'.");
            }
        }
    } else if args.len() == 3 {
        let action = &args[1];
        let query = &args[2];

        match action.as_str() {
            "create" => {
                create_action(query)?;
                println!("Data created successfully.");
            }
            "read" => {
                read_action(query)?;
                println!("Data read successfully.");
            }
            "update" => {
                update_action(query)?;
                println!("Data updated successfully.");
            }
            "delete" => {
                delete_action(query)?;
                println!("Data deleted successfully.");
            }
            _ => {
                println!("Invalid action. Use 'extract' or 'transform_load'.");
            }
        }
    } else {
        eprintln!(
            "Invalid number of command-line arguments. Use: {} <action> [SQL query]",
            args[0]
        );
        std::process::exit(1);
    }

    Ok(())
}
