
use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn main() {
    let arg: Option<String> = env::args().nth(1);

    if let Some(day) = arg {
        match &*day {
            "1" => day_01::run(),
            "2" => day_02::run(),
            "3" => day_03::run(),
            "4" => day_04::run(),
            "5" => day_05::run(),
            "6" => day_06::run(),
            "7" => day_07::run(),
            "8" => day_08::run(),
            "all" => {
                day_01::run();
                day_02::run();
                day_03::run();
                day_04::run();
                day_05::run();
                day_06::run();
                day_07::run();
                day_08::run();
            }
            _ => panic!("day not implemented")
        }
    } else {
        panic!("Not enough arguments");
    }

}
