use core::{fmt::Write, str::FromStr};

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String};
use ini_core as ini;

use crate::mt::{
    fs::{MtFile, OpenMode},
    object::Object,
};

pub struct Reader;
pub struct Writer;

pub struct Ini {
    map: BTreeMap<String, BTreeMap<String, Option<String>>>,
}

impl Reader {
    pub fn read(path: &str) -> Result<Ini, String> {
        let file = MtFile::open(path, OpenMode::Read).map_err(|err| format!("{err}"))?;

        let mut buf = unsafe { Box::<[u8]>::new_zeroed_slice(file.size()).assume_init() };
        assert_eq!(file.read(&mut buf), buf.len());
        file.close();
        file.destroy();

        let contents = str::from_utf8(&buf).map_err(|err| format!("{err}"))?;

        let mut map = BTreeMap::new();

        let mut section = "";
        for elem in ini::Parser::new(contents) {
            match elem {
                ini::Item::Error(err) => return Err(err.to_owned()),
                ini::Item::Section(s) => {
                    map.insert(s.to_owned(), BTreeMap::new());
                    section = s;
                }
                ini::Item::Property(name, value) => {
                    map.get_mut(section)
                        .unwrap()
                        .insert(name.trim().to_owned(), value.map(|v| v.trim().to_owned()));
                }
                _ => (),
            }
        }

        Ok(Ini::new(map))
    }
}

impl Writer {
    pub fn write(path: &str, ini: &Ini) -> Result<(), String> {
        let file = MtFile::open(path, OpenMode::Write).map_err(|err| format!("{err}"))?;

        for (section, properties) in &ini.map {
            writeln!(file, "[{}]", section).map_err(|err| format!("{err}"))?;

            for (key, value) in properties {
                match value {
                    Some(v) => writeln!(file, "{} = {}", key, v).map_err(|err| format!("{err}"))?,
                    None => writeln!(file, "{}", key).map_err(|err| format!("{err}"))?,
                }
            }
        }

        file.close();
        file.destroy();

        Ok(())
    }
}

impl Default for Ini {
    fn default() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl Ini {
    pub fn new(map: BTreeMap<String, BTreeMap<String, Option<String>>>) -> Self {
        Self { map }
    }

    /// Set `value` under `section` to `contents`, creating the section if needed.
    pub fn set(&mut self, section: &str, value: &str, contents: String) {
        self.map
            .entry(section.to_owned())
            .or_default()
            .insert(value.to_owned(), Some(contents));
    }

    pub fn get_map(&self) -> &BTreeMap<String, BTreeMap<String, Option<String>>> {
        &self.map
    }

    pub fn get(&self, section: &str, value: &str) -> Option<&String> {
        self.map.get(section)?.get(value)?.as_ref()
    }

    pub fn get_as<T: FromStr>(&self, section: &str, value: &str, default: T) -> T {
        match self.get(section, value) {
            Some(v) => v.as_str().parse::<T>().unwrap_or(default),
            None => default,
        }
    }
}
