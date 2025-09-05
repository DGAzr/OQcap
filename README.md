# OQcap - Obsidian Quick Capture

A minimal, native Wayland application for quickly capturing text and sending it to Obsidian. Built with Rust and GTK4 for seamless integration with modern Linux desktop environments.

## Features

- **Native Wayland support** with GTK4 and libadwaita
- **Minimal floating window** that matches your system theme
- **Configurable Obsidian integration** with support for:
  - Custom vaults
  - Folders and templates
  - Plugin integration
  - Custom URL parameters
- **Keyboard shortcuts**: Ctrl+Enter to submit, Escape to close, Ctrl+, for settings
- **Auto-close** after sending text to Obsidian

## Prerequisites

### System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt install build-essential libgtk-4-dev libadwaita-1-dev
```

**Fedora:**
```bash
sudo dnf install gcc gtk4-devel libadwaita-devel
```

**Arch Linux:**
```bash
sudo pacman -S base-devel gtk4 libadwaita
```

### Rust
Install Rust from [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Build Instructions

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd OQcap
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   cargo run --release
   ```

## Installation

### Manual Installation

1. **Build the binary:**
   ```bash
   cargo build --release
   ```

2. **Copy the binary to your PATH:**
   ```bash
   sudo cp target/release/oqcap /usr/local/bin/
   ```

3. **Install the desktop file:**
   ```bash
   sudo cp oqcap.desktop /usr/share/applications/
   ```

4. **Update desktop database:**
   ```bash
   sudo update-desktop-database
   ```

## Configuration

OQcap uses a TOML configuration file located at `~/.config/oqcap/config.toml`. The configuration file is automatically created on first run with default settings.

### Configuration Options

```toml
# Obsidian vault name (optional)
vault = "MyVault"

# Obsidian action (new, open, search, quickadd, etc.)
action = "new"

# Parameter name for user content (optional)
# - For "new" action: defaults to "content"
# - For other actions: no content parameter sent unless specified
content_param = "content"

# Additional URL parameters
[parameters]
folder = "Quick Notes"     # Target folder for new notes
template = "Daily"         # Template to use

# Plugin integration (optional)
[plugin]
command = "templater"      # Plugin command

[plugin.params]
template = "quick-note"    # Plugin-specific parameters
```

### Example Configurations

**Basic setup (default):**
```toml
action = "new"
```
Generates: `obsidian://new?content=<text>`

**Specific vault and folder:**
```toml
vault = "Work Notes"
action = "new"

[parameters]
folder = "Quick Capture"
```
Generates: `obsidian://new?vault=Work%20Notes&content=<text>&folder=Quick%20Capture`

**QuickAdd Plugin integration:**
```toml
vault = "Personal"
action = "quickadd"
content_param = "value"

[parameters]
choice = "quick-note"
```
Generates: `obsidian://quickadd?vault=Personal&value=<text>&choice=quick-note`

**Search in vault:**
```toml
vault = "Research"
action = "search"
content_param = "query"
```
Generates: `obsidian://search?vault=Research&query=<text>`

## Usage

1. **Launch the application:**
   - From terminal: `oqcap`
   - From application launcher: Search for "Quick Capture"
   - From desktop file: Click the OQcap icon

2. **Capture text:**
   - Type or paste your text in the text area
   - Press `Ctrl+Enter` or click "Send to Obsidian"
   - The window will close automatically after sending

3. **Keyboard shortcuts:**
   - `Ctrl+Enter`: Send text to Obsidian
   - `Escape`: Close window

## Configuration Management

OQcap automatically creates a configuration file at `~/.config/oqcap/config.toml` on first run. An example configuration file is also created at `~/.config/oqcap/config.example.toml` for reference.

### Edit configuration:
```bash
# Edit the main config file
nano ~/.config/oqcap/config.toml

# Or view the example configuration for reference
cat ~/.config/oqcap/config.example.toml
```

### View current configuration:
```bash
cat ~/.config/oqcap/config.toml
```

### Reset to defaults:
```bash
rm ~/.config/oqcap/config.toml
```

The configuration will be recreated with defaults on next run.

## Obsidian Setup

Ensure Obsidian is installed and configured to handle `obsidian://` protocol URLs. Most Obsidian installations handle this automatically, but you may need to:

1. Start Obsidian at least once
2. Verify that clicking `obsidian://` links in your browser opens Obsidian
3. Test with a simple URL like: `obsidian://new?content=test`

## Troubleshooting

### Application won't start
- Ensure GTK4 and libadwaita are installed
- Check that you're running on a Wayland or X11 desktop environment

### Obsidian integration not working
- Verify Obsidian is installed and has been run at least once
- Test manual Obsidian URLs in your browser
- Check terminal output for error messages when running `oqcap`

### Configuration issues
- Check file permissions on `~/.config/oqcap/config.toml`
- Verify TOML syntax in your configuration file
- Remove config file to reset to defaults

## Development

### Running in development mode:
```bash
cargo run
```

### Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

### Running tests:
```bash
cargo test
```

## License

This project is released as open source under the GPLv3. Please see the LICENSE file for details.
