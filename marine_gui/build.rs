fn main() {
    std::env::set_var("SLINT_STYLE", "native");
    slint_build::compile("ui/appwindow.slint").unwrap();
}
