# Binaries vs libraries

Binaries MAY expose their functionality as a library as well.

**Why?**
Some binaries are simple and don't necessarily need to expose their functionality as a library. Other binaries are more complex, in which case they may wish to expose their functionality as a library so that others may build upon it.

In any case, binary *crates* SHOULD NOT expose their library functionality within the same crate. The library, if exposed, SHOULD be a different crate, with an appropriate name linking the two. The library SHOULD NOT have any command-line parsing logic.

**Why?**
* For other consumers of the library, clap may be an unnecessary dependency.
* The binary's versioning is separated out from the library's versioning; see [Versioning](ch03-01-versioning.html) for more.
* Compile times become quicker.

**Downsides**
* Maintaining a library in addition to a binary is hard work.
* In some cases, maintainers may want to expose their functionality *only* as a binary, and to force a looser coupling with downstream consumers. As a counterexample, the presence of the [libgit2](https://libgit2.org/) and [JGit](https://www.eclipse.org/jgit/) libraries to Git has made it significantly harder to make improvements to Git's data structures. Those libraries are tightly coupled to their consumers, which in practice means that Git improvements are tied to the release schedules of projects like Xcode and Visual Studio. Forced loose coupling can lead to greater development velocity.
