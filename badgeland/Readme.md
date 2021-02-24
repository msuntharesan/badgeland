# Badgeland

## CLI

### Installation

```sh
cargo install badgeland --feature cli
```

### Usage

```sh
❯ cargo badge --help
badgeland
Fast badge generator for any purpose

USAGE:
    cargo-badge [FLAGS] [OPTIONS] <content>

ARGS:
    <content>    Badge content

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

    -c, --classic    Classic badge style (Default)
    -f, --flat       Flat badge style
    -l, --large      Large badge size
    -m, --medium     Medium badge size
    -x, --small      Small badge size (Default)

OPTIONS:
        --color <color>              Badge color. Must be a valid css color
        --icon <icon>                Badge icon. icon can be any `Brand` or `Solid` icons from
                                     fontawesome
        --icon-color <icon-color>    Icon color. Must be a valid css color
    -o, --out <out>                  Output svg to file
    -s, --subject <subject>          Badge subject
```
