#![allow(non_snake_case)]

mod arrays;
mod channels;
mod closures;
mod control_flow;
mod custom_console;
mod enums;
mod generics;
mod hashmaps;
mod iflet;
mod lifetimes;
mod mpsc;
mod mutex;
mod my_iterators;
mod options;
mod ownership_and_borrowing;
mod pattern_matching;
mod pointers;
mod result_enum;
mod strings;
mod structures;
mod threads;
mod traits;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    custom_console::greet();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    ownership_and_borrowing::run();
    structures::run();
    control_flow::run();
    pattern_matching::run();
    enums::run();
    options::run();
    vectors::run();
    hashmaps::run();
    iflet::run();
    result_enum::run();
    traits::run();
    generics::run();
    pointers::run();
    closures::run();
    my_iterators::run();
    lifetimes::run();
    threads::run();
    channels::run();
    mpsc::run();
    mutex::run();
}
