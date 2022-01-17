# Organizing code in binary crates

Within a binary crate, here's the organization that's *recommended*.

`my-app/src/command.rs`:

```rust
{{#rustdoc_include ../code/organizing-binary/src/command.rs}}
```

`my-app/src/lib.rs`:

```rust
{{#rustdoc_include ../code/organizing-binary/src/lib.rs}}
```

`my-app/src/main.rs`:
```rust
{{#rustdoc_include ../code/organizing-binary/src/main.rs}}
```

Notes:
* **Most of the logic is within `command.rs`.**
  * In general, you *should* keep lib.rs as minimal as possible, unless your entire library fits in it. That's
    because all methods and fields in `lib.rs` are visible to the entire library---code in the top-level module
    cannot be marked private to the rest of the module.
* **There's a `lib.rs` separate from the `main.rs`.**
  * This is because `rustdoc` doesn't use standard privacy rules if building documentation from `main.rs`,
  so private modules are visible in the public documentation.
* **Only the top-level `MyApp` is exported.**
  * The top-level `MyApp` is all `main.rs` should generally need to care about.
* **`MyApp` is marked `#[doc(hidden)]`.**
  * The details of `MyApp` are only meant to be seen by `main`. The library is not part of the public API. Only
  the command-line interface is.
