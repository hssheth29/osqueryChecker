# osqueryChecker

This is a simple Rust program to check for the installation of osquery on your system and install it if it's not already installed.

## Requirements

- Rust programming language and Cargo package manager installed.
- `curl` and `dnf` (for Fedora-based systems) installed for downloading and installing osquery.

## Installation

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/hssheth29/osqueryChecker.git
    ```

2. Navigate to the repository directory:

    ```bash
    cd osqueryChecker
    ```

3. Compile and run the program:

    ```bash
    cargo run
    ```

## Usage

Simply run the program, and it will check if osquery is installed on your system. If not, it will proceed to download and install it.

## Contributing

Contributions are welcome! If you have any ideas, improvements, or bug fixes, feel free to open an issue or create a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
