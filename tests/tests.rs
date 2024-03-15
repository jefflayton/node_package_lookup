use std::process::Command;

const BINARY_PATH: &str = "./target/debug/node_package_lookup";

#[test]
fn test_valid_package_json() {
    let output = Command::new(BINARY_PATH)
        .arg("./tests/test-package.json")
        .output()
        .expect("Failed to run node_package_lookup.");

    let output = String::from_utf8_lossy(&output.stdout);
    assert_eq!("axios | MIT\nexpress | MIT\nmongoose | MIT\n", &output);
}

#[test]
fn test_missing_package_json() {
    let output = Command::new(BINARY_PATH)
        .arg("./tests/missing-package.json")
        .output()
        .expect("Failed to run node_package_lookup.");

    let output = String::from_utf8_lossy(&output.stdout);
    assert_eq!("Failed to open file: missing-package.json\n", &output);
}

#[test]
fn test_dev_dependency() {
    let output = Command::new(BINARY_PATH)
        .arg("./tests/dev-package.json")
        .output()
        .expect("Failed to run node_package_lookup.");

    let output = String::from_utf8_lossy(&output.stdout);
    assert_eq!("dotenv | BSD-2-Clause\nnodemon | MIT\n", output);
}

#[test]
fn test_incorrect_file_extension() {
    let output = Command::new(BINARY_PATH)
        .arg("./tests/test-package.js")
        .output()
        .expect("Failed to run node_package_lookup.");

    let output = String::from_utf8_lossy(&output.stdout);
    assert_eq!("test-package.js is not a JSON file.\n", output);
}

// #[test]
// fn test_invalid_package_json() {
//     let output = Command::new(BINARY_PATH)
//     .arg("./tests/invalid-package.json")
//     .output()
//     .expect("Failed to run node_package_lookup.");

//     let output = String::from_utf8_lossy(&output.stdout);
//     assert_eq!("There was an error parsing the JSON.\n", output);

// }