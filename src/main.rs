#![allow(non_snake_case)]

mod custom_console;
mod strings;
mod types;
mod vars;

fn main() {
    custom_console::greet();
    vars::run();
    types::run();
    strings::run();
}
