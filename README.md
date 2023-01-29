# cardego-server

**Current version: alpha-10**

This is the game-data server for Cardego, the homebrew RPG system involving cards.

## Building the server

You will need to build SQLite3, and use Rust and `cargo` to build the
project. Go to SQLite3's website, and build for x64. For Windows, make
sure to also download Visual Studio 2019 and build it using `x64 Native
Tools Command Prompt for VS 2019`.

~~In addition, you will need the development libraries for Cairo in
order to render cards. Please refer to
https://github.com/wingtk/gvsbuild to build Cairo on Windows.~~
**Update: I have dropped Cairo in favor of a process call to
`wkhtmltoimage` to generate card images.**

## Running the server

You should execute `cardego-server` with the working directory set to
the root of the repository.

- `static` is where compile-time and never-changing application
  resources should be.
- `config` is specifically where configuration files go.
- `runtime` is where run-time and dynamic resources go. However, do not
  assume that you can just wipe the directory -- some iterations of
  Cardego use a local database file that can be under here. Deleting
  this directory will destroy your local database!

Look at `schema.rs`. The schema for the SQL tables required for the
server to run properly are detailed in there.

The SQLite3 database file `cards.db` should be placed within
`runtime/data`.

### wkhtmltoimage

Get the executable/DLL/so from the website, and then also place it in
the working directory.

## Editing the server

One application that I've found helpful is to use SQLiteStudio to
perform a lot of the database editing. Given that the server itself, as
of the time of writing, does not have full CRUD capabilities, doing
content changes at the database level is preferred.

## Roadmap: post alpha-9

I hope to eventually create another app called "Cardego Studio" that can
be used to interface with the back-end application to perform CRUD
informations on the database with a new UI. Currently, I just
implemented a new search query language since there wasn't an easy one
in a Rust crate that worked well for me, so I rolled my own.

Authentication and account management is still something that is not
even ready to be thought about. I think it should be rolled into another
database for single-sign-on delegation.
