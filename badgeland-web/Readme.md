# Badgeland-web

```sh
Usage:

    https://merit-badge.dev/badge/{subject}[/{text}][?params]

Path:
    /subject         string
    /text (Optional) string. Text can also be comma separated numbers for sparkline

Query Params:
    color       badge color. Must be a valid css color
    icon        icon can be any "Brand" or "Solid" icons from fontawesome
    icon_color  icon color. Must be a valid css color
    style       [possible values: flat, classic] defaults to classic
    size        [possible values: large, medium, small] defaults to small
```

|                                |                                                     |                |
| ------------------------------ | --------------------------------------------------- | :------------- |
| **Badge with only subject**    | `https://merit-badge.dev/b/text`                    | ![badge_sub]   |
| **Default badge**              | `https://merit-badge.dev/b/subject/text`            | ![badge_def]   |
| **Badge with medium size**     | `https://merit-badge.dev/b/size/medium?size=medium` | ![badge_md]    |
| **Badge with large size**      | `https://merit-badge.dev/b/size/large?size=large`   | ![badge_lg]    |
| **Red badge**                  | `https://merit-badge.dev/b/color/red?color=ff0000`  | ![badge_color] |
| **Badge with brand icon**      | `https://merit-badge.dev/b/icon/brand?icon=npm`     | ![badge_icon1] |
| **Badge with solid icon**      | `https://merit-badge.dev/b/icon/solid?icon=code`    | ![badge_icon2] |
| **Badge with sparkline chart** | `https://merit-badge.dev/b/data/1,5,2,4,8,3,7`      | ![badge_data]  |
| **Flat badge**                 | `https://merit-badge.dev/b/style/flat?style=flat`   | ![badge_flat]  |

> Icon cany be any **Brand** or **Solid** icons from [fontawesome](http://fontawesome.com/icons?d=gallery&s=brands,solid)
> Color can be any 6 or 8 digit hex color, a valid css color name or RGB / RGBA color

## URL

> Generate live badges from your own endpoint.

- **URL**
  `https://merit-badge.dev/url`

- **Method**
  `GET`

- **Query Params**

| param        | type                       | required | Description                                               |
| ------------ | -------------------------- | :------: | --------------------------------------------------------- |
| `source`     | `url`                      |    ✅    | Source for the badge                                      |
| `color`      | `string`                   |          | Any valid css color. Supports Color name, RGB and hex     |
| `icon`       | `string`                   |          | Icon can be any "Brand" or "Solid" icons from fontawesome |
| `icon_color` | `string`                   |          | Any valid css color. Supports Color name, RGB and hex     |
| `style`      | `flat \| classic`          |          | Style of the badge                                        |
| `size`       | `large \| medium \| small` |          | Size of the badge                                         |

- **Source Param is expected to be as following**

  - **METHOD**
    `GET`

  - **Response Type**
    `application/json`

  - **Response Body**

  ```json
  {
        text?: string
        subject: string
        style?: "Flat" | "Classic"
        size?: "Large" | "Medium" | "Small"
        color?: string // Can be any valid CSS color
        icon?: string // Icon can be any "Brand" or "Solid" icons from fontawesome
        icon_color?: string // Can be any valid CSS color
        data?: number[]
    }
  ```

> Query params take presidence if any option is passed in both query param and endpoint.

Runkit example

|                         |                                                                                |                      |
| ----------------------- | ------------------------------------------------------------------------------ | -------------------- |
| Text badge              | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/allText`    | ![runkit_allText]    |
| Data badge              | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/allData`    | ![runkit_allData]    |
| badge with only subject | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/subject`    | ![runkit_subject]    |
| default badge with text | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/withText`   | ![runkit_withText]   |
| Medium size badge       | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/mediumSize` | ![runkit_mediumSize] |
| Large size badge        | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/largeSize`  | ![runkit_largeSize]  |
| Red badge               | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/red`        | ![runkit_red]        |
| badge with brand icon   | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/iconBrand`  | ![runkit_iconBrand]  |
| badge with solid icon   | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/iconSolid`  | ![runkit_iconSolid]  |
| Chart badge             | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/data`       | ![runkit_data]       |
| Flat badge              | `https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/flat`       | ![runkit_flat]       |

[badge_sub]: https://merit-badge.dev/b/text "badge with only text"
[badge_def]: https://merit-badge.dev/b/subject/text "default badge"
[badge_md]: https://merit-badge.dev/b/subject/text?size=medium "badge with medium size"
[badge_lg]: https://merit-badge.dev/b/subject/text?size=large "badge with large size"
[badge_color]: https://merit-badge.dev/b/color/red?color=ff0000 "red badge"
[badge_icon1]: https://merit-badge.dev/b/icon/brand?icon=npm "badge with brand icon"
[badge_icon2]: https://merit-badge.dev/b/icon/solid?icon=code "badge with solid icon"
[badge_data]: https://merit-badge.dev/b/data/1,5,2,4,8,3,7 "badge with sparkline chart"
[badge_flat]: https://merit-badge.dev/b/style/flat?style=flat "flat badge"
[runkit_alltext]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/allText "url badge https://b5vhr8tsmbj6.runkit.sh/allText"
[runkit_alldata]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/allData "url badge https://b5vhr8tsmbj6.runkit.sh/allData"
[runkit_subject]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/subject "url badge https://b5vhr8tsmbj6.runkit.sh/subject"
[runkit_withtext]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/withText "url badge https://b5vhr8tsmbj6.runkit.sh/withText"
[runkit_mediumsize]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/mediumSize "url badge https://b5vhr8tsmbj6.runkit.sh/mediumSize"
[runkit_largesize]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/largeSize "url badge https://b5vhr8tsmbj6.runkit.sh/largeSize"
[runkit_red]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/red "url badge https://b5vhr8tsmbj6.runkit.sh/red"
[runkit_iconbrand]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/iconBrand "url badge https://b5vhr8tsmbj6.runkit.sh/iconBrand"
[runkit_iconsolid]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/iconSolid "url badge https://b5vhr8tsmbj6.runkit.sh/iconSolid"
[runkit_data]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/data "url badge https://b5vhr8tsmbj6.runkit.sh/data"
[runkit_flat]: https://merit-badge.dev/url?source=https://b5vhr8tsmbj6.runkit.sh/flat "url badge https://b5vhr8tsmbj6.runkit.sh/flat"
