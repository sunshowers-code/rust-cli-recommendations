# Binaries vs libraries

Binaries MAY expose their functionality as a library as well.

**Why?**
Some binaries are simple and don't necessarily need to expose their functionality as a library. Other binaries are more complex, in which case they may wish to expose their functionality as a library so that others may build upon it.

In any case, binary *crates* SHOULD NOT expose their library functionality within the same crate. The library, if exposed, SHOULD be a different crate, with an appropriate name linking the two. The library SHOULD NOT have any command-line parsing logic.

**Why?**
* For other consumers of the library, clap may be an unnecessary dependency.
* The binary's versioning is separated out from the library's versioning; see [Versioning](ch04-00-versioning.html) for more.
* Compile times become quicker.

**Why not?**
* Maintaining a library in addition to a binary is hard work.
* In some cases, maintainers may decide to expose their functionality *only* as a binary to force a looser coupling with downstream consumers.
  * For example, the presence of the [libgit2](https://libgit2.org/) and [JGit](https://www.eclipse.org/jgit/) libraries for Git has made it significantly harder to improve Git's data structures. These libraries are tightly coupled to their consumers, which in practice means that Git improvements are tied to the release schedules of commercial projects like Xcode and Visual Studio. Forced loose coupling can lead to greater development velocity.

## Providing an IPC interface

Programs MAY (and in the case of forced loose coupling, SHOULD) make their CLI available as an interface not just to humans but to other programs. In these cases, follow these rules:
* Programs SHOULD accept a `--message-format` flag, and SHOULD support at least `json` for programmatic consumption.
* Programs MAY also provide their output as other formats like XML, [CBOR](https://cbor.io/), [MessagePack](https://msgpack.org/index.html), or other **self-describing** formats.
  * A self-describing format is one where the keys, or some equivalent, are part of the serialized output.
  * Formats like [protobuf](https://developers.google.com/protocol-buffers) are suitable as well, if up-to-date IDLs (e.g. `.proto` files) are published along with releases. One neat trick is to embed them into your binary and add a command to write them out to a given directory.
* Programs MUST NOT provide their output as [bincode](https://github.com/bincode-org/bincode) or other non-self-describing formats. These formats are unsuitable for inter-process communication.
* For greater compatibility with existing Unix tools like `xargs`, programs MAY provide list output as newline-delimited items.
  * If list items that may themselves have newlines in them, programs MUST provide list output as null-delimited (`\0`-delimited) items. In particular, filenames may have newlines embedded in them but not null bytes. Almost all standard Unix commands understand NULL-delimited output (e.g. `xargs --null`).
* Color MUST be disabled while writing to a pipe, unless `--color=always` is passed in.
