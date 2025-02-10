#![no_std]
//! This crate works by building a match at compile time which is then used to lookup in.

mod eui;
mod lookup {
    include!(concat!(env!("OUT_DIR"), "/lookup.rs"));
}

pub use eui::{EUI48ParseError, EUI48};

/// Lookup a manufacturer by a mac address.
///
/// # Example
/// ```rust
/// let manuf = macnuf::lookup("90:84:2B:12:34:56".parse().unwrap()).unwrap();
/// assert_eq!(manuf, "LEGO System A/S");
/// ```
pub fn lookup(eui: EUI48) -> Option<&'static str> {
    lookup::mac_lookup(eui.inner)
}

#[cfg(test)]
mod tests {
    use crate::{lookup, EUI48};

    #[test]
    fn d_link() {
        let manuf = lookup("C4:A8:1D:73:D7:8C".parse().unwrap()).unwrap();
        assert_eq!(manuf, "D-Link International")
    }

    #[test]
    fn netgear() {
        let manuf = lookup("9C:D3:6D:9A:CA:81".parse().unwrap()).unwrap();
        assert_eq!(manuf, "Netgear")
    }

    #[test]
    fn shanghai_broadwan_communications() {
        let manuf = lookup("40:ED:98:6F:DB:AC".parse().unwrap()).unwrap();
        assert_eq!(manuf, "Shanghai Broadwan Communications Co.,Ltd")
    }

    #[test]
    fn piranha_ems() {
        let manuf = lookup("70:B3:D5:8C:CD:BE".parse().unwrap()).unwrap();
        assert_eq!(manuf, "Piranha EMS Inc.")
    }

    #[test]
    fn slash_24_lookup() {
        let manuf = lookup("3C:24:F0:42:BE:CF".parse().unwrap()).unwrap();
        assert_eq!(manuf, "Inter-Coastal Electronics")
    }

    #[test]
    fn samsung_electronics() {
        let manuf = lookup("24:FC:E5:AD:BB:89".parse().unwrap()).unwrap();
        assert_eq!(manuf, "Samsung Electronics Co.,Ltd")
    }

    #[test]
    fn invalid_address() {
        "G4:FC:E5:AD:BB:89".parse::<EUI48>().unwrap_err();
    }
}
