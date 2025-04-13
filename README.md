# rsfetch

rsfetch is a lightweight and customizable system information fetcher written in Rust. It displays essential system details in a clean and visually appealing format.

<!-- add status icons for the repo -->
![GitHub release](https://img.shields.io/github/v/release/BenSimmers/rsfetch)
![License](https://img.shields.io/github/license/BenSimmers/rsfetch)
## Table of Contents
- [rsfetch](#rsfetch)
  - [Table of Contents](#table-of-contents)
  - [Get latest release](#get-latest-release)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)


## Get latest release
```bash
curl -s https://api.github.com/repos/BenSimmers/rsfetch/releases/latest \
| grep "browser_download_url" \
| head -n 1 \
| cut -d '"' -f 4 \
| xargs wget
```
## Features
- Displays system information such as OS, kernel, uptime, memory usage, and more.
## Installation
1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/BenSimmers/rsfetch.git
   cd rsfetch
   ```
3. Build and install the binary:
   ```bash
   cargo install --path .
   ```

## Usage
Run `rsfetch` to display system information:
```bash
rsfetch
```

Customize the output using configuration files or command-line options (details coming soon).

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.
