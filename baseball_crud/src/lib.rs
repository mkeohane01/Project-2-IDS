use rusqlite::{Connection, Result};

// Function to execute any SQL query on the BaseballDB.db
pub fn execute_query(query: &str) -> Result<Vec<Vec<String>>> {
    // Open a connection to the database
    let conn = Connection::open("BaseballDB.db").expect("Unable to open database");

    // Execute the query and get the results
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        let mut values = vec![];
        for i in 0..row.column_count() {
            values.push(row.get(i)?);
        }
        Ok(values)
    })?;

    // Convert the results to a vector of tuples
    let mut results = vec![];
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

// // Get specific info from the baseball database
// fn query_baseball_info() -> Result<()> {
//     // Open a connection to the database
//     let conn = Connection::open("BaseballDB.db").expect("Unable to open database");

//     // Execute a query and get the results
//     let mut stmt = conn.prepare("SELECT * FROM BaseballDB LIMIT 10")?;
//     let rows = stmt.query_map([], |row| {
//         Ok((
//             row.get::<_, String>(0)?,
//             row.get::<_, String>(1)?,
//             row.get::<_, String>(2)?,
//             row.get::<_, String>(3)?,
//             row.get::<_, String>(4)?,
//             row.get::<_, String>(5)?,
//         ))

//     })?;

//     // Print out the results
//     for row in rows {
//         let (name, team, position, height, weight, age) = row?;
//         let name = name.replace("_", " ");
//         let position = position.replace("_", " ");
//         println!("Name: {}, Team: {}, Position: {}, Height: {}, Weight: {}, Age: {}", name, team, position, height, weight, age);
//     }

//     Ok(())
// }
