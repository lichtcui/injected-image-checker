# injected-image-checker

Verify whether the target image contains additional data, such as video or other types of information.

support:

- png
- jpg
- jpeg
- gif
- svg

### install

```bash
cargo install injected-image-checker
```

### params

- `--path`: target image path

### example

```bash
# mac
injected-image-checker --path="/target/dir/test.png"
```
