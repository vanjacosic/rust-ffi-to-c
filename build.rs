extern crate cc;

fn main() {
    cc::Build::new().file("src/multiply.c").compile("multiply");
}
