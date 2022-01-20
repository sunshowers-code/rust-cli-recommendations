# Binaries vs libraries

You *may* expose your application's functionality as a library. Some binaries are simple and don't necessarily need to expose their functionality as a library. Other binaries are more complex, in which case their functionality can be exposed as a library that others can build upon.

**Why separate libraries from binaries?**
* For other consumers of the library, clap and other binary-only dependencies are unnecessary.
* The binary's versioning is separated out from the library's versioning; see [Versioning](versioning.html) for more.

**Reasons against exposing a library**
* Maintaining a library in addition to a binary is hard work. It involves documentation and versioning.
* In some cases, maintainers can decide to expose their functionality *only* as a binary to force a looser coupling with downstream consumers.
  * *Case study:* The presence of the [libgit2](https://libgit2.org/) and [JGit](https://www.eclipse.org/jgit/) libraries for Git has made it significantly harder to improve Git's data structures. These libraries are tightly coupled to their consumers, which in practice means that Git improvements are tied to the release schedules of commercial projects like Xcode and Visual Studio.
  * Cargo and rustc are not designed to be invoked as libraries. They force loose coupling.

## If you've decided to make a library

> Note: In this section, "package" means all code scoped to a single `Cargo.toml` file.

If your code is meant to be uploaded to a registry like crates.io:
* Binary packages *must not* expose their library functionality within the same package.
* The library package *must* be separated out, with an appropriate name linking the two.

If your code is internal to the workspace:
* Binary packages *should not* expose a library within the same package.
* The library package *should* be separated out, with an appropriate name linking the two.

Some examples of linked names:
* *my-lib* for the library, and *my-lib-cli* for the binary, if most people are going to use the library.
* *my-app-core* for the library, and *my-app* for the binary, if most people are going to use the binary.
* *my-utility* for the library, and *cargo-my-utility* for the binary, if your program is a Cargo plugin.

There's an intermediate solution possible here, which is to have a single crate that enables being built as a binary with `--features=bin`. However, you *must not* do this for code uploaded to a registry, because you lose out on the benefits of having separate versioning. You *may* use this pattern for code internal to a workspace.
