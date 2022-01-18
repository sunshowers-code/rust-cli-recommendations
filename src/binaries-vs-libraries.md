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

## Machine-readable output

Applications *may* (and in the case of forced loose coupling, *should*) make their CLI available as an interface not just to humans but to other programs. If you're making your interface available this way, follow these rules:
* For lists of simple items like strings, programs *should* provide list output as newline-delimited items.
  * This is most useful for compatibility with existing tools like `xargs`.
  * If list items are filenames or can have newlines or other in them, programs *must* provide a `-0` flag or similar to list output as null-delimited (`\0`-delimited) items. Almost all standard Unix commands understand null-delimited output (e.g. `xargs --null`).
* For more complex structured data, programs *should* accept a flag to provide output (e.g. `--output-format`, or `--message-format` if multiple messages are printed out), and *should* support at least `json` machine-readable output.
* Programs *may* also provide their output as other formats like XML, [CBOR](https://cbor.io/), [MessagePack](https://msgpack.org/index.html), or other **self-describing** formats.
  * A self-describing format is one where the keys, or some equivalent, are part of the serialized output.
  * Formats like [protobuf](https://developers.google.com/protocol-buffers) are suitable as well, if up-to-date IDLs (e.g. `.proto` files) are published along with releases. One neat trick is to embed them into your binary and add a command to write them out to a given directory.
* All machine-readable output *must* be printed to stdout, *not* stderr.
* Programs *must not* provide their output as [bincode](https://github.com/bincode-org/bincode) or other non-self-describing formats. These formats are unsuitable for inter-process communication where stability is paramount.
* Colors *must* be disabled while writing to a pipe, unless `--color=always` is passed in. See [Colors](./colors.html) for more.
* Within a version series, output *must* be kept stable and append-only, and any breaking changes *must* be gated to an argument (e.g. `--format-version 2` or `--message-format json-v2`).
