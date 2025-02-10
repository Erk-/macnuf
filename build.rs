use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

static MANUF: &str = include_str!("data/manuf.txt");

fn main() {
    let mut data = BTreeMap::<Vec<u8>, String>::new();
    let mut slash_28 = BTreeMap::<Vec<u8>, String>::new();
    let mut slash_36 = BTreeMap::<Vec<u8>, String>::new();

    for line in MANUF.lines() {
        let current_line = line.replace("\t\t", "\t");
        let fields = current_line.split("\t").collect::<Vec<&str>>();

        if fields[0].starts_with("#") || line.is_empty() {
            continue;
        }

        let mac = fields[0].to_string();
        let (mac_only, _) = mac.split_once('/').unwrap_or_else(|| (&mac, ""));
        let mac_only = mac_only.to_owned();
        let eui = mac_only
            .split(&['-', ':'])
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect::<Vec<_>>();
        let manuf = fields[1].to_string();
        if mac.contains(":00/28") {
            slash_28.insert(eui.clone(), manuf.clone());
        } else if mac.contains(":00/36") {
            slash_36.insert(eui.clone(), manuf.clone());
        }
        data.insert(eui, manuf);
    }

    let mut f = String::new();

    writeln!(
        &mut f,
        "pub(crate) const fn mac_lookup(eui: [u8; 6]) -> Option<&'static str> {{"
    )
    .unwrap();
    writeln!(&mut f, "match eui.as_slice() {{").unwrap();

    for (key, value) in data.iter() {
        if key.len() > 3
        {
            continue;
        }

        if value == "IEEE Registration Authority" {
            if slash_28.keys().any(|x| x.starts_with(key)) {
                write!(&mut f, "[").unwrap();
                for bp in key.iter() {
                    write!(&mut f, "0x{bp:02X},").unwrap();
                }
                writeln!(&mut f, " ..] => slash28_lookup(eui),").unwrap();
            } else if slash_36.keys().any(|x| x.starts_with(key)) {
                write!(&mut f, "[").unwrap();
                for bp in key.iter() {
                    write!(&mut f, "0x{bp:02X},").unwrap();
                }
                writeln!(&mut f, " ..] => slash36_lookup(eui),").unwrap();
            }
            continue;
        }

        write!(&mut f, "[").unwrap();
        for bp in key.iter() {
            write!(&mut f, "0x{bp:02X},").unwrap();
        }
        let name = value.trim().replace('\u{200B}', "");
        writeln!(&mut f, " ..] => Some(r#\"{}\"#),", name).unwrap();
    }
    writeln!(&mut f, "_ => None,").unwrap();
    writeln!(&mut f, "}} }}").unwrap();

    writeln!(&mut f, "const fn slash28_lookup(mut eui: [u8; 6]) -> Option<&'static str> {{").unwrap();
    writeln!(&mut f, "eui[3] &= 0xF0;").unwrap();
    writeln!(&mut f, "eui[4] = 0;").unwrap();
    writeln!(&mut f, "eui[5] = 0;").unwrap();
    writeln!(&mut f,   "match eui.as_slice() {{").unwrap();
    for (key, value) in slash_28.iter() {
        write!(&mut f, "[").unwrap();
        for bp in key.iter() {
            write!(&mut f, "0x{bp:02X},").unwrap();
        }
        let name = value.trim().replace('\u{200B}', "");
        writeln!(&mut f, " ..] => Some(r#\"{}\"#),", name).unwrap();
    }
    writeln!(&mut f, "_ => None,").unwrap();
    writeln!(&mut f, "}} }}").unwrap();


    writeln!(&mut f, "const fn slash36_lookup(mut eui: [u8; 6]) -> Option<&'static str> {{").unwrap();
    writeln!(&mut f, "eui[4] &= 0xF0;").unwrap();
    writeln!(&mut f, "eui[5] = 0;").unwrap();
    writeln!(&mut f,   "match eui.as_slice() {{").unwrap();
    for (key, value) in slash_36.iter() {
        write!(&mut f, "[").unwrap();
        for bp in key.iter() {
            write!(&mut f, "0x{bp:02X},").unwrap();
        }
        let name = value.trim().replace('\u{200B}', "");
        writeln!(&mut f, " ..] => Some(r#\"{}\"#),", name).unwrap();
    }
    writeln!(&mut f, "_ => None,").unwrap();
    writeln!(&mut f, "}} }}").unwrap();
    

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lookup.rs");
    std::fs::write(&dest_path, f).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=data/manuf.txt");
}
