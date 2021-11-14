
use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;

fn main() {
    let arg: Option<String> = env::args().nth(1);

    let tasks = [
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
        day_06::run,
        day_07::run,
        day_08::run,
        day_09::run,
        day_10::run,
        day_11::run,
        day_12::run,
        day_13::run,
        day_14::run,
        day_15::run,
        day_16::run,
        day_17::run,
        day_18::run,
        day_19::run,
    ];

    if let Some(day) = arg {

        if day == "all" {
            for task in tasks.iter() {
                task();
                println!();
            }
        } else {
            if let Ok(index) = day.parse::<usize>() {
                if index > 0 && index <= tasks.len() {
                    tasks[index-1]();
                } else {
                    panic!("day not implemented");
                }
            } else {
                panic!("not a valid argument");
            }
        }

    } else {
        panic!("not enough arguments");
    }

}
