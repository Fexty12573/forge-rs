# mhgu-forge-rt

NRO runtime startup for [`forge`](https://github.com/Fexty12573/forge) plugins:
the MOD0 header, `init`/`fini` array handling, and the other crt0-level glue an
NRO needs to load into Monster Hunter Generations Ultimate.

You normally don't depend on it directly - [`mhgu-forge`](https://crates.io/crates/mhgu-forge)
pulls it in for you.

## Features

- `logging` - enable runtime logging support (depends on [`mhgu-forge-sys`](https://crates.io/crates/mhgu-forge-sys)).

Part of the [forge-rs](https://github.com/Fexty12573/forge-rs) workspace.
