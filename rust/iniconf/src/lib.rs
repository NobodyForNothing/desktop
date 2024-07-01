use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use log::warn;

#[derive(Debug)]
pub struct IniFile {
    path: PathBuf,
    sections: HashMap<String, Section>,
}
impl IniFile {
    /// Open an existing file or create a reference to a new file.
    pub fn open(path: PathBuf) -> Result<Self, IniFileOpenError> {
        if path.is_file() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Some(sections) = Self::parse(data) {
                    Ok(IniFile{
                        path,
                        sections: sections
                            .into_iter()
                            .map(|sect| (sect.name.to_string(), sect))
                            .collect()
                    })
                } else {
                    Err(IniFileOpenError::FormatError)
                }
            } else {
                Err(IniFileOpenError::IOError)
            }
        } else {
            // File doesn't exist
            Ok(IniFile{
                path,
                ..Self::default()
            })
        }
    }

    fn parse(data: String) -> Option<Vec<Section>> {
        let mut sections = Vec::new();
        let mut curr_sect: Option<Section> = None;
        for token in tokenize(&data) {
            match token {
                IniToken::Empty => {}
                IniToken::Comment => {}
                IniToken::SectionHeader(name) => {
                    if let Some(curr_sect) = curr_sect {
                        sections.push(curr_sect);
                    }
                    curr_sect = Some(Section::new(name));
                }
                IniToken::KeyValuePair(key, value) => {
                    if let Some(ref mut curr_sect) = curr_sect {
                        curr_sect.set(key, value);
                    } else {
                        return None;
                    }
                }
                IniToken::Unknown(line) => {
                    warn!("Unrecognized token: {line}");
                }
            }
        }
        Some(sections)
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut string = String::new();
        for sect in self.sections.values() {
            string.push_str(format!("[{}]\n", &sect.name).as_str());
            for (k, v) in &sect.kv {
                string.push_str(format!("  {} = {}\n", &k, &v).as_str());
            }
        }
        fs::write(&self.path, string)
    }

    /// Read the value of [key] in [section] if it exists.
    pub fn get_string(&self, section: &str, key: &str) -> Option<&String> {
        let sect = self.sections.get(section)?;
        sect.get(key.to_string())
    }

    /// Read the value of [key] in [section] if it exists.
    pub fn get<F: std::str::FromStr>(&self, section: &str, key: &str) -> Option<F> {
        let str = self.get_string(section, key)?;
        str.parse::<F>().ok()
    }

    pub fn set_str(&mut self, section: &str, key: &str, value: &str) {
        let sect = self.sections
            .entry(section.to_string())
            .or_insert(Section::new(section.to_string()));
        sect.set(key.to_string(), value.to_string());
    }
}

impl Default for IniFile {
    fn default() -> Self {
        Self {
            path: PathBuf::default(),
            sections: HashMap::default(),
        }
    }
}

#[derive(Debug)]
struct Section {
    pub(crate) name: String,
    pub(crate) kv: HashMap<String, String>,
}

impl Section {
    pub(crate) fn new(name: String) -> Self {
        Section {
            name,
            kv: HashMap::new(),
        }
    }

    pub(crate) fn set(&mut self, key: String, value: String) {
        self.kv.insert(key, value);
    }

    pub(crate) fn get(&self, key: String) -> Option<&String> {
        self.kv.get(&key)
    }
}

#[derive(Debug)]
pub enum IniFileOpenError {
    /// File is read protected, ect...
    IOError,
    /// File is in invalid format.
    FormatError,
}


fn tokenize(data: &String) -> Vec<IniToken> {
    let mut tokens = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        let t = if line.starts_with(";") || line.starts_with("#") {
            IniToken::Comment
        } else if line.starts_with("[") && line.ends_with("]") {
            let header = &line[1..(line.len()-1)];
            IniToken::SectionHeader(header.to_string())
        } else if line.is_empty() {
            IniToken::Empty
        } else if let Some(kv) = line.split_once("="){
            let key= kv.0.trim().to_string();
            let value= kv.1.trim();
            let value = value.trim_matches('"').to_string();
            IniToken::KeyValuePair(key, value)
        } else {
            IniToken::Unknown(line.to_string())
        };
        tokens.push(t);
    }
    tokens
}

enum IniToken {
    Empty,
    Comment,
    SectionHeader(String),
    KeyValuePair(String, String),
    Unknown(String),
}
