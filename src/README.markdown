
### Installing Rust
Note: The application can only clear the screen for vt100 terminal emulators, so you must be using one of those (OSX's builtin Terminal and iTerm2 both work).

##### OSX

1. brew install rust

##### Linux

    $ curl -O http://static.rust-lang.org/dist/rust-0.8.tar.gz
    $ tar -xzf rust-0.8.tar.gz
    $ cd rust-0.8
    $ ./configure
    $ make
    $ sudo make install

##### Windows

Install a linux partition, follow linux steps.  ([Or follow this...](https://github.com/mozilla/rust/wiki/Note-getting-started-developing-Rust#windows))

### Run the tests

    rust test src/all-tests.rs

### Play the game

    rust run src/main.rs

