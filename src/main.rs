use std::env;

mod challenge;
mod input;
mod runner;

mod one;
mod three;
mod two;

use runner::Runner;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, day, part] => Runner::new().run(&*day, &*part).map_err(|e| e.to_string()),
        _ => Err("`day` and `part` args required".to_string()),
    }
}
