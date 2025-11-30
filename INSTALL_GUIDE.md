# Zere CLI Installation Guide for Users

This guide explains how to install the Zere CLI tool on your system.

## Quick Start

Choose your preferred installation method below:

### 🚀 Recommended: Quick Install Script

**macOS or Linux:**
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
irm https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.ps1 | iex
```

This automatically:
- Detects your operating system and CPU architecture
- Downloads the correct binary
- Installs to your system PATH
- Verifies the installation

## Alternative Methods

### Method 1: Homebrew (macOS/Linux)

```bash
brew tap yourusername/zere
brew install zere-cli
```

### Method 2: Cargo (Rust Package Manager)

If you have Rust installed:
```bash
cargo install zere-cli
```

### Method 3: Download Binary Manually

1. Go to [Releases](https://github.com/yourusername/zere-synth/releases)
2. Download the binary for your platform:
   - **macOS (M1/M2/M3)**: `zere-darwin-arm64`
   - **macOS (Intel)**: `zere-darwin-amd64`
   - **Linux (64-bit)**: `zere-linux-amd64`
   - **Windows (64-bit)**: `zere-windows-amd64.exe`

3. **macOS/Linux:** Make it executable and move to PATH
   ```bash
   chmod +x zere-darwin-arm64
   sudo mv zere-darwin-arm64 /usr/local/bin/zere
   ```

4. **Windows:** Move to `C:\Program Files\Zere\` and add to PATH

### Method 4: Docker

No installation needed - run directly:
```bash
docker run --rm -it ghcr.io/yourusername/zere-cli:latest --help
```

For convenience, create an alias:
```bash
alias zere='docker run --rm -it -v ~/.config/zere:/root/.config/zere ghcr.io/yourusername/zere-cli:latest'
```

## Verify Installation

After installation, verify it works:

```bash
zere --version
```

You should see: `zere-cli 0.1.0` (or latest version)

## First Steps

1. **Login to your account:**
   ```bash
   zere login
   ```

2. **Launch interactive TUI:**
   ```bash
   zere --interactive
   ```

3. **View help:**
   ```bash
   zere --help
   ```

## Troubleshooting

### macOS: "Cannot be opened because the developer cannot be verified"

Run this command:
```bash
xattr -d com.apple.quarantine /usr/local/bin/zere
```

### Linux: "Permission denied"

Make the binary executable:
```bash
chmod +x /usr/local/bin/zere
```

### Windows: "Not recognized as a command"

Add the installation directory to your PATH:
1. Search for "Environment Variables" in Start Menu
2. Click "Environment Variables"
3. Under "User variables", select "Path" and click "Edit"
4. Click "New" and add `C:\Users\YourUsername\AppData\Local\Zere`
5. Click "OK" and restart your terminal

### Command not found after installation

Restart your terminal or run:
```bash
source ~/.bashrc    # or ~/.zshrc for zsh
```

## Updating

### Quick Install Script
Re-run the install script - it will overwrite the old version:
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.sh | bash
```

### Homebrew
```bash
brew upgrade zere-cli
```

### Cargo
```bash
cargo install zere-cli --force
```

## Uninstalling

### Quick Install / Manual
```bash
sudo rm /usr/local/bin/zere
rm -rf ~/.config/zere
```

### Homebrew
```bash
brew uninstall zere-cli
rm -rf ~/.config/zere
```

### Cargo
```bash
cargo uninstall zere-cli
rm -rf ~/.config/zere
```

### Windows
1. Delete `C:\Users\YourUsername\AppData\Local\Zere\`
2. Remove from PATH (reverse installation steps)
3. Delete `C:\Users\YourUsername\.config\zere\`

## Getting Help

- **Documentation**: [GitHub README](https://github.com/yourusername/zere-synth/tree/main/zere-cli)
- **Issues**: [GitHub Issues](https://github.com/yourusername/zere-synth/issues)
- **In-app help**: `zere --help` or press `?` in TUI mode

## System Requirements

- **Operating System**: macOS 10.15+, Linux (glibc 2.31+), Windows 10+
- **Architecture**: x86_64 (64-bit Intel/AMD) or ARM64 (Apple Silicon, ARM servers)
- **Disk Space**: ~20 MB
- **Network**: Required for API calls

## What's Next?

After installation, check out:
- [Usage Guide](README.md#usage)
- [TUI Mode Guide](README.md#interactive-tui-mode)
- [Examples](README.md#examples)

Happy data generation! 🚀
