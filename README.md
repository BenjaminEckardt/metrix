# metrix

Small pet project for code analyzing tool.

Currently only counting LOC.

## usage

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