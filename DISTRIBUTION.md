# Zere CLI Distribution Guide

This document explains how the Zere CLI is packaged and distributed to end users.

## Distribution Methods

### 1. GitHub Releases (Pre-built Binaries)

**What:** Pre-compiled binaries for all major platforms
**Who:** Users who want quick installation without Rust
**How it works:**

1. Tag a new release: `git tag cli-v0.1.0 && git push --tags`
2. GitHub Actions builds binaries for all platforms
3. Binaries uploaded to GitHub Releases page
4. Users download via browser or install script

**Platforms supported:**
- macOS (Intel & Apple Silicon)
- Linux (x86_64 & ARM64)
- Windows (x64)

**Build workflow:** `.github/workflows/release-cli.yml`

### 2. Install Scripts (Recommended for End Users)

**Bash script (macOS/Linux):** `install.sh`
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.sh | bash
```

**PowerShell script (Windows):** `install.ps1`
```powershell
irm https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.ps1 | iex
```

**What it does:**
- Detects OS and architecture
- Downloads correct binary from latest GitHub Release
- Installs to system PATH
- Verifies installation

### 3. Cargo (Rust Package Manager)

**What:** Source distribution via crates.io
**Who:** Rust developers, users who want latest features
**How to publish:**

```bash
# Login to crates.io
cargo login

# Publish from zere-cli directory
cd zere-cli
cargo publish
```

**Users install with:**
```bash
cargo install zere-cli
```

**Metadata:** Defined in `Cargo.toml`

### 4. Homebrew (macOS/Linux)

**What:** Package manager for macOS/Linux
**Who:** Users who prefer Homebrew
**How to set up:**

1. Create a tap repository: `homebrew-zere`
2. Add formula: `homebrew/zere-cli.rb`
3. Users install with:
   ```bash
   brew tap yourusername/zere
   brew install zere-cli
   ```

**Formula:** `homebrew/zere-cli.rb` (template provided)

### 5. Docker

**What:** Containerized CLI
**Who:** Users who prefer containers, CI/CD environments
**How it works:**

**Build locally:**
```bash
docker build -t zere-cli .
```

**Run from GitHub Container Registry:**
```bash
docker run --rm -it ghcr.io/yourusername/zere-cli:latest --help
```

**Published automatically:** GitHub Actions builds and pushes on release

### 6. NPM (Optional - for JS developers)

**What:** Binary wrapper via npm
**Who:** JavaScript/TypeScript developers
**How to set up:**

Create `package.json`:
```json
{
  "name": "@zeredata/cli",
  "version": "0.1.0",
  "bin": {
    "zere": "./bin/zere"
  },
  "scripts": {
    "postinstall": "node install.js"
  }
}
```

Create `install.js` to download correct binary.

Users install with:
```bash
npm install -g @zeredata/cli
```

## Release Process

### Step 1: Prepare Release

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Test locally: `cargo build --release`
4. Commit changes: `git commit -am "chore: bump version to 0.1.0"`

### Step 2: Create Git Tag

```bash
git tag cli-v0.1.0
git push origin main
git push origin cli-v0.1.0
```

### Step 3: Automated Build

GitHub Actions automatically:
- Builds binaries for all platforms
- Creates GitHub Release
- Uploads binaries as release assets
- Builds and pushes Docker image

### Step 4: Publish to Crates.io (Manual)

```bash
cd zere-cli
cargo publish
```

### Step 5: Update Homebrew Formula (Manual)

1. Download release binaries
2. Calculate SHA256 checksums: `shasum -a 256 zere-*`
3. Update `homebrew/zere-cli.rb` with new version and checksums
4. Commit to homebrew tap repository

### Step 6: Announce Release

- GitHub Release notes (auto-generated)
- Documentation site
- Social media / blog post

## Version Numbering

Follow Semantic Versioning (semver):
- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

**Tag format:** `cli-v{MAJOR}.{MINOR}.{PATCH}`
Example: `cli-v0.1.0`, `cli-v1.0.0`, `cli-v1.2.3`

## Binary Naming Convention

**Format:** `{name}-{os}-{arch}[.exe]`

**Examples:**
- `zere-darwin-arm64` (macOS Apple Silicon)
- `zere-darwin-amd64` (macOS Intel)
- `zere-linux-amd64` (Linux x86_64)
- `zere-linux-arm64` (Linux ARM64)
- `zere-windows-amd64.exe` (Windows x64)

## Installation Verification

All installation methods should result in:

```bash
$ zere --version
zere-cli 0.1.0

$ which zere
/usr/local/bin/zere  # or equivalent
```

## Troubleshooting Common Issues

### macOS Gatekeeper Warning

**Issue:** "zere cannot be opened because the developer cannot be verified"

**Solution:**
```bash
xattr -d com.apple.quarantine /usr/local/bin/zere
```

Or sign the binary with Apple Developer certificate in CI.

### Linux: Permission Denied

**Issue:** Binary not executable

**Solution:**
```bash
chmod +x /usr/local/bin/zere
```

### Windows: Not Recognized

**Issue:** 'zere' is not recognized as an internal or external command

**Solution:** Add installation directory to PATH:
1. Open "Environment Variables"
2. Edit "Path" user variable
3. Add `C:\Users\{username}\AppData\Local\Zere`
4. Restart terminal

## CI/CD Integration

For automated workflows, use Docker or install script:

**GitHub Actions:**
```yaml
- name: Install Zere CLI
  run: curl -sSL https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.sh | bash
```

**GitLab CI:**
```yaml
install_zere:
  script:
    - curl -sSL https://raw.githubusercontent.com/yourusername/zere-synth/main/zere-cli/install.sh | bash
```

**Docker:**
```yaml
- docker run --rm -v $(pwd):/workspace ghcr.io/yourusername/zere-cli:latest jobs list
```

## Future Distribution Channels

- **Chocolatey** (Windows package manager)
- **Scoop** (Windows package manager)
- **Snap** (Linux universal packages)
- **AUR** (Arch User Repository)
- **apt/yum repositories** (Debian/RedHat packages)

## Security Considerations

1. **Code Signing**
   - Sign macOS binaries with Apple Developer certificate
   - Sign Windows binaries with code signing certificate
   - Improves trust, reduces security warnings

2. **Checksums**
   - Publish SHA256 checksums with releases
   - Install scripts should verify checksums

3. **Supply Chain Security**
   - Use GitHub's signed commits
   - Enable dependency scanning
   - Regular security audits with `cargo audit`

## Support

For installation issues:
- GitHub Issues: https://github.com/yourusername/zere-synth/issues
- Documentation: https://github.com/yourusername/zere-synth/tree/main/zere-cli

## License

MIT License - See LICENSE file for details
