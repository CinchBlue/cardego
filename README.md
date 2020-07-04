# cardego-server

**Current version: alpha-2**

This is the server for Cardego, the homebrew RPG system involving cards.

Cardego is designed to be played with Tabletop Simulator, but its coded
components can be broken up into the front-end and back-end. This repo
contains the back-end, and could potentially be used with a different
front-end besides Tabletop Simulator.

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

## Editing the server

One application that I've found helpful is to use SQLiteStudio to
perform a lot of the database editing. Given that the server itself, as
of the time of writing, does not have full CRUD capabilities, doing
content changes at the database level is preferred.


