# Node Package Lookup

This is a Rust project that reads a JSON file containing JavaScript package dependencies and prints out the package name along with its license.

## Getting Started

To get a local copy up and running, follow these steps:

1. Clone the repository:
```sh
git clone https://github.com/jefflayt/node_package_lookup.git
```

2. Navigate to the project directory:
```sh
cd node_package_lookup
```

3. Build the project:
```sh
cargo build
```

## Usage

To use this project, you need to pass the path to a JSON file as an argument when running the program. The JSON file should contain JavaScript package dependencies in the following format:

```json
{
    "dependencies": {
        "package1": "version",
        "package2": "version",
        ...
    }
}
```

Here's an example of how to run the program:

```sh
cargo run path/to/your/file.json
```

This will print out the package name and its license for each package in the JSON file.

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Contributing

Contributions are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License.