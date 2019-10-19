# Web

## Generic

```sh
http://localhost:3000/badge/{subject}
http://localhost:3000/badge/{subject}/{text}
http://localhost:3000/badge/{subject}/{text}/{color}
http://localhost:3000/badge/{subject}/{text}/{color}/{size}
```

## [npm](https://www.npmjs.com)

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

## [crates.io](https://crates.io/)

```sh
  http://localhost:3000/crates/{package}/
  http://localhost:3000/crates/{package}/{tag}
  http://localhost:3000/crates/{package}/lic
  http://localhost:3000/crates/{package}/dl
  http://localhost:3000/crates/{package}/hist
```

Following querystrings can be applied

- `icon`: icon name
- `icon_color`: icon colour
- `style`: [possible values: flat, classic]
- `size`: [possible values: large, medium, small]
