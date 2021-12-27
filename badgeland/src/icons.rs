use std::convert::TryFrom;

use super::error::IconError;

#[cfg(feature = "static_icons")]
include!(concat!(env!("OUT_DIR"), "/icons_map.rs"));

#[cfg(feature = "static_icons")]
pub fn icon_exists(icon_name: &str) -> bool {
    SYMBOLS.contains_key(icon_name)
}

#[cfg(feature = "static_icons")]
pub fn icon_keys() -> Vec<&'static str> {
    SYMBOLS.keys().map(|&k| k).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Icon<'a> {
    name: &'a str,
    symbol: &'a str,
}

impl<'a> Icon<'a> {
    pub fn new(name: &'a str, symbol: &'a str) -> Icon<'a> {
        Icon { name, symbol }
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn symbol(&self) -> &'a str {
        self.symbol
    }
}

#[cfg(feature = "static_icons")]
impl<'a> TryFrom<&'a str> for Icon<'a> {
    type Error = IconError<'a>;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        SYMBOLS
            .get(name)
            .map(|&symbol| Icon { name, symbol })
            .ok_or(Self::Error { name })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "static_icons")]
    use super::{icon_keys, Icon, SYMBOLS};
    use std::convert::TryFrom;

    #[test]
    fn get_icon_symbol_pass() {
        let icon = Icon::try_from("bluetooth");
        assert!(icon.is_ok());
        assert!(icon.unwrap().symbol.len() > 0);
    }

    #[test]
    fn get_icon_symbol_fail() {
        let icon = Icon::try_from("someicon");
        assert!(icon.is_err());
        assert_eq!(icon.unwrap_err().to_string(), "Invalid Icon Name someicon");
    }

    #[test]
    fn get_icon_keys() {
        assert!(icon_keys().len() > 0);
        assert!(SYMBOLS.contains_key(icon_keys()[0]))
    }
}
