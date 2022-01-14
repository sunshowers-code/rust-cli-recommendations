# Rain's Rust CLI recommendations

This living document covers [Rain's](https://github.com/sunshowers) [recommendations](https://rust-cli-recommendations.sunshowers.io)
for how to organize and manage Rust CLI applications.

In this document, I cover a set of tips and best practices for writing Rust applications, informed
[by experience](https://rust-cli-recommendations.sunshowers.io/why-listen.html). I hope you find them useful.

## Key words

The key words *must*, *must not*, *required*, *should*, *should not*,
*recommended*, *may*, and *optional*, derive their meanings from
[RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119), but have
somewhat different connotations because this is a list of recommendations
and not a standard.
* *must*, *must not* and *required* mean that an application that doesn't follow this recommendation is *incorrect* and has a bug that needs to be fixed.
* *should*, *should not* and *recommended* mean that most applications should follow this recommendation, but there are valid reasons not to.
* *may* and *optional* mean that programs are free to follow this recommendation or ignore it; there are valid reasons in either direction.

## Locations

This document is hosted online at [https://rust-cli-recommendations.sunshowers.io](https://rust-cli-recommendations.sunshowers.io). The source is hosted [on GitHub](https://github.com/sunshowers-code/rust-cli-recommendations).

This document is available offline by installing `git` and running the following command while online.

```sh
git clone https://github.com/sunshowers-code/rust-cli-recommendations/ --branch gh-pages
```

then pointing your web browser at `rust-cli-recommendations/index.html`.


Suggestions for changes and pull requests are welcome! However, please be OK with me not accepting your suggestions. If your opinions diverge from mine, please feel free to fork this project under the license terms listed below.

## License

This document, other than inline code snippets, is licensed under [CC BY 4.0]. This means that you are welcome to share, adapt or modify this material as long as you give appropriate credit.

Code snippets included in this document are licensed under [CC0 1.0]. The author(s) have waived all of their rights to the work worldwide under copyright law, to the extent allowed by law.

[CC BY 4.0]: https://creativecommons.org/licenses/by/4.0/
[CC0 1.0]: https://creativecommons.org/publicdomain/zero/1.0/
