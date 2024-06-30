use std::fs;
use std::path::PathBuf;
use log::warn;

pub struct IniFile {
    path: PathBuf,
    sections: Vec<Section>,
}
impl IniFile {
    /// Open an existing file
    pub fn open(path: PathBuf) -> Result<Self, IniFileOpenError> {
        if path.is_file() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Some(sections) = Self::parse(data) {
                    Ok(IniFile{
                        path,
                        sections,
                    })
                } else {
                    Err(IniFileOpenError::FormatError)
                }
            } else {
                Err(IniFileOpenError::IOError)
            }
        } else {
            Err(IniFileOpenError::IOError)
        }
    }

    fn parse(data: String) -> Option<Vec<Section>> {
        let mut sections = Vec::new();
        let mut curr_sect: Option<Section> = None;
        for token in tokenize(&data) {
            match token {
                IniToken::Empty => {}
                IniToken::Comment(_) => {}
                IniToken::SectionHeader(name) => {
                    if let Some(curr_sect) = curr_sect {
                        sections.push(curr_sect);
                    }
                    curr_sect = Some(Section::new(name));
                }
                IniToken::KeyValuePair(key, value) => {
                    if let Some(ref mut curr_sect) = curr_sect {
                        curr_sect.kv_pairs.push((key, value));
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
}

pub struct Section {
    name: String,
    pub(crate) kv_pairs: Vec<(String, String)>,
}

impl Section {
    pub(crate) fn new(name: String) -> Self {
        Section {
            name,
            kv_pairs: Vec::new(),
        }
    }
}

pub enum IniFileOpenError {
    /// File is doesn't exist, is read protected, ect...
    IOError,
    /// File is in invalid format.
    FormatError,
}


fn tokenize(data: &String) -> Vec<IniToken> {
    let mut tokens = Vec::new();
    for line in data.lines() {
        let line = line.trim();
        let t = if line.starts_with(";") || line.starts_with("#") {
            IniToken::Comment(line.to_string())
        } else if line.starts_with("[") && line.ends_with("]") {
            let header = &line[1..(line.len()-1)];
            IniToken::SectionHeader(header.to_string())
        } else if line.is_empty() {
            IniToken::Empty
        } else if let Some(kv) = line.split_once("="){
            let key= kv.0.trim().to_string();
            let value= kv.1.trim().to_string();
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
    Comment(String),
    SectionHeader(String),
    KeyValuePair(String, String),
    Unknown(String),
}
