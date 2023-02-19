# Power Consumption Tracker

Power Consumption Tracker is a program that calculates the total wattage of the CPUs on a computer by using the crate `sysinfo`, download the actual France electricity price from the website of the CRE (Commission de la régulation de l'Énergie) and outputs every second the cost of running the computer.

## Installation

1. Clone the repository to your local machine.
2. Make sure you have Rust installed.
3. Build the program by running `cargo build --release`.
4. Run the program by running `./target/release/power-consumption-tracker`.

## Usage

The program will output the total wattage of the CPUs in your computer. The wattage is calculated by summing the power consumption of each CPU core.

## Contributing

Contributions to Power Consumption Tracker are welcome! To contribute:

1. Fork the repository.
2. Make your changes.
3. Create a pull request.

## License

Power Consumption Tracker is released under the MIT License. See `LICENSE` for more information.
