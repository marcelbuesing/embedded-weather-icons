# Embedded Weather-Icons

This library is entirely based on the [weather-icons](https://github.com/erikflowers/weather-icons) repository.
It provides the different icons as [tinybmp::Bmp](https://crates.io/crates/tinybmp) and can easily be used in [embedded-graphics](embedded-graphics) projects.

![Icon Preview](http://i.imgur.com/XmZW2q3.png)

Weather Icons licensed under SIL OFL 1.1
Code licensed under MIT License

```toml
[dependencies]
# Icons are provided in the followings sizes "icons16x16", "icons32x32", "icons64x64", "icons128x128"
embedded-weather-icons = { version = "0.1", features = ["icons32x32"] }
```

```
use embedded_weather_icons as icons;

fn main() {
    let icon = icons::wi_day_sunny_32x32();
}
```

# Development

Bitmaps are regenerated using the `convert_icons.sh` script.
Run `./convert_icons.sh` to regenerate the bitmaps in the assets folder.
