const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 2 Part 1: {}", part1());
    println!("Day 2 Part 2: {}", part2());
}

fn report_is_safe(report: Vec<i32>, damping: i32) -> bool {
    let mut is_ascending = false;

    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        if i == 0 && b > a {
            is_ascending = true;
        }
        let diff = a.abs_diff(b);
        if diff < 1 || diff > 3 || is_ascending != (b > a) {
            if damping > 0 {
                for i in 0..report.len() {
                    let mut clone = report.clone();
                    clone.remove(i);
                    if report_is_safe(clone, damping - 1) {
                        return true;
                    }
                }
            }
            return false;
        }
    }
    return true;
}

pub fn part1() -> i32 {
    let reports = INPUT.lines();

    let mut safe_reports = 0;
    for line in reports {
        if report_is_safe(
            line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect(),
            0,
        ) {
            safe_reports += 1;
        }
    }
    safe_reports
}

pub fn part2() -> i32 {
    let reports = INPUT.lines();

    let mut safe_reports = 0;
    for line in reports {
        if report_is_safe(
            line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect(),
            1,
        ) {
            println!("{line}");
            safe_reports += 1;
        }
    }
    safe_reports
}
