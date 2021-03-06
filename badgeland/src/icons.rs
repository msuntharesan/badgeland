use std::convert::TryFrom;

include!(concat!(env!("OUT_DIR"), "/icons_map.rs"));

pub fn icon_exists(icon_name: &str) -> bool {
    SYMBOLS.get(icon_name).is_some()
}

pub fn icon_keys() -> Vec<&'static str> {
    SYMBOLS.keys().map(|k| *k).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Icon<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
}

impl<'a> TryFrom<&'a str> for Icon<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        SYMBOLS
            .get(name)
            .map(|symbol| Icon { name, symbol })
            .ok_or("Icon does not exists".into())
    }
}

#[cfg(test)]
mod tests {
    use super::{icon_keys, Icon, SYMBOLS};
    use std::convert::TryFrom;

    #[test]
    fn get_icon_symbol() {
        let icon = Icon::try_from("bluetooth-b");
        assert!(icon.is_ok());
        assert!(icon.unwrap().symbol.len() > 0);
    }

    #[test]
    fn get_icon_keys() {
        assert!(icon_keys().len() > 0);
        assert!(SYMBOLS.contains_key(icon_keys()[0]))
    }
}
