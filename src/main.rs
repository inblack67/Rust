#![allow(non_snake_case)]

mod arrays;
mod custom_console;
mod strings;
mod tuples;
mod types;
mod vars;

fn main() {
    custom_console::greet();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
}
