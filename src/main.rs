mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

macro_rules! run {
    ($mod:ident) => {
        $mod::part1();
        $mod::part2();
    };
}

fn main() {
    run!(day6);
}
