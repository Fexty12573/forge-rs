# Release

1. Bump **all** versions in [Cargo.toml](./Cargo.toml)
2. Commit Changes
3. Publish Crates (order matters)

```sh
cargo publish -p mhgu-forge-macros
cargo publish -p mhgu-forge-sys
cargo publish -p mhgu-forge-rt
cargo publish -p mhgu-forge
cargo publish -p cargo-mhgu-forge
```
