# Binaries vs libraries

You *may* expose your application's functionality as a library. Some binaries are simple and don't necessarily need to expose their functionality as a library. Other binaries are more complex, in which case their functionality can be exposed as a library that others can build upon.

In any case, binary crates *must not* expose their library functionality within the same crate. The library, if exposed, *must* be a different crate, with an appropriate name linking the two. The library *should not* have any command-line parsing logic.

**Why separate libraries from binaries?**
* For other consumers of the library, clap and other binary-only dependencies are unnecessary.
* The binary's versioning is separated out from the library's versioning; see [Versioning](versioning.html) for more.

**Reasons against exposing a library**
* Maintaining a library in addition to a binary is hard work. It involves documentation and versioning.
* In some cases, maintainers can decide to expose their functionality *only* as a binary to force a looser coupling with downstream consumers.
  * *Case study:* The presence of the [libgit2](https://libgit2.org/) and [JGit](https://www.eclipse.org/jgit/) libraries for Git has made it significantly harder to improve Git's data structures. These libraries are tightly coupled to their consumers, which in practice means that Git improvements are tied to the release schedules of commercial projects like Xcode and Visual Studio.
  * Cargo and rustc are not designed to be invoked as libraries. They force loose coupling.
