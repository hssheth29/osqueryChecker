# osqueryChecker

`osqueryChecker` is a Rust-based tool designed to check if `osquery` is installed on a Linux system, and if not, automatically install it. This tool ensures that it operates with appropriate permissions by requiring sudo privileges for its execution.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- A Linux system with either a Debian-based or Fedora-based distribution.
- Rust programming environment set up, including `cargo`.
- `sudo` privileges on the system where `osqueryChecker` is to be executed.

## Installation

To install `osqueryChecker`, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/hssheth29/osqueryChecker.git
```

2. Navigate to the cloned repository directory:

```bash
cd osqueryChecker
```

3. Compile the project using Cargo:

```bash
cargo build --release
```

This command compiles the application in release mode, optimizing for performance. The compiled binary will be located in `./target/release/osquerychecker`.

## Usage

To use `osqueryChecker`, execute the binary with `sudo` to ensure it has the necessary permissions:

```bash
sudo ./target/release/osquerychecker
```

The program will check if `osquery` is installed. If it is not, it will attempt to download and install `osquery` automatically.

## Contributing

We welcome contributions to `osqueryChecker`. If you have suggestions, please:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/yourFeature`).
3. Make your changes and commit them (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/yourFeature`).
5. Open a pull request.

Please make sure to update tests as appropriate.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Getting Help

If you have questions or need further assistance, please open an issue in the project's GitHub repository.
