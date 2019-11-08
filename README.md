![Tweleter - more flames means more lit banner](https://raw.githubusercontent.com/literalplus/tweleter/master/assets/console-banner.png)

This application might in the future be able to delete tweets
based on a Twitter archive. For now, it just does random stuff
to help me get started with Rust and Qt.

Building
--------

To build this application, you first need to check it out:

````bash
git clone https://github.com/literalplus/tweleter.git
````

To compile it, run this: (you need to 
[set up Rust](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
for this)

````bash
cargo build
````

This places the binaries in `target/debug/tweleter`
(`tweleter.exe` on Windows).

You can also compile and instantly run the project:

````bash
cargo run
````

In the future, I may provide pre-compiled binaries.

Contributing
------------

Any and all contributions (issues, pull requests, wiki edits, ...)
are welcome! Just try to stick to the style of the existing code
and the generally accepted standards for Rust. I'm always open
for suggestions.

This project uses [Glade](https://glade.gnome.org/) for GTK+ UI development.

License
-------

This project is licensed under a MIT License.
Check out the `LICENSE.txt` file for more information.
