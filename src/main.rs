use std::time::Instant;

use crate::calculator::CalculatorParser;
use rustemo::Parser;

#[rustfmt::skip]
mod calculator;
#[allow(unused)]
#[rustfmt::skip]
mod calculator_actions;

fn main() {
    let expression = String::from(
        "
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
            2+3;
        ",
    );

    let start = Instant::now();
    let result = CalculatorParser::new().parse(&expression);
    let end = Instant::now();
    let duration = end.duration_since(start);

    println!("{:#?}", result);
    println!("Duration: {:#?}", duration.as_secs_f64());
}
