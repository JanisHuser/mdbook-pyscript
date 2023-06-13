# mdbook-mermaid

A preprocessor for [mdbook][] to add [python][] support.

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[pyscript]: [https://mermaidjs.github.io/](https://pyscript.net)

## Usage

```python
print("hii")
```

## Extra Modules

By default the following modules are enabled:
- numpy
- requests
- matplotlib

These are configured in the `load-py-modules.js` file. Any other packages must be added there.

## Installation

```
cargo install --git https://github.com/JanisHuser/mdbook-pyscript
```

Then let `mdbook-pyscript` add the required files and configuration:

```
mdbook-pyscript install path/to/your/book
```


This will add the following configuration to your `book.toml`:

```toml
[preprocessor.python]
command = "mdbook-pyscript"

[output.html]
additional-js = ["pyscript.js", "load-py-modules.js" ]
```

Additionally it copies the files `pyscript.js` and  `load-py-modules.js` into your book's directory.
You find these files in the [`src/bin/assets`](src/bin/assets) directory.

Finally, build your book:

```
mdbook path/to/book
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018-2021 Jan-Erik Rediger <janerik@fnordig.de>

Mermaid is [MIT licensed](https://github.com/knsv/mermaid/blob/master/LICENSE).
The bundled assets (`mermaid.min.js`) are MIT licensed.
