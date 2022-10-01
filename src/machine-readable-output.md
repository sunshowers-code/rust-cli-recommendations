# Machine-readable output

Applications *may* (and in the case of forced loose coupling, *should*) make their CLI available as an interface not just to humans but to other programs. If you're making your interface available this way, follow these rules:

**For lists of strings, programs *should* provide list output as newline-delimited items.**

* This is most useful for compatibility with existing tools like `xargs`.
* If list items are filenames or can have newlines or other in them, programs *must* provide a `-0` flag or similar to list output as null-delimited (`\0`-delimited) items. Almost all standard Unix commands understand null-delimited output (e.g. `xargs --null`).

**For more complex structured data, programs *should* accept a flag to provide output (e.g. `--output-format`, or `--message-format` if many lines of structured data are printed out).**

* Programs *should* support at least `json` machine-readable output.
* Programs *may* also provide their output as XML, [CBOR](https://cbor.io/), [MessagePack](https://msgpack.org/index.html), or other **self-describing** formats.
  * A self-describing format is one where the keys, or some equivalent, are part of the serialized output.
* Formats like [protobuf](https://developers.google.com/protocol-buffers) are suitable as well, if up-to-date IDLs (e.g. `.proto` files) are published along with releases. One neat trick is to embed them into your binary and add a command to write them out to a given directory.
* If many lines of structured data are incrementally printed out, prefer a format like [newline-delimited JSON](http://ndjson.org/). This is the format used by Cargo's `--message-format json` option.

**Programs *must not* provide their output as [bincode](https://github.com/bincode-org/bincode) or other non-self-describing formats.** These formats are unsuitable for interoperability, where stability is paramount.

**All machine-readable output *must* be printed to stdout, *not* stderr.**

**Colors *must* be disabled for machine-readable output.**

**Within a binary version series, output *must* be kept stable and append-only.** Breaking changes *must* be gated to an argument (e.g. `--format-version 2` or `--message-format json-v2`). Adding new keys to a JSON map or equivalent is generally considered stable.
