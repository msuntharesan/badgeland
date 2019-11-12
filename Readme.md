# Merit

Fast badge generator for any purpose

## Usage

### Web

```
USAGE:

    http://localhost:3000/badge/{subject}[/{text}][?params]

PATH:
    /subject             : String
    /text (Optional)     : String. Text can also be comma separated numbers for sparkline

QUERY PARAMS:
    color       : Any valid css color. Supports Color name, RGB and hex
    icon        : Icon cany be any "Brand" or "Solid" icons from fontawesome
    icon_color  : Any valid css color. Supports Color name, RGB and hex
    style       : [possible values: flat, classic] defaults to classic
    size        : [possible values: large, medium, small] defaults to small
```

|                                |                                                       |                |
| ------------------------------ | ----------------------------------------------------- | :------------- |
| **badge with only subject**    | `http://localhost:3000/badge/subject`                 | ![badge_sub]   |
| **Default badge**              | `http://localhost:3000/badge/subject/text`            | ![badge_def]   |
| **badge with medium size**     | `http://localhost:3000/badge/size/medium?size=medium` | ![badge_md]    |
| **badge with large size**      | `http://localhost:3000/badge/size/large?size=large`   | ![badge_lg]    |
| **red badge**                  | `http://localhost:3000/badge/color/red?color=ff0000`  | ![badge_color] |
| **badge with brand icon**      | `http://localhost:3000/badge/icon/brand?icon=npm`     | ![badge_icon1] |
| **badge with solid icon**      | `http://localhost:3000/badge/icon/solid?icon=code`    | ![badge_icon2] |
| **badge with sparkline chart** | `http://localhost:3000/badge/data/1,5,2,4,8,3,7`      | ![badge_data]  |
| **flat badge**                 | `http://localhost:3000/badge/style/flat?style=flat`   | ![badge_flat]  |

