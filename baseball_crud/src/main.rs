use baseball_crud::execute_query;
use std::env;

// Runs the funtion to query the database
fn main() {
    // Gather command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} '<query>'", args[0]);
        return;
    }
    let query = &args[1..].join(" ");

    // Execute the query specified on the command line and print results
    match execute_query(query) {
        Ok(results) => {
            for row in results {
                println!("{:?}", row);
            }
        }
        Err(err) => eprintln!("Error executing query: {}", err),
    }
}
