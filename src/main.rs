#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod arrays;
mod closures;
mod control_flow;
mod enumerations;
mod for_loop;
mod functions;
mod generics;
mod high_order_fun;
mod match_statement;
mod methods;
pub mod odds_and_ends;
mod options;
mod ownership;
mod pattern_matching;
mod sh;
mod slices;
mod strings;
mod structs;
mod traits;
mod tuples;
mod types_and_variables;
mod vectors;
mod while_and_loops;

fn main() {
    types_and_variables::types_and_variables();
    sh::stack_and_heap();
    control_flow::if_statement();
    while_and_loops::while_and_loop();
    for_loop::for_loop();
    match_statement::match_statement();
    structs::structures();
    enumerations::enums();
    options::option();
    arrays::array();
    vectors::vectors();
    slices::slices();
    strings::strings();
    tuples::tuples();
    pattern_matching::pattern_matching();
    generics::generics();
    functions::functions();
    methods::methods();
    closures::closures();
    high_order_fun::high_order_fun();
    traits::traits();
    ownership::ownership();
    ownership::borrowing();
}
