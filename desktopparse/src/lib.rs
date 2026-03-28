use std::{borrow::Cow, sync::LazyLock};

pub use freedesktop_desktop_entry::DesktopEntry;

pub static LOCALE: LazyLock<Vec<String>> =
    LazyLock::new(freedesktop_desktop_entry::get_languages_from_env);

pub trait WaylandSession {
    fn parse_wmexec(&self) -> Vec<String>;
    fn wm_exec(&self) -> &str;
    fn wm_name<'a>(&'a self) -> Cow<'a, str>;
    fn wm_comment<'a>(&'a self) -> Option<Cow<'a, str>>;
}

impl WaylandSession for DesktopEntry {
    fn wm_name<'a>(&'a self) -> Cow<'a, str> {
        self.name(&LOCALE).unwrap()
    }
    fn parse_wmexec(&self) -> Vec<String> {
        self.parse_exec().unwrap()
    }
    fn wm_exec(&self) -> &str {
        self.exec().unwrap()
    }
    fn wm_comment<'a>(&'a self) -> Option<Cow<'a, str>> {
        self.comment(&LOCALE)
    }
}

pub fn get_all_desktop() -> Vec<DesktopEntry> {
    use glob::glob;
    if let Ok(entry) = glob("/usr/share/wayland-sessions/*.desktop") {
        entry
            .flatten()
            .filter_map(|entry| DesktopEntry::from_path(entry, Some(&LOCALE)).ok())
            .filter(|entry| entry.name(&LOCALE).is_some() && entry.parse_exec().is_ok())
            .collect()
    } else {
        Vec::new()
    }
}
