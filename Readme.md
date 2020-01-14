# Merit

Fast badge generator for any purpose

## Usage

### Web

```
USAGE:

    https://merit-badge.dev/badge/{subject}[/{text}][?params]

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
| **badge with only subject**    | `https://merit-badge.dev/badge/subject`                 | ![badge_sub]   |
| **Default badge**              | `https://merit-badge.dev/badge/subject/text`            | ![badge_def]   |
| **badge with medium size**     | `https://merit-badge.dev/badge/size/medium?size=medium` | ![badge_md]    |
| **badge with large size**      | `https://merit-badge.dev/badge/size/large?size=large`   | ![badge_lg]    |
| **red badge**                  | `https://merit-badge.dev/badge/color/red?color=ff0000`  | ![badge_color] |
| **badge with brand icon**      | `https://merit-badge.dev/badge/icon/brand?icon=npm`     | ![badge_icon1] |
| **badge with solid icon**      | `https://merit-badge.dev/badge/icon/solid?icon=code`    | ![badge_icon2] |
| **badge with sparkline chart** | `https://merit-badge.dev/badge/data/1,5,2,4,8,3,7`      | ![badge_data]  |
| **flat badge**                 | `https://merit-badge.dev/badge/style/flat?style=flat`   | ![badge_flat]  |

> Icon cany be any **Brand** or **Solid** icons from [fontawesome](http://fontawesome.com/icons?d=gallery&s=brands,solid)
> Color can be any 6 or 8 digit hex color, a valid css color name or RGB / RGBA color

### Services

> Same query params as `/badge` can be applied any service

#### npm

```
USAGE:
    GET VERSION             : https://merit-badge.dev/npm[/@{scope}]/{package}
    GET VERSION FOR A TAG   : https://merit-badge.dev/npm[/@{scope}]/{package}/{tag}
    GET LICENSE             : https://merit-badge.dev/npm[/@{scope}]/{package}/lic
    GET DOWNLOAD #          : https://merit-badge.dev/npm[/@{scope}]/{package}/dl/{period}
    GET DOWNLOAD SPARKLINE  : https://merit-badge.dev/npm[/@{scope}]/{package}/hist/{period}

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
| https://merit-badge.dev/npm/typescript/next   | ![npm_tag]   |
| https://merit-badge.dev/npm/@types/react      | ![npm_scope] |
| https://merit-badge.dev/npm/typescript/dl/m   | ![npm_dl]    |
| https://merit-badge.dev/npm/typescript/hist/m | ![npm_hist]  |

#### crates.io

```
USAGE:
    GET VERSION             : https://merit-badge.dev/crates/{package}
    GET LICENSE             : https://merit-badge.dev/crates/{package}/lic
    GET DOWNLOAD #          : https://merit-badge.dev/crates/{package}/dl
    GET DOWNLOAD SPARKLINE  : https://merit-badge.dev/crates/{package}/hist

PATH:
    /lic    : Get the license of the package
    /dl     : Get download count for given period.
    /hist   : Get download trending in sparkline chart

```

|                                              |                  |
| -------------------------------------------- | ---------------- |
| https://merit-badge.dev/crates/actix-web       | ![crates_latest] |
| https://merit-badge.dev/crates/actix-web/lic   | ![crates_lic]    |
| https://merit-badge.dev/crates/actix-web/dl    | ![crates_dl]     |
| https://merit-badge.dev/crates/actix-web/hist  | ![crates_hist]   |

#### github

```
USAGE:
    GET LICENSE   : https://merit-badge.dev/github/{owner}/{name}/lic
    GET START #   : https://merit-badge.dev/github/{owner}/{name}/stars
    GET WATCHERS # : https://merit-badge.dev/github/{owner}/{name}/watchers
    GET FORKS #   : https://merit-badge.dev/github/{owner}/{name}/forks

PATH:
    /{owner}  : User Name or Org
    /{name}  : Name of the repo

```

|                                                     |                   |
| --------------------------------------------------- | ----------------- |
| https://merit-badge.dev/github/rust-lang/rust/lic     | ![github_lic]     |
| https://merit-badge.dev/github/rust-lang/rust/stars   | ![github_stars]   |
| https://merit-badge.dev/github/rust-lang/rust/watchers | ![github_watchers] |
| https://merit-badge.dev/github/rust-lang/rust/forks   | ![github_forks]   |

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

[badge_sub]: https://merit-badge.dev/badge/subject 'badge with only subject'
[badge_def]: https://merit-badge.dev/badge/subject/text 'default badge'
[badge_md]: https://merit-badge.dev/badge/subject/text?size=medium 'badge with medium size'
[badge_lg]: https://merit-badge.dev/badge/subject/text?size=large 'badge with large size'
[badge_color]: https://merit-badge.dev/badge/color/red?color=ff0000 'red badge'
[badge_icon1]: https://merit-badge.dev/badge/icon/brand?icon=npm 'badge with brand icon'
[badge_icon2]: https://merit-badge.dev/badge/icon/solid?icon=code 'badge with solid icon'
[badge_data]: https://merit-badge.dev/badge/data/1,5,2,4,8,3,7 'badge with sparkline chart'
[badge_flat]: https://merit-badge.dev/badge/style/flat?style=flat 'flat badge'
[npm]: https://merit-badge.dev/npm/react
[npm_tag]: https://merit-badge.dev/npm/typescript/next
[npm_scope]: https://merit-badge.dev/npm/@types/react
[npm_dl]: https://merit-badge.dev/npm/typescript/dl/m
[npm_hist]: https://merit-badge.dev/npm/typescript/hist/m
[crates_latest]: https://merit-badge.dev/crates/actix-web
[crates_lic]: https://merit-badge.dev/crates/actix-web/lic
[crates_dl]: https://merit-badge.dev/crates/actix-web/dl
[crates_hist]: https://merit-badge.dev/crates/actix-web/hist
[github_lic]: https://merit-badge.dev/github/rust-lang/rust/lic
[github_stars]: https://merit-badge.dev/github/rust-lang/rust/stars
[github_watchers]: https://merit-badge.dev/github/rust-lang/rust/watchers
[github_forks]: https://merit-badge.dev/github/rust-lang/rust/forks
