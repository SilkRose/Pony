# Friendship is Magic Document

Converts Markdown into FIMFiction BBCode.

## Examples:
Here are examples for how to use the FimDoc executable, and the FimDoc parser library.

### Command line use:
Run with an input and an output file:
```sh
fimdoc input.md output.txt
```

Run with stdin and an output file:
```sh
md | fimdoc output.txt
```

Run with an input file and stdout:
```sh
fimdoc input.md | bbcode
```

Run with stdin and stdout:
```sh
md | fimdoc | bbcode
```

#### Command line options:
FimDoc has the following command line options, only one of which can be provided at a time.

```txt
-w | --warn  (default) >> Warns the user in yellow on unsupported markdown syntax.
-f | --fail >> Errors in red before terminating on unsupported markdown syntax.
-q | --quiet >> Ignores and skips over unsupported markdown syntax.
```

Here are some examples of their use:
```sh
fimdoc -q input.md output.txt
```

```sh
md | fimdoc --fail output.txt
```

### Library use:
Run the parse function with the markdown string and WarningType enum.
```rust
let md = "# Hello World!".to_string();
let warn = WarningType::Quiet;
let bbcode = parse(md, &warn);
println!("{bbcode}");
```

Would produce:
```txt
[h1]
Hello World!
[/h1]
```
