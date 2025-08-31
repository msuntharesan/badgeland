# Badgeland

## CLI

### Installation

```sh
cargo install badgeland --features cli
```

### Usage

```sh
❯ cargo badge --help
cargo-badge
Fast badge generator for any purpose

USAGE:
    cargo badge [OPTIONS] <CONTENT>

ARGS:
    <CONTENT>    Badge content. Can be string or csv

OPTIONS:
    -c, --classic                    Classic badge style (Default)
        --color <COLOR>              Badge color. Must be a valid css color
    -f, --flat                       Flat badge style
    -z, --social                     Social badge style
    -h, --help                       Print help information
        --icon <ICON>                Badge icon. Icons are from
                                     https://fontawesome.com/search?s=brands,
                                     https://fontawesome.com/search?s=solid and
                                     https://simpleicons.org/
        --icon-color <ICON_COLOR>    Icon color. Must be a valid css color
    -l, --large                      Large badge size
    -m, --medium                     Medium badge size
    -o, --out <OUT>                  Output svg to file
    -s, --subject <SUBJECT>          Badge subject
    -x, --small                      Small badge size (Default)
```
