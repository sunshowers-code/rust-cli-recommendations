# Adding colors to applications

Colors can make your command-line applications look nice, and also make output easier to understand by leveraging human pattern recognition. However, adding them requires a significant amount of care.

## General recommendations

These rules apply to all command-line programs, not just Rust ones.

1. Applications *should* have a global `--color` option, with the values `always`, `auto` (default) and `never`. If this is specified as `always` or `never`, applications *must* enable or disable colors respectively.
2. Otherwise, if one of a number of environment variables is specified, applications *may* do what it says.[^color]
3. Otherwise, if the output stream (stdout or stderr) is a terminal (also called a "tty"), and the enable colors. Otherwise, disable them.

2 and 3 are covered by the [supports-color](https://docs.rs/supports-color/latest/supports_color/) Rust library. The exact set of environment variables is too complicated to describe here. See [the source code of supports-color](https://docs.rs/supports-color/latest/src/supports_color/lib.rs.html) for a full list.

**It *must* be possible to disable colors.** Some users's terminals may have broken color support; in some cases, pipe detection may not work as expected.

[^color]: This recommendation is somewhat controversial. See [this discussion](https://github.com/rust-lang/rust/pull/27867) in the Rust repository for more about this. I generally believe that using environment variables is OK in any output that's not designed to be machine readable.

## Color palettes

Terminals may support one of three color palettes:
* Truecolor (16 million colors): 24-bit color; 8 bits for each of red, green and blue. This is the standard that web pages and most monitors support. You may have seen these colors written as e.g. <span style="color:#9b4fd1">#9b4fd1</span>.
* 256 colors: 18-bit color; 6 bits for each of red, green and blue. [This page by Pádraig Brady](http://www.pixelbeat.org/docs/terminal_colours/#256) has more information about them.
* 16 colors: 4-bit color; black, red, green, yellow, blue, magenta, cyan, white, and a "bright" version of each.

**The default color schemes in applications *must* be restricted to 12 colors: red, green, yellow, blue, magenta, cyan, and the bright versions of each of these.**
* While the wider palettes are useful for terminal theming controlled by the user, applications *should not* use them. The reason is that users may be using a variety of terminal themes with different backgrounds. **Truecolors and 16-bit colors will not render properly with all terminal themes.** For example, light-colored text will fade into a light background, or dark-colored text will fade into a dark background.
* Most terminals allow you to configure these colors to whatever one pleases. In most themes, these 12 colors are set to contrast with the background.
<tt><span style="color: #acacab; background-color:#050505">Themes with dark backgrounds <span style="color: #a9cdeb">set "blue" to be lighter</span></span></tt>,
while <tt><span style="color: #0e0101; background-color:#ffffdd">themes with light backgrounds <span style="color: #3465a4">set "blue" to be darker</span></span></tt>. (These examples are from real themes.)
* The "black" and "white" colors generally *do not* contrast with the background.

**Applications *may* allow users to set their own color schemes.** If users can set their own color schemes, like [ls](https://man7.org/linux/man-pages/man5/dir_colors.5.html), [emacs](https://www.gnu.org/software/emacs/manual/html_node/emacs/Colors.html) or [vim](https://vimhelp.org/usr_06.txt.html) do, a wider range of colors can be supported. In these cases, users can match their color schemes with their terminal themes.

**Applications *may* use bold text.** Almost all terminals support **bold text**. Some terminals do not support *italic text* or ~~strikethroughs~~, so relying on them can cause issues.

**Applications *must not* use blinking text.** Blinking text can be distracting or difficult to read for some people. The similar `<blink>` tag was [removed from web pages](https://www.fastcompany.com/3015408/saying-goodbye-to-the-html-tag) around 2013.

## ANSI color codes and Windows color APIs

Most Unix terminals support [ANSI color codes](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors). For example, turning the foreground color to "green" involves writing the characters `\x1b` (ESC), `[`, `32` (for green), and `m` to the terminal.

Historically, Windows provided a set of [Console APIs](https://docs.microsoft.com/en-us/windows/console/console-screen-buffers#character-attributes) for the same purpose. These APIs have [since been deprecated](https://docs.microsoft.com/en-us/windows/console/classic-vs-vt), and Windows now supports the same ANSI color codes other platforms do.

**Cross-platform applications *should not* target the Windows Console APIs.** Instead, they should rely on the ANSI color code support built into modern Windows terminals. Note that Windows requires ANSI color code support to be initialized: the [enable-ansi-support](https://github.com/sunshowers/enable-ansi-support) crate does that for you if you're using Rust. Call it early in `main`.