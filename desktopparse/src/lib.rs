#[derive(Debug)]
pub struct WaylandDesktop {
    pub filename: String,
    pub name: String,
    pub comment: Option<String>,
    pub exec: String,
    pub desktop_type: Option<String>,
}

impl WaylandDesktop {
    pub fn new<T>(desktop: T, filename: String) -> Option<Self>
    where
        T: ToString,
    {
        let desktop = desktop.to_string();
        let mut name: Option<String> = None;
        let mut comment: Option<String> = None;
        let mut exec: Option<String> = None;
        let mut desktop_type: Option<String> = None;
        let keys = desktop.lines();
        for key in keys {
            let key = key.as_bytes();
            if key.starts_with(b"Name=") {
                let name_value = &key[5..];
                if let Ok(token) = String::from_utf8(name_value.to_vec()) {
                    name = Some(token);
                }
            }
            if key.starts_with(b"Comment=") {
                let comment_value = &key[8..];
                if let Ok(token) = String::from_utf8(comment_value.to_vec()) {
                    comment = Some(token);
                }
            }
            if key.starts_with(b"Exec=") {
                let exec_value = &key[5..];
                if let Ok(token) = String::from_utf8(exec_value.to_vec()) {
                    exec = Some(token);
                }
            }
            if key.starts_with(b"Type=") {
                let type_value = &key[5..];
                if let Ok(token) = String::from_utf8(type_value.to_vec()) {
                    desktop_type = Some(token);
                }
            }
        }
        match (name, exec) {
            (Some(name), Some(exec)) => Some(Self {
                filename,
                name,
                comment,
                exec,
                desktop_type,
            }),
            _ => None,
        }
    }
}

pub fn get_all_desktop() -> Vec<WaylandDesktop> {
    use glob::glob;
    use std::fs::read_to_string;
    if let Ok(entry) = glob("/usr/share/wayland-sessions/*.desktop") {
        entry
            .flatten()
            .filter_map(|enty| {
                read_to_string(enty.clone()).ok().and_then(|message| {
                    WaylandDesktop::new(
                        message,
                        enty.file_stem().unwrap().to_str().unwrap().to_string(),
                    )
                })
            })
            .collect()
    } else {
        Vec::new()
    }
}
