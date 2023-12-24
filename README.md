# fugitrs

> Python access to libgit2 Rust bindings for git diff patch info and printing

_Fugitrs_ is a Python extension written in Rust using PyO3 aiming to steadily usurp the
functionality of the `fugit` Python package into faster extension code.

## Installation

```sh
maturin develop --release
```

## Usage

To print `git diff`, run:

```py
import fugitrs

fugitrs.get_git_diff()
```

This passes default values equivalent to:

```py
fugitrs.diff(repo=".")
```
