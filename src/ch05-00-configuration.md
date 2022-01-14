# Configuration

Simple applications are able to accept all their options over the command line, but more complex ones eventually need to add support for configuration files.

## Configuration formats

**Configuration *should* be in the [TOML format](https://toml.io/en/).** The TOML format, as a descendant of INI, is widely understood and is easy to read and write for both humans and computers.

Other configuration formats like [YAML](https://yaml.org/) *may* be used if the configuration is extremely complex (though in these cases it's often fruitful to devise ways of reducing complexity), or if there are legacy constraints.

## Configuration scopes

Depending on the application, the following scopes for a configuration are often seen in practice:
1. *Directory-scoped.* Applies to a directory and its subdirectories. Controlled by a file somewhere in this directory or a parent. For example, [`.gitignore`](https://git-scm.com/docs/gitignore) is directory-scoped.
2. *Repository-scoped.* Applies to a repository: controlled by a file somewhere in a code repository. For example, [`clippy.toml`](https://github.com/rust-lang/rust-clippy#configuration) is repository-scoped.
3. *User-scoped.* A file somewhere in the user's home directory.
4. *System-wide.* A file somewhere in a central location on the computer.

Not all applications support all of these: which scopes make sense is a matter of judgment and thinking about use cases. Some server-side applications support fetching configuration from a remote server; they are out of scope here.

**If applications support repository-scoped configuration:**
* Applications *should* put repository-scoped configuration in a `.config` directory under the repository root. Typically, applications place their configuration at the top level of the repository. However, too many config files at the top level can pollute directory listings.
* Applications *should* allow both local and checked-in configuration files. For example, an application `myapp` should support configuration in both `.config/myapp.toml` and `.config/myapp.local.toml`. Entries in `./config/myapp.local.toml` *must* override those in `.config/myapp.toml`.

**If applications support user-specific configuration:**
* On Unix platforms other than macOS, applications *should* follow the [XDG specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).
* On macOS and Windows, applications *should* either use `$HOME/.config` or the native platform directory. On macOS and Windows, the platform-native directories are somewhat harder to access on the command line, so `$HOME/.config` is a suitable alternative.

[dirs](https://crates.io/crates/dirs) is the most actively maintained Rust library for getting the native config directory (and other directories) for every platform.

**Applications *may* read configuration options over the command line and the environment.** It is often reasonable to let users override configuration via command-line options and environment variables. If so, then:
* Environment variables *must* be prefixed with a unique identifier based on the app. For example, an app called `myapp` can support a "limit" configuration through a `MYAPP_LIMIT` variable.
* Environment variables *should* also be supported as command-line options. For example, `myapp --limit`. Command-line options are more discoverable than environment variables. If you actually *want* your options to be less discoverable, for example if exposing them would increase support load, you can add hidden command-line options.
* Command-line arguments *must* override environment variables. An environment variable can be set further up in the environment. A command-line argument expresses user intent most directly.

## Hierarchical configuration

**Applications *should* follow a hierarchical configuration structure.** Use the following order, from highest priority to lowest.
1. Command-line arguments
2. Environment variables
3. Directory or repository-scoped configuration
4. User-scoped configuration
5. System-wide configuration
6. Default configuration shipped with the program.

**Configurations *should* be merged rather than completely overwritten.** Consider the following configuration files.

```toml
# $HOME/.config/myapp.toml -- user-scoped config
limit = 42

[encoding]
input = "utf16"
output = "utf32"
```

```toml
# <repository>/.config/myapp.toml -- repository-scoped config
limit = 84

[encoding]
input = "utf8"
```

The *merged* configuration is:

```toml
[myapp]
limit = 84

[myapp.encoding]
input = "utf8"
output = "utf32"
```

Exactly what the boundaries of merged entries should be is application-specific: it is hard to give general guidance here.

## Rust libraries for managing configuration

There are two main Rust libraries for managing hierarchical configuration:

* [config](https://crates.io/crates/config). I've used this and it seems to work well.
* [figment](https://crates.io/crates/figment). This seems quite nice as well, though I haven't used it.
