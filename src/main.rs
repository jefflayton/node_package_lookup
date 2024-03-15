use serde_json::Value;
use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    let file_name = path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("File not found");

    let file_extension = path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("No extension");

    if file_extension != "json" {
        println!("{} is not a JSON file.", file_name);
        return;
    }

    let file: Result<File, Error> = File::open(path);
    match file {
        Ok(mut file) => {
            let mut contents: String = String::new();
            let _ = file.read_to_string(&mut contents);

            let json_contents: Value = serde_json::from_str(&contents.as_str())
                .expect("There was an error parsing the JSON.");

            let dependencies: &Value = &json_contents["dependencies"];
            let dev_dependencies: &Value = &json_contents["devDependencies"];

            read_dependencies(dependencies);
            read_dependencies(dev_dependencies);
        }
        Err(error) => {
            println!("Failed to open file: {}", file_name);
            eprintln!("Error: {}", error);
        }
    };
}

fn read_dependencies(dependencies: &Value) {
    if let Some(dependencies) = dependencies.as_object() {
        for (package, _) in dependencies {
            let license: String = get_license(package);
            println!("{} | {}", package, license);
        }
    }
}

fn get_license(package: &String) -> String {
    let output = Command::new("npm")
        .arg("view")
        .arg(package)
        .arg("license")
        .output()
        .expect("Failed to execute command");

    let license: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

    license.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{get_license, read_dependencies};
    use serde_json;
    use serde_json::Value;

    #[test]
    fn test_read_dependencies() {
        let result = std::panic::catch_unwind(|| {
            let dependency_json = r#"
                {
                    "axios": "^0.21.1",
                    "express": "^4.17.1",
                    "mongoose": "^5.12.3"
                }
            "#;
            let dependencies: Value = serde_json::from_str(dependency_json).unwrap();
            read_dependencies(&dependencies);
        });

        match result {
            Ok(_) => {
                // Handle the case where `read_dependencies` runs successfully
                // For example, you might want to assert that it returned the correct value
            },
            Err(err) => {
                // Handle the case where `read_dependencies` panics
                let print_output = err.downcast_ref::<String>().unwrap();
                assert_eq!("axios | MIT\nexpress | MIT\nmongoose | MIT\n", print_output);
            }
        }
    }

    #[test]
    fn test_get_license() {
        let packages = vec![
            String::from("axios"),
            String::from("express"),
            String::from("mongoose"),
        ];
        for package in packages.iter() {
            let license = get_license(package);
            assert_eq!("MIT", license);
        }
    }
}
