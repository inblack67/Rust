#![allow(non_snake_case)]

mod custom_console;
mod vars;
mod types;

fn main() {
    custom_console::greet();
    vars::run();
    types::run();
}
