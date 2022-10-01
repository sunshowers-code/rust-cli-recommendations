# Versioning

A library crate, if provided, *should* follow [the usual Rust library versioning rules](https://doc.rust-lang.org/cargo/reference/semver.html).

A binary crate *should* define its public API as consisting of the command-line interface, plus anything else related to the interface that the project's maintainers wish to keep stable.

* This means that major version changes happen when there are breaking changes to the CLI, not to internal or library code.
* For example, [cargo-hakari's stability policy](https://docs.rs/cargo-hakari/latest/cargo_hakari/#stability-guarantees) is to keep the contents of a generated checked-in file the same, unless a config option is turned on or there's a bugfix.

**Why?** It is easier to avoid making breaking changes to command-line interfaces. Mature projects like [GNU coreutils](https://www.gnu.org/software/coreutils/) avoid breaking changes to their CLIs for decades.

## Tips to avoid breaking changes

* Make experimental commands available via an environment variable or some other gating mechanism to gather feedback, with a warning that the behavior of these can change at any time.
* Mark old commands or arguments deprecated, and possibly hide them from help text. Continue to preserve their behavior.
* If the program persists data on disk, make it possible to do forward transitions but not backward ones. Add a *format version* to persisted data and increment it every time the data format changes. If an old version of the program reads a format version it does not understand, error out gracefully.

> Tip: If you're using GitHub Actions for CI, use the [baptiste0928/cargo-install](https://github.com/baptiste0928/cargo-install) action to install a binary from crates.io, using a cached version if possible. This action lets you specify a version range, which works well with the binary versioning policy above.
