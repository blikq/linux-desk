# linux-desk

`linux-desk` is an easy-to-use CLI tool for creating `.desktop` files for applications on Linux. `.desktop` files are used to integrate applications into the Linux desktop environment, allowing them to appear in application menus, launchers, and more.

## Features

- Create `.desktop` files with custom application names, icons, and executable paths.
- Automatically saves `.desktop` files to the appropriate directory (`~/.local/share/applications/`).
- Updates the desktop database to reflect changes immediately.
- Automatically extracts the application name from the executable path if no name is provided.
- Supports optional custom icons for the application.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/linux-desk.git
   cd linux-desk
   ```

2. Build the project using `cargo`:
   ```bash
   cargo build --release
   ```

3. Optionally, move the binary to a directory in your `PATH`:
   ```bash
   sudo mv target/release/linux-desk /usr/local/bin/desk
   ```

## Usage

### Basic Command
```bash
desk [OPTIONS] -p <path>
```

### Arguments
- `-p, --path <PATH>`: The path to the executable file for the application (required).

### Options
- `-n, --name <NAME>`: The name of the application (optional). If not provided, the name will be derived from the executable's filename.
- `-i, --icon <ICON>`: The path to the icon file for the application (optional).

### Examples

#### Create a `.desktop` File with a Custom Name and Icon
```bash
desk -p /usr/bin/firefox -n "Firefox Browser" -i /usr/share/icons/firefox.png
```

#### Create a `.desktop` File Without Specifying a Name or Icon
```bash
desk -p /usr/bin/vlc
```

#### Output
The `.desktop` file will be created in `~/.local/share/applications/` and will look like this:
```ini
[Desktop Entry]
Name=Firefox Browser
Exec=/usr/bin/firefox
Type=Application
Icon=/usr/share/icons/firefox.png
```

## How It Works

1. The tool generates a `.desktop` file based on the provided arguments.
2. If no name is provided, it extracts the name from the executable's filename.
3. The file is saved to `~/.local/share/applications/`.
4. The `update-desktop-database` command is run to refresh the desktop environment's application database.

## Requirements

- Rust (for building the project)
- `update-desktop-database` (part of the `desktop-file-utils` package)

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.