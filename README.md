# cardego-server

This is the server for Cardego, the homebrew RPG system involving cards.

## Running the server

You should execute `cardego-server` with the working directory set to
the root of the repository.

Look at `schema.rs`. The schema for the SQL tables required for the
server to run properly are detailed in there.

The SQLite3 database file `cards.db` should be placed within
`runtime/data`. 

## Building the server

You will need to build SQLite3, and use Rust and `cargo` to build the
project.


