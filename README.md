# Home Inventory

This is a project started as all projects should: with a problem needing to be solved. For my family, this problem was our inability to easily keep up with all the food we had in the house. While mistakes weren't exactly commonplace, it was still fairly easy to misscount something in the pantry, or forget to check a spot in the freezer, or simply misremember how many of something we needed once we got to the store. The solution seemed so simple: run a database and build an interface for tracking the contents of our house. And thus, the Home Inventory.

## Usage

At this time, the only way to use this inventory is to build from source. This means you'll essentially need the entire Rust developement toolchain installed. That's not too difficult if you know what you're doing and have the time, but it's certainly not ideal. If you are able/willing to do that, below are some general instructions.

1. Create a directory for your finished app
2. From home-inventory/inventory-web/, execute `trunk build` (you will need [Trunk](https://trunkrs.dev/) installed)
3. Once trunk has built the web portion, copy the files from home-inventory/inventory-web/dist/ into a web/ directory for the finished app
4. From home-inventory/inventory-api/, execute `cargo build --release`
    - I have had issues on both my devices regarding `clang`. I do not believe there is a one-size-fits-all solution to this, unfortunately.
    - Compiling will take a hot minute. Rocket and SurrealDB are very large libraries, and the number of macros required to expose all the endpoints is a bit silly.
5. Once compiled, you only need the binary home-inventory/inventory-api/target/release/inventory-api. Copy this binary to the root directory of the finished app
6. At any point, you can copy home-inventory/inventory-api/inventory_config.toml to the root directory of the finished app
7. Configure the inventory using inventory_config.toml. Some of the defaults within the app or in the provided settings file will be sufficient, but some of them will require configuration.
8. The application can now be launched and should function as expected.
    - The provided home-inventory/inventory-api/shellstart.vbs can be used to leverage the Windows Task Scheduler for silently autostarting.
