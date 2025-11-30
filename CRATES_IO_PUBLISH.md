# Publishing to crates.io

Quick guide to publish `zere-cli` to the Rust package registry.

## Prerequisites

- [ ] Cargo.toml has all metadata filled
- [ ] README.md is complete
- [ ] LICENSE file exists (MIT)
- [ ] Code builds successfully: `cargo build --release`
- [ ] Tests pass: `cargo test`
- [ ] Public repo is synced

## Step 1: Create crates.io Account

1. Go to https://crates.io
2. Click "Log in with GitHub"
3. Authorize crates.io

## Step 2: Get API Token

1. Go to https://crates.io/settings/tokens
2. Click "New Token"
3. Name: `zere-cli-publish`
4. Scope: `publish-update` (or `publish-new` for first time)
5. Click "Create"
6. Copy the token

## Step 3: Login with Cargo

```bash
cargo login <YOUR_TOKEN_HERE>
```

This stores the token in `~/.cargo/credentials`

## Step 4: Verify Package

Before publishing, do a dry run:

```bash
cd /Users/umitkavala/Documents/code/zere-synth/zere-cli

# Check for issues
cargo publish --dry-run

# Review what will be published
cargo package --list
```

Common issues:
- Missing README.md → Copy from repo root
- Missing LICENSE → Add MIT license file
- Large file size → Check for included binaries/test data

## Step 5: Publish

```bash
cargo publish
```

This will:
1. Build the package
2. Upload to crates.io
3. Verify it compiles on their servers

**Important:** Once published, you cannot delete or modify a version. You can only yank it (hide from search).

## Step 6: Verify

1. Visit https://crates.io/crates/zere-cli
2. Check that it shows version 0.1.0
3. Test installation:
   ```bash
   cargo install zere-cli
   zere --version
   ```

## Updating to New Versions

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.1.1"
   ```

2. Update CHANGELOG.md with changes

3. Commit the version bump:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.1.1"
   git push
   ```

4. Publish:
   ```bash
   cargo publish
   ```

## Version Numbering (Semantic Versioning)

- **0.1.0 → 0.1.1**: Bug fixes (PATCH)
- **0.1.1 → 0.2.0**: New features, backward compatible (MINOR)
- **0.2.0 → 1.0.0**: Breaking changes (MAJOR)

Before 1.0.0, any version can have breaking changes.

## Automation (Optional)

Add to `.github/workflows/publish-crates.yml`:

```yaml
name: Publish to crates.io

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

Then add `CARGO_REGISTRY_TOKEN` secret to GitHub repo settings.

## Troubleshooting

### "package name is already taken"

If `zere-cli` is taken, try:
- `zeredata-cli`
- `zere-data-cli`
- `zere-synth-cli`

Update `Cargo.toml`:
```toml
name = "zeredata-cli"  # Package name on crates.io

[[bin]]
name = "zere"  # Binary name stays the same
```

### "the remote server responded with an error: crate name is reserved"

Some names are reserved. Choose a different name.

### "failed to verify package tarball"

Run `cargo publish --dry-run --verbose` to see what's failing.

Common causes:
- Missing files referenced in Cargo.toml
- Path dependencies instead of crates.io dependencies

### "binary already exists in publish queue"

Wait 5-10 minutes and try again. crates.io processes uploads sequentially.

## Package Metadata Checklist

Ensure these are set in `Cargo.toml`:

- [x] name
- [x] version
- [x] authors
- [x] edition
- [x] description (max 200 chars)
- [x] license
- [x] homepage
- [x] repository
- [x] readme
- [x] keywords (max 5)
- [x] categories (max 5)

## Best Practices

1. **README**: Should have installation instructions and examples
2. **Examples**: Include `examples/` directory with sample code
3. **Documentation**: Use `cargo doc` to generate docs
4. **CI/CD**: Test on multiple platforms before publishing
5. **Changelog**: Keep CHANGELOG.md updated
6. **Versioning**: Follow semver strictly

## After Publishing

1. Add badge to README:
   ```markdown
   [![Crates.io](https://img.shields.io/crates/v/zere-cli.svg)](https://crates.io/crates/zere-cli)
   ```

2. Update installation instructions:
   ```bash
   cargo install zere-cli
   ```

3. Announce on:
   - Twitter/X
   - Reddit (r/rust)
   - This Week in Rust
   - Company blog/website

## Yanking a Version (Emergency)

If you published a broken version:

```bash
cargo yank --vers 0.1.0
```

This hides it from searches but doesn't delete it (existing users can still use it).

To undo:
```bash
cargo yank --vers 0.1.0 --undo
```

## Support

- crates.io help: https://doc.rust-lang.org/cargo/reference/publishing.html
- Package search: https://crates.io/search?q=zere
- Your package: https://crates.io/crates/zere-cli (after publishing)
