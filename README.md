# R-Vision Test

## Description

This repository contains a Rust-based solution for receiving and processing data chunks using the NATS messaging system. It is designed to read files, break them into chunks, and send these chunks to a writer consumer using NATS. The writer consumer then assembles these chunks back into the original file with some replacements.

Not all additional requirements are made, but the basic architecture is ready for further iterative improvement.


## Features

* Efficient data chunking and transmission using NATS.
* Extensible processing layer for the received data chunks.
* Error handling for various edge cases.

## Dependencies

* [Rust](https://www.rust-lang.org/)
* [NATS](https://nats.io/)
* [anyhow](https://crates.io/crates/anyhow)
* [bincode](https://crates.io/crates/bincode)

## Setup

1. Install Rust and Cargo, following the instructions [here](https://www.rust-lang.org/tools/install).
2. Clone this repository: 

    ```bash
    git clone https://github.com/qqrm/r-vision-test.git
    ```

3. Navigate to the project folder and build the project:

    ```bash
    cd r-vision-test
    cargo build
    ```

## Usage

Please refer to the internal documentation and comments in the source files for specific details on how to use and extend this project.

## Testing

Run NATS.

Run the test suite using:

```bash
cargo test
```
