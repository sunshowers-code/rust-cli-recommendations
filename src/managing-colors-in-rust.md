# Managing colors in Rust

There are many Rust libraries for managing colors: my favorite is [owo-colors](https://crates.io/crates/owo-colors) because it is the only library I've found that meets all of these criteria:
* actively maintained
* has a simple, intuitive API
* minimizes dependencies on global state
* involves zero allocations

> Note: you *should not* use [termcolor](https://docs.rs/termcolor/latest/termcolor/) because it targets the deprecated Console APIs on Windows—and has a significantly more involved API as a result.
>
> Instead, use a library that just only supports ANSI color codes, and initialize support for them on Windows with [enable-ansi-support](https://crates.io/crates/enable-ansi-support).

There are two general patterns with which color support can be handled: the "immediate pattern" and the "stylesheet pattern". **Library code that supports colors *should* use the stylesheet pattern to manage the colors that are displayed.** Code in a binary crate can use whichever pattern leads to simpler code.

## The immediate pattern

This pattern is usually presented in examples and tutorials. It is conceptually quite simple.

Here's an example of what it looks like:

```rust,noplaypen
{{#rustdoc_include ../code/managing-colors/src/bin/immediate.rs:immediate}}
```

Notes:
* **`owo_colors::set_override` is used to control color support globally.** The global configuration only has an effect if `if_supports_color` is called.
* **`println!` is paired with `Stream::Stdout`.** If this were `eprintln!`, it would need to be paired with `Stream::Stderr`.

While this pattern is sometimes convenient in binary code, **it *should not* be used in libraries.** That is because libraries *should not* print information directly out to stdout or stderr—instead, they should return values that implement `Display` or similar. Library code *should* use the stylesheet pattern instead.

## The stylesheet pattern

This pattern involves defining a `Styles` struct containing colors and styles to apply to a text.

A stylesheet is simply a list of dynamic styles, customized to a particular type to be displayed. Here's an example:

```rust,noplaypen
{{#rustdoc_include ../code/managing-colors/src/lib.rs:stylesheet}}
```

Here's some library code that uses the above stylesheet:

```rust,noplaypen
{{#rustdoc_include ../code/managing-colors/src/lib.rs:my_value}}
```

And finally, here's the binary code that uses the library.

```rust,noplaypen
{{#rustdoc_include ../code/managing-colors/src/bin/stylesheet.rs:stylesheet}}
```

Notes:
* **Library code is completely unaware of whether the environment supports colors.** All it cares about is whether the `colorize` method is called.
  * Note that the global `set_override` and `unset_override` methods have no impact on library code in the example above, since it is only active if `if_supports_color` is called as in the immediate pattern above.
* **The stylesheet is stored as `Box<Styles>`.** The boxing isn't strictly required, but each `Style` is pretty large, and a struct containing e.g. 16 styles is 272 bytes as of owo-colors 3.2.0. That's a pretty large amount of data to store on the stack.
* **`Styles::default()` initializes all the styles to having no effect.** The `colorize()` method then initializes them as required.
* **For custom color support, `Styles` can be made public.** Most library code won't need to give users the ability to customize styles, but this pattern naturally extends to that use case.
* **Use of a separate `MyAppDisplay` type.** The `colorize` call is isolated to this particular `MyAppDisplay`, without influencing other display calls.
* **`println!` is paired with `Stream::Stdout`.** If this were `eprintln!`, it would need to be paired with `Stream::Stderr`.
