# Skybal

A desktop application built with Tauri, providing a native wrapper for the Skybal POS web service.

## Description

Skybal is a cross-platform desktop application developed using the Tauri framework. It provides a native desktop experience for the Skybal POS web service, leveraging Rust for the backend to ensure security and performance.

## Features

- **Single Instance**: Ensures only one instance of the application runs at a time, with focus management for subsequent launches.
- **Cross-Platform**: Runs on Windows, macOS, and Linux.
- **Lightweight**: Built with Rust for minimal resource usage.
- **Web Integration**: Seamlessly loads the Skybal POS web interface in a native window.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd skybal
   ```

2. Install dependencies:
   ```bash
   cargo install tauri-cli
   ```

## Development

To run the application in development mode:

```bash
cargo tauri dev
```

This will start the development server and open the application window.

## Building

To build the application for production:

```bash
cargo tauri build
```

The built binaries will be available in the `target/release` directory.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
