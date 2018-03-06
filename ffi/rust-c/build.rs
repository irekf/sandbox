extern crate cc;

fn main() {
    cc::Build::new().file("src/ccode.c").compile("cbin");
}
