use aegrep::search;

#[test]
fn search_error_no_files() {
    let query = String::from("duck");
    let files = vec![String::from("test.tmp")];
    let result = search(query, files, true, false);

    assert!(result.is_err())
}

#[test]
fn search_error_second_file_broke() {
    let query = String::from("duck");
    let files = vec![String::from("poem.txt"), String::from("test.tmp")];
    let result = search(query, files, true, false);

    assert!(result.is_err())
}

#[test]
fn search_found_a_file() {
    let query = String::from("duck");
    let files = vec![String::from("poem.txt")];
    let result = search(query, files, true, false);

    assert!(result.is_ok())
}

#[test]
fn search_found_two_file() {
    let query = String::from("duck");
    let files = vec![String::from("poem.txt"), String::from("poem.txt")];
    let result = search(query, files, true, false);

    assert!(result.is_ok())
}
