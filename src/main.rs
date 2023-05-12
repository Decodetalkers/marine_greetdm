use once_cell::sync::Lazy;

use desktopparse::WaylandDesktop;

static DESKTOPS: Lazy<Vec<WaylandDesktop>> = Lazy::new(|| desktopparse::get_all_desktop());

fn main() {
    println!("Hello, world!");
}
