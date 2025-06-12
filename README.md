# r42

A small command line tool for converting `.r42` templates into source code.

## About r42

r42 reads templates that mix normal text with special markers. Files end with
`*.extension.r42` where `extension` is the file type to generate. Template blocks
start with `<#` and end with `#>`. Expressions use `<#=` to evaluate Rust code
inside the template. The generated function writes to a `String` buffer.

Supported languages are defined in `src/langs.rs` and currently include Rust,
C#, Java, JavaScript and C++.

### Example template

```r42
<#
pub fn render(buffer: &mut String) {
#>

<thing><#=5+5#></thing>

<#}#>
```

## Building

```bash
cargo build
```

This generates the binary `target/debug/r42` (or `target/release/r42` when using `--release`).

## Converting templates

r42 accepts either a language name or a glob pattern as its argument.

### From STDIN

```bash
echo "<thing><#=5+5#></thing>" | ./target/debug/r42 Rust
```

### From files

```bash
./target/debug/r42 "testdata/*.rs.r42"
```

The second form converts all matching templates on disk. The example template
`testdata/test.rs.r42` is converted into `testdata/test.rs` and demonstrates how
the tool works.

