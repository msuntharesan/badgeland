# Badge-r-us

A service to generate badges.

## Usage

### CLI

```sh
USAGE:
    badge-r-us [OPTIONS] --subject <subject>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --color <color>
        --data <data>
        --icon <icon>
        --icon_colour <icon_colour>
        --out <out>
        --size <size>                   [possible values: Large, Medium, Small]
        --style <style>                 [possible values: Flat, Classic]
        --subject <subject>
        --text <text>
```

### Web

- `http://localhost:3000/badge/{subject}`
- `http://localhost:3000/badge/{subject}/{text}`
- `http://localhost:3000/badge/{subject}/{text}/{color}`
- `http://localhost:3000/badge/{subject}/{text}/{color}/{size}`

Following querystrings can be applied

- `icon`: icon name
- `color`: icon colour
- `style`: [possible values: Flat, Classic]

> Icon cany be any **Brand** icons from [fontawesome](https://fontawesome.com/icons?d=gallery&s=brands)

> Color can be any 6 or 8 digit hex color or a valid css color name
