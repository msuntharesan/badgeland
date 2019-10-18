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

- Generic

  ```sh
  http://localhost:3000/badge/{subject}
  http://localhost:3000/badge/{subject}/{text}
  http://localhost:3000/badge/{subject}/{text}/{color}
  http://localhost:3000/badge/{subject}/{text}/{color}/{size}
  ```

- [npm](https://www.npmjs.com)

  ```sh
    http://localhost:3000/npm/{package}/npm/{package}
    http://localhost:3000/npm/{package}/npm/{package}/{tag}
    http://localhost:3000/npm/{package}/npm/{package}/lic
    http://localhost:3000/npm/{package}/npm/{package}/dl/{period} # [possible values: d, w, m, y]
    http://localhost:3000/npm/{package}/npm/{package}/hist/{period} # [possible values: d, w, m, y]

    http://localhost:3000/npm/@{scope}/{package}/npm/{package}/
    http://localhost:3000/npm/@{scope}/{package}/npm/{package}/{tag}
    http://localhost:3000/npm/@{scope}/{package}/npm/{package}/lic
    http://localhost:3000/npm/@{scope}/{package}/npm/{package}/dl/{period} # [possible values: d, w, m, y]
    http://localhost:3000/npm/@{scope}/{package}/npm/{package}/hist/{period} # [possible values: d, w, m, y]
  ```

- [crates.io](https://crates.io/)

  ```sh
    http://localhost:3000/crates/{package}/
    http://localhost:3000/crates/{package}/{tag}
    http://localhost:3000/crates/{package}/lic
    http://localhost:3000/crates/{package}/dl
    http://localhost:3000/crates/{package}/hist
  ```

Following querystrings can be applied

- `icon`: icon name
- `color`: icon colour
- `style`: [possible values: Flat, Classic]

> Icon cany be any **Brand** or **Solid** icons from [fontawesome](https://fontawesome.com/icons?d=gallery&s=brands,solid)

> Color can be any 6 or 8 digit hex color or a valid css color name
