# Rain's Rust CLI recommendations

This living document covers [Rain's](https://github.com/sunshowers) opinions about how to organize and manage Rust CLI applications.

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

This book is hosted online at [https://rust-cli-recommendations.sunshowers.io](https://rust-cli-recommendations.sunshowers.io). The sources are hosted [on GitHub](https://github.com/sunshowers-code/rust-cli-recommendations). The book is available offline by installing `git` and running the following command while online.

```sh
git clone https://github.com/sunshowers-code/rust-cli-recommendations/ --branch gh-pages
```

then pointing your web browser to `rust-cli-recommendations/index.html`.


Issues containing recommendations and PRs are welcome! However, I get to make the final call: if your opinions diverge from mine, you're welcome to fork this project.

## License

This work, other than inline code snippets, are licensed under [CC BY 4.0]. This means that you are welcome to share or adapt this material as long as you give appropriate credit.

Code snippets included in this book are licensed under [CC0 1.0]. The author(s) have waived all of their rights to the work worldwide under copyright law, to the extent allowed by law.

[CC BY 4.0]: https://creativecommons.org/licenses/by/4.0/
[CC0 1.0]: https://creativecommons.org/publicdomain/zero/1.0/
