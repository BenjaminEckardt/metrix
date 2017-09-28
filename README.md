# metrix [![Build Status][travis-image]][travis-url]
Small pet project of a code analyzing tool - just to write some rust.

Currently only counting __LOC__.

## usage
Use `cargo install` in this project root to install the binary.

Use it in the project you want to analyze
```bash
$ metrix
```

## sample output
```
+----------------+-----+
| Path           | LOC |
+----------------+-----+
| Cargo.lock     | 120 |
+----------------+-----+
| src/main.rs    | 57  |
+----------------+-----+
| src/metrics.rs | 32  |
+----------------+-----+
| LICENSE        | 17  |
+----------------+-----+
| .travis.yml    | 8   |
+----------------+-----+
| Cargo.toml     | 7   |
+----------------+-----+
| .gitignore     | 2   |
+----------------+-----+
| README.md      | 2   |
+----------------+-----+
```

[travis-image]: https://api.travis-ci.org/BenjaminEckardt/metrix.svg?branch=master
[travis-url]: https://travis-ci.org/BenjaminEckardt/metrix

