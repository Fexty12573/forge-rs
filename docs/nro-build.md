# NRO build and linking

## Build with this checkout

Use Rust nightly with the `rust-src` component and devkitARM's
`arm-none-eabi-gcc`. Build the CLI from this repository so it embeds the updated
target template; a previously installed or published CLI may use the old one.

Run these commands in Bash from the repository root:

```bash
cargo +nightly build --locked --release -p cargo-mhgu-forge
export PATH="$PWD/target/release:$PATH"
cargo +nightly forge new my-amazing-plugin
cd "my-amazing-plugin"
cargo forge build --release
```

Make `arm-none-eabi-gcc` available on `PATH`, or set
`CARGO_TARGET_ARMV7A_NONE_EABI_LINKER` to its absolute path before building.
For a Windows Rust toolchain running from Bash, use a Windows-compatible path:

```bash
export CARGO_TARGET_ARMV7A_NONE_EABI_LINKER="C:/devkitPro/devkitARM/bin/arm-none-eabi-gcc.exe"
```

The generated `rust-toolchain.toml` selects nightly and `rust-src`. The project's
`.cargo/config.toml` selects its local `armv7a-none-eabi.json` and builds `core`
and `alloc`. Its `build.rs` supplies `link/switch32.ld` and the build ID option.

`cargo forge build --release` first links an ARM32 ELF, then converts it into
`my_amazing_plugin.nro` in the project root. No external `elf2nro` command is
needed. Install the NRO in `romfs/nativeNX/plugins` under the game's mod directory.
The same NRO build configuration can be used for Switch hardware and an emulator
with the compatible Forge loader installed.

## Target JSON settings

Keep the existing `"relocation-model": "pic"` and add:

```json
"pre-link-args": {
    "gcc": [
        "-nostartfiles",
        "-Wl,-z,text"
    ]
}
```

`pre-link-args` tells Rust which arguments to pass before the normal linker
inputs. The `gcc` key selects the GCC linker-driver flavor; this target uses
`arm-none-eabi-gcc`, not the host PC's GCC.

| Setting | Purpose |
| --- | --- |
| `"relocation-model": "pic"` | Generate position-independent ARM code so the plugin can be loaded at different addresses. This setting was already present. |
| `-nostartfiles` | Exclude GCC's default executable startup objects. Forge and the plugin runtime provide the module entry and initialization. |
| `-Wl,-z,text` | Pass `-z text` through GCC to the linker and reject dynamic relocations in read-only segments. It detects a bad link; it does not repair an existing NRO. |

MHGU's SDK applies dynamic relocations when loading an NRO. A relocation that
writes into its RX text or read-only data can fault on Switch hardware, even
when an emulator allowed the same file to run. The tested build excludes the
startup objects that introduced those relocations and rejects any remaining
text relocations during linking. The loader still needs to support the emitted
ARM relocation types, and relocation destinations must be writable data/BSS.
Other build configurations can also satisfy these requirements.

## Update an existing plugin

`cargo forge new` copies the embedded template into the new project. Updating
the CLI alone does not modify an existing project's target JSON.

1. Check the `[build].target` path in the plugin's `.cargo/config.toml`.
2. Add the two flags above to that JSON, keeping PIC. If `pre-link-args.gcc`
   already exists, extend its array instead of adding a duplicate JSON key.
3. Run `cargo forge build --release` again and deploy the newly generated NRO.

Plugin gameplay code does not need to change just to apply these linker flags.
SDK bindings must separately match the loader's ABI: this checkout already has
the Forge 1.5 shared-JIT `Hook` layout (`context` and `ctx_cell`). Keep that
upstream fix when porting build changes from an older checkout. This linker
change does not bump the SDK version.