> Icon cany be any **Brand** or **Solid** icons from [fontawesome](https://fontawesome.com/icons?d=gallery&s=brands,solid)
> Color can be any 6 or 8 digit hex color, a valid css color name or RGB / RGBA color

### Services

> Same query params as `/badge` can be applied any service

#### npm

```
USAGE:
    GET VERSION             : http://localhost:3000/npm[/@{scope}]/{package}
    GET VERSION FOR A TAG   : http://localhost:3000/npm[/@{scope}]/{package}/{tag}
    GET LICENSE             : http://localhost:3000/npm[/@{scope}]/{package}/lic
    GET DOWNLOAD #          : http://localhost:3000/npm[/@{scope}]/{package}/dl/{period}
    GET DOWNLOAD SPARKLINE  : http://localhost:3000/npm[/@{scope}]/{package}/hist/{period}

PATH:
    /@{scope}       : Scope of the npm package. Eg: /@angular
    /{tag}          : Specific tag of the package. Eg: beta, next, rc 1.0.8
    /lic            : Get the license of the package
    /dl/{period}    : Get download count for given period.
    /hist/{period}  : Get download trending in sparkline chart
                      Periods are: "d" for Daily, "w" for Weekly, "m" for Monthly, "y" for Yearly,
```

|                                             |              |
| ------------------------------------------- | ------------ |
| http://localhost:3000/npm/typescript/next   | ![npm_tag]   |
| http://localhost:3000/npm/@types/react      | ![npm_scope] |
| http://localhost:3000/npm/typescript/dl/m   | ![npm_dl]    |
| http://localhost:3000/npm/typescript/hist/m | ![npm_hist]  |

#### crates.io

```
USAGE:
    GET VERSION             : http://localhost:3000/crates/{package}
    GET VERSION FOR A TAG   : http://localhost:3000/crates/{package}/{tag}
    GET LICENSE             : http://localhost:3000/crates/{package}/lic
    GET DOWNLOAD #          : http://localhost:3000/crates/{package}/dl
    GET DOWNLOAD SPARKLINE  : http://localhost:3000/crates/{package}/hist

PATH:
    /{tag}  : Specific tag of the package. Eg: beta, next, rc 1.0.8
    /lic    : Get the license of the package
    /dl     : Get download count for given period.
    /hist   : Get download trending in sparkline chart

```

|                                              |                  |
| -------------------------------------------- | ---------------- |
| http://localhost:3000/crates/actix-web       | ![crates_latest] |
| http://localhost:3000/crates/actix-web/alpha | ![crates_tag]    |
| http://localhost:3000/crates/actix-web/lic   | ![crates_lic]    |
| http://localhost:3000/crates/actix-web/dl    | ![crates_dl]     |
| http://localhost:3000/crates/actix-web/hist  | ![crates_hist]   |

#### github

```
USAGE:
    GET LICENSE   : http://localhost:3000/github/{owner}/{name}/lic
    GET START #   : http://localhost:3000/github/{owner}/{name}/stars
    GET WATCHES # : http://localhost:3000/github/{owner}/{name}/watches
    GET FORKS #   : http://localhost:3000/github/{owner}/{name}/forks

PATH:
    /{owner}  : User Name or Org
    /{name}  : Name of the repo

```

|                                                     |                   |
| --------------------------------------------------- | ----------------- |
| http://localhost:3000/github/rust-lang/rust/lic     | ![github_lic]     |
| http://localhost:3000/github/rust-lang/rust/stars   | ![github_stars]   |
| http://localhost:3000/github/rust-lang/rust/watches | ![github_watches] |
| http://localhost:3000/github/rust-lang/rust/forks   | ![github_forks]   |

### CLI

```sh
❯ merit --help
USAGE:
    merit [OPTIONS] --subject <subject>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    --color <color>                6 or 8 digit hex color or a valid css color name
    --data <data>
    --icon <icon>                  Icon cany be any Brand or Solid icons from fontawesome
    --icon-colour <icon-colour>    6 or 8 digit hex color or a valid css color name
    --out <out>
    --size <size>                   [possible values: Large, Medium, Small]
    --style <style>                 [possible values: Flat, Classic]
    --subject <subject>
    --text <text>
```

[badge_sub]: http://localhost:3000/badge/subject 'badge with only subject'
[badge_def]: http://localhost:3000/badge/subject/text 'default badge'
[badge_md]: http://localhost:3000/badge/subject/text?size=medium 'badge with medium size'
[badge_lg]: http://localhost:3000/badge/subject/text?size=large 'badge with large size'
[badge_color]: http://localhost:3000/badge/color/red?color=ff0000 'red badge'
[badge_icon1]: http://localhost:3000/badge/icon/brand?icon=npm 'badge with brand icon'
[badge_icon2]: http://localhost:3000/badge/icon/solid?icon=code 'badge with solid icon'
[badge_data]: http://localhost:3000/badge/data/1,5,2,4,8,3,7 'badge with sparkline chart'
[badge_flat]: http://localhost:3000/badge/style/flat?style=flat 'flat badge'
[npm]: http://localhost:3000/npm/react
[npm_tag]: http://localhost:3000/npm/typescript/next
[npm_scope]: http://localhost:3000/npm/@types/react
[npm_dl]: http://localhost:3000/npm/typescript/dl/m
[npm_hist]: http://localhost:3000/npm/typescript/hist/m
[crates_latest]: http://localhost:3000/crates/actix-web
[crates_tag]: http://localhost:3000/crates/actix-web/alpha
[crates_lic]: http://localhost:3000/crates/actix-web/lic
[crates_dl]: http://localhost:3000/crates/actix-web/dl
[crates_hist]: http://localhost:3000/crates/actix-web/hist
[github_lic]: http://localhost:3000/github/rust-lang/rust/lic
[github_stars]: http://localhost:3000/github/rust-lang/rust/stars
[github_watches]: http://localhost:3000/github/rust-lang/rust/watches
[github_forks]: http://localhost:3000/github/rust-lang/rust/forks
