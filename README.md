```markdown
# ipcalc

A Rust-based IP address calculator.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Introduction

`ipcalc` is a tool for performing IP address calculations, including subnetting and CIDR calculations.

## Features

- Calculate network address
- Determine broadcast address
- Compute number of hosts
- Subnetting support

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Steps

1. Clone the repository:
    ```sh
    git clone https://github.com/cbyam/ipcalc.git
    cd ipcalc
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the project:
    ```sh
    cargo run
    ```

## Usage

Use `ipcalc` to calculate IP addresses and subnets. Example command:

```sh
cargo run -- 192.168.1.1/24
