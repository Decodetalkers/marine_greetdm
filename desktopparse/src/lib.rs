use std::sync::LazyLock;

pub use freedesktop_desktop_entry::DesktopEntry;

pub static LOCALE: LazyLock<Vec<String>> =
    LazyLock::new(freedesktop_desktop_entry::get_languages_from_env);

pub fn get_all_desktop() -> Vec<DesktopEntry> {
    use glob::glob;
    if let Ok(entry) = glob("/usr/share/wayland-sessions/*.desktop") {
        entry
            .flatten()
            .filter_map(|entry| DesktopEntry::from_path(entry, Some(&LOCALE)).ok())
            .collect()
    } else {
        Vec::new()
    }
}
