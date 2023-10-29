use baseball_crud::execute_query;

#[test]
fn test_baseball_structure() {
    let info = execute_query("SELECT * FROM BaseballDB LIMIT 1").unwrap();

    // Check number of columns
    assert_eq!(info[0].len(), 7);

    // Check query values
    let expected = vec![
        "Adam_Donachie".to_string(),
        "BAL".to_string(),
        "Catcher".to_string(),
        "74".to_string(),
        "180".to_string(),
        "22.99".to_string(),
        "Catcher".to_string(),
    ];

    // Check to see if query matches expected
    assert_eq!(info[0], expected);
}
