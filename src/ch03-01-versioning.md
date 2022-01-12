# Versioning

A library crate, if provide, MUST follow [the usual Rust library versioning rules](https://doc.rust-lang.org/cargo/reference/semver.html).

A binary crate MUST NOT follow those rules. Instead, the public API MUST consist of the command-line interface, plus anything else related to the interface that the project's maintainers wish to keep stable.
* For example, [cargo-hakari's stability policy](https://docs.rs/cargo-hakari/latest/cargo_hakari/#stability-guarantees) is to keep the contents of a generated checked-in file the same, unless a new config option is added or a bugfix is involved.

**Why?**
* It is easier to make append-only changes to command-line interfaces, so breaking changes are rarer. Mature projects may avoid breaking changes to the CLI for decades.
