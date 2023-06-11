# Hierarchical configuration

**Applications *should* follow a hierarchical configuration structure.** Use the following order, from highest priority to lowest.

1. Command-line arguments
2. Environment variables
3. Directory or repository-scoped configuration
4. User-scoped configuration
5. System-wide configuration
6. Default configuration shipped with the program.

> Tip: One neat trick is to embed your app's default configuration [as a config file](https://doc.rust-lang.org/std/macro.include_str.html) within your binary. The default configuration can serve as an example to other users.

There are some exceptions. For example, color support *should* follow the rules listed in the [Colors](./colors.md) section.

**Configurations *may* be merged rather than completely overwritten.** Consider the following configuration files.

```toml
# $HOME/.config/myapp.toml -- user-scoped config
limit = 42

[encoding]
input = "utf16"
output = "latin1"
```

```toml
# <repository>/.config/myapp.toml -- repository-scoped config
limit = 84

[encoding]
input = "utf8"
```

One way to merge configurations is to combine them, as follows:

```toml
limit = 84

[encoding]
input = "utf8"
output = "latin1"
```

Exactly how deep merges should go is application-specific.

## Rust libraries for managing configuration

There are two main Rust libraries for managing hierarchical configuration:

* [config](https://crates.io/crates/config). I've used this for my own projects.
* [figment](https://crates.io/crates/figment). Seems high quality, though I haven't used it.

These configuration libraries can be used in combination with serde, so that you can manage hierarchies and merges with dynamically typed variables at the edges of your program, then switch over to well-typed serde structures for validating the config's schema. For how to do this with config, [see this example].

[see this example]: https://github.com/mehcode/config-rs/blob/53e43fbcf96b5c2a661d052a6e3d55fc3709f1e1/examples/hierarchical-env/settings.rs
