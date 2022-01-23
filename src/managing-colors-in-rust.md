# Managing colors in Rust

There are many Rust libraries for managing terminal colors. You *should* use [owo-colors](https://crates.io/crates/owo-colors) because it is the only library I've found that meets all of these criteria:
* actively maintained
* has a simple, intuitive API
* minimizes dependencies on global state
* involves zero allocations

> Note: you *should not* use [termcolor](https://docs.rs/termcolor/latest/termcolor/) because it targets the deprecated Console APIs on Windows—and has a significantly more complicated API as a result.
>
> Instead, you *should* use a library that just only supports ANSI color codes, and initialize support for them on Windows with [enable-ansi-support](https://crates.io/crates/enable-ansi-support).

There are two general ways with which color support can be handled. I'm going to call them the "immediate pattern" and the "stylesheet approach", respectively. **Library code that supports colors *should* use the stylesheet approach.** Code in a binary crate can use whichever pattern leads to simpler code.

## The immediate pattern

This pattern is usually presented in examples and tutorials. It is conceptually quite simple.

Here's an example of what it looks like:

```rust,noplaypen
{{#rustdoc_include ../code/managing-colors/src/bin/immediate.rs:immediate}}
```

Notes:
* **`owo_colors::set_override` is used to control color support globally.** The global configuration only has an effect if `if_supports_color` is called.
* **`println!` is paired with `Stream::Stdout`.** If this were `eprintln!`, it would need to be paired with `Stream::Stderr`.

While this pattern is sometimes convenient in binary code, **it *should not* be used in libraries.** That is because libraries *should not* print information directly out to stdout or stderr—instead, they should return values that implement `Display` or similar. Library code *should* use the stylesheet approach instead.

## The stylesheet approach

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
  * Note that the global `set_override` and `unset_override` methods have no impact on library code in the stylesheet example.
  * The global methods are only active if `if_supports_color` is called, as shown by the example for [the immediate pattern] above. This is by design: most libraries shouldn't reach out to global state.
* **The stylesheet is stored as `Box<Styles>`.** The boxing isn't strictly required, but each `Style` is pretty large, and a struct containing e.g. 16 styles is 272 bytes as of owo-colors 3.2.0. That's a pretty large amount of data to store on the stack.
* **`Styles::default()` initializes all the styles to having no effect.** The `colorize()` method then initializes them as required.
* **For custom color support, `Styles` can be made public.** Most library code won't need to give users the ability to customize styles, but this pattern naturally extends to that use case.
* **Use of a separate `MyAppDisplay` type.** The `colorize` call is isolated to this particular `MyAppDisplay`, without influencing other display calls.
* **`println!` is paired with `Stream::Stdout`.** If this were `eprintln!`, it would need to be paired with `Stream::Stderr`.

[the immediate pattern]: #the-immediate-pattern
