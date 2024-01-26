## Project Title

This repository contains the source code for a financial services gateway server.

### Prerequisites

Before running the server, ensure you have the following installed:
- Rust programming language (installation instructions below)
- Rust programming language
- Cargo (Rust's package manager and build tool)

#### Installing Rust

Rust can be installed on Windows, macOS, and Linux through a script. Run the following command in your terminal and follow the on-screen instructions:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install `rustc`, `cargo`, and other standard tools. Close your terminal and reopen it, then run `rustc --version` to confirm the installation.

### Running the Server

To run the server, follow these steps:

1. Clone the repository to your local machine.
2. Navigate to the repository directory in your terminal.
3. Build the project by running `cargo build`.
4. Once the build is complete, you can start the server with `cargo run`.

The server should now be running and accessible. Check the terminal output for the address and port it is listening on (usually `127.0.0.1:4000`).

### Additional Information

For more detailed instructions on how to interact with the server, refer to the documentation in the `docs` directory.

### Contributing

If you wish to contribute to this project, please read the `CONTRIBUTING.md` file for guidelines on how to make contributions.

### License

This project is licensed under the MIT License - see the `LICENSE` file for details.
