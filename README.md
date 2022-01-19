# Rain's Rust CLI recommendations

This living document comprises [my](https://github.com/sunshowers) [recommendations](https://rust-cli-recommendations.sunshowers.io)
for how to organize and manage Rust CLI applications.

In this document, I cover some tips and best practices for writing Rust applications, informed
[by my experience](https://rust-cli-recommendations.sunshowers.io/why-trust.html) writing real-world Rust tools. I hope you find them useful.

If you haven't read the [Rust CLI Book](https://rust-cli.github.io/book/index.html) yet, go read it first! That book provides a lot of useful information about how command-line apps should be written in Rust. This document covers some more advanced material and is more opinionated overall.

## Locations

This document is hosted online at [https://rust-cli-recommendations.sunshowers.io](https://rust-cli-recommendations.sunshowers.io). The source is hosted [on GitHub](https://github.com/sunshowers-code/rust-cli-recommendations).

This document is available offline by installing `git` and running the following command while online.

```sh
git clone https://github.com/sunshowers-code/rust-cli-recommendations --branch gh-pages
```

then pointing your web browser at `rust-cli-recommendations/index.html`.


Pull requests to fix typos or unclear language are welcome! However, if you have a suggestion for a change to the document, please [search through the issues] to see if it's been discussed already. If not, please [open an issue].

[search through the issues]: https://github.com/sunshowers-code/rust-cli-recommendations/issues?q=is%3Aissue+sort%3Aupdated-desc
[open an issue]: https://github.com/sunshowers-code/rust-cli-recommendations/issues/new

## License

This document, other than inline code snippets, is licensed under [CC BY 4.0]. This means that you are welcome to share, adapt or modify this material as long as you give appropriate credit.

Code snippets included in this document are licensed under [CC0 1.0]. The author(s) have waived all of their rights to the work worldwide under copyright law, to the extent allowed by law.

[CC BY 4.0]: https://creativecommons.org/licenses/by/4.0/
[CC0 1.0]: https://creativecommons.org/publicdomain/zero/1.0/
