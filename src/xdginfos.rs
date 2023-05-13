use once_cell::sync::Lazy;

use desktopparse::WaylandDesktop;

pub static DESKTOPS: Lazy<Vec<WaylandDesktop>> = Lazy::new(desktopparse::get_all_desktop);
