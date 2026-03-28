use once_cell::sync::Lazy;

use desktopparse::DesktopEntry;

pub static DESKTOPS: Lazy<Vec<DesktopEntry>> = Lazy::new(desktopparse::get_all_desktop);
