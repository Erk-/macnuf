use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufWriter, Write as _},
    path::Path,
};

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

    let path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("lookup.rs");
    let mut f = BufWriter::new(File::create(&path).unwrap());

    writeln!(&mut f, "enum Manuf {{ Manuf(&'static str), S28, S36 }}").unwrap();

    writeln!(
        &mut f,
        "pub(crate) fn mac_lookup(eui: [u8; 6]) -> Option<&'static str> {{"
    )
    .unwrap();
    writeln!(&mut f, "let eui_prefix = [eui[0], eui[1], eui[2]];").unwrap();

    let mut mac_map = phf_codegen::Map::new();
    let mut slash28 = HashSet::new();
    let mut slash36 = HashSet::new();
    for (key, value) in data.iter() {
        if key.len() > 3 {
            continue;
        }

        let arr = [key[0], key[1], key[2]];

        if value == "IEEE Registration Authority" {
            if slash_28.keys().any(|x| x.starts_with(key)) {
                if slash28.insert(arr) {
                    mac_map.entry(arr, "Manuf::S28");
                }
            } else if slash_36.keys().any(|x| x.starts_with(key)) {
                if slash36.insert(arr) {
                    mac_map.entry(arr, "Manuf::S36");
                }
            }

            continue;
        }

        let name = value.trim().replace('\u{200B}', "");
        let name = format!("Manuf::Manuf(r#\"{name}\"#)");
        mac_map.entry(arr, &name);
    }
    writeln!(
        &mut f,
        "static PREFIX: phf::Map<[u8; 3], Manuf> = \n{};\n",
        mac_map.build()
    )
    .unwrap();

    writeln!(
        &mut f,
        "match PREFIX.get(&eui_prefix)? {{ Manuf::Manuf(s) => Some(s), Manuf::S28 => slash28_lookup(eui), Manuf::S36 => slash36_lookup(eui) }}"
    ).unwrap();

    writeln!(&mut f, "}}").unwrap();

    writeln!(
        &mut f,
        "fn slash28_lookup(eui: [u8; 6]) -> Option<&'static str> {{"
    )
    .unwrap();
    let mut mac_map = phf_codegen::Map::new();
    for (key, value) in slash_28.iter() {
        let arr = [key[0], key[1], key[2], key[3] & 0xF0];
        let name = value.trim().replace('\u{200B}', "");
        let name = format!("r#\"{name}\"#");
        mac_map.entry(arr, &name);
    }
    writeln!(
        &mut f,
        "static SLASH28: phf::Map<[u8; 4], &'static str> = \n{};\n",
        mac_map.build()
    )
    .unwrap();
    writeln!(
        &mut f,
        "SLASH28.get(&[eui[0], eui[1], eui[2], eui[3] & 0xF0]).map(|v| &**v)",
    )
    .unwrap();

    writeln!(&mut f, "}}").unwrap();

    writeln!(
        &mut f,
        "fn slash36_lookup(eui: [u8; 6]) -> Option<&'static str> {{"
    )
    .unwrap();
    let mut mac_map = phf_codegen::Map::new();
    for (key, value) in slash_36.iter() {
        let arr = [key[0], key[1], key[2], key[3], key[4] & 0xF0];
        let name = value.trim().replace('\u{200B}', "");
        let name = format!("r#\"{name}\"#");
        mac_map.entry(arr, &name);
    }
    writeln!(
        &mut f,
        "static SLASH36: phf::Map<[u8; 5], &'static str> = \n{};\n",
        mac_map.build()
    )
    .unwrap();
    writeln!(
        &mut f,
        "SLASH36.get(&[eui[0], eui[1], eui[2], eui[3], eui[4] & 0xF0]).map(|v| &**v)",
    )
    .unwrap();
    writeln!(&mut f, "}}").unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=data/manuf.txt");
}
