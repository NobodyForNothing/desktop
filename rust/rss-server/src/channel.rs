/// RSS channel
#[derive(Debug)]
pub struct Channel {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub guid: Option<String>,
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            title: None,
            link: None,
            description: None,
            language: None,
            items: Vec::new(),
        }
    }
}

impl Item {
    pub fn new() -> Self {
        Item {
            title: None,
            link: None,
            description: None,
            guid: None,
        }
    }
}