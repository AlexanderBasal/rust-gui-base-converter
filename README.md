# **Rust GUI Base Converter App**

Rust GUI Base Converter is a Rust-based application that converts numbers between different numeral systems: Binary, Decimal, and Hexadecimal. The application provides a simple interface for users to input a number in one format and get the equivalent values in the other formats.

<p align="center">
  <img src="https://github.com/AlexanderBasal/rust-gui-base-converter/blob/main/app.png" />
</p>

## Features

- Convert Binary to Decimal and Hexadecimal
- Convert Decimal to Binary and Hexadecimal
- Convert Hexadecimal to Binary and Decimal
- User-friendly error handling for invalid inputs

## Getting Started

### Prerequisites

Ensure you have the following installed on your system:

1. **Rust**: You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. **egui**: A GUI library for Rust. You can add `egui` to your project by including it in your `Cargo.toml` file
   
    ```toml
    [dependencies]
    egui = "0.20"
    eframe = "0.20"
    ```

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/AlexanderBasal/rust-gui-base-converter.git
    cd rust-gui-base-converter
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the application:

    ```bash
    cargo run
    ```

## Usage

Upon running the application, you will be prompted to enter a number and select the input type (Binary, Decimal, Hexadecimal). The application will then display the equivalent values in the other numeral systems.

### Example

1. **Input Type:** Binary
2. **Input Value:** `1010`

**Output:**

- Binary: `1010`
- Decimal: `10`
- Hexadecimal: `A`

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/AlexanderBasal/rust-gui-base-converter?tab=MIT-1-ov-file) file for details.

## Contact

If you have any questions or suggestions, feel free to open an issue or contact me directly at [alexanderian.basal@gmail.com].
