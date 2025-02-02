use minigrep::{search, search_case_insensitive};

#[test]
fn test_search() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents).unwrap()
    );
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents).unwrap()
    );
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents).unwrap()
    );
}

// #[test]
// fn test_load_file() {
//     let file_path = "test_file.txt";
//     let mut file = File::create(file_path).expect("Failed to create test file");
//     writeln!(file, "Hello, world!").expect("Failed to write to test file");

//     let content = load_file(Path::new(file_path)).expect("Failed to load file");
//     assert_eq!(content.trim(), "Hello, world!");

//     std::fs::remove_file(file_path).expect("Failed to remove test file");
// }
