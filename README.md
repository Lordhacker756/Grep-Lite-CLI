# grep_lite✨🦀
`grep_lite` is a simple command-line utility written in Rust, inspired by the classic `grep` tool. This is my first project using Rust, and it aims to provide basic text searching capabilities.

## Features

- Search for a string in a file
- Case-sensitive and case-insensitive search options
- Simple and efficient implementation

## Installation

To build and run `grep_lite`, you need to have Rust installed. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/Lordhacker756/grep_lite.git
cd grep_lite
```

Build the project using Cargo:

```sh
cargo build
```

## Usage

You can run `grep_lite` with the following command:

```sh
cargo run -- <query> <file_path>
```

For example, to search for the word "example" in a file named `sample.txt`, you would use:

```sh
cargo run -- example sample.txt
```

## Project Structure

- `src/main.rs`: The entry point of the application.
- `src/lib.rs`: Contains the core functionality of the `grep_lite` library.
- `Cargo.toml`: The configuration file for the Rust project.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [The Rust Programming Language](https://doc.rust-lang.org/book/) for providing excellent documentation and learning resources.
- The Rust community for their support and contributions.

Happy coding!
