const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 4 Part 1: {}", part1());
    println!("Day 4 Part 2: {}", part2());
}

pub fn part1() -> i32 {
    let grid_width = INPUT.find('\n').unwrap() + 1;

    let strides = [1, grid_width, grid_width + 1, grid_width - 1];

    let mut total = 0;
    for (i, c) in INPUT.char_indices() {
        if c != 'X' {
            continue;
        }
        for stride in strides {
            let i_m = i + stride;
            let i_a = i_m + stride;
            let i_s = i_a + stride;

            if i_s < INPUT.len()
                && INPUT.as_bytes()[i_m] == b'M'
                && INPUT.as_bytes()[i_a] == b'A'
                && INPUT.as_bytes()[i_s] == b'S'
            {
                total += 1;
            }

            if let Some(i_s) = i.checked_sub(stride * 3) {
                let i_a = i_s + stride;
                let i_m = i_a + stride;

                if INPUT.as_bytes()[i_m] == b'M'
                    && INPUT.as_bytes()[i_a] == b'A'
                    && INPUT.as_bytes()[i_s] == b'S'
                {
                    total += 1;
                }
            }
        }
    }

    total
}

pub fn part2() -> i32 {
    let grid_width = INPUT.find('\n').unwrap() + 1;

    let mut total = 0;
    for (i, c) in INPUT.char_indices() {
        if c != 'A' {
            continue;
        }

        let Some(ul) = i.checked_sub(grid_width + 1) else {
            continue;
        };
        let Some(ur) = i.checked_sub(grid_width - 1) else {
            continue;
        };
        let dl = i + grid_width - 1;
        let dr = i + grid_width + 1;
        if dl > INPUT.len() || dr > INPUT.len() {
            continue;
        }

        let ul_dr = (INPUT.as_bytes()[ul] == b'M' && INPUT.as_bytes()[dr] == b'S')
            || (INPUT.as_bytes()[dr] == b'M' && INPUT.as_bytes()[ul] == b'S');
        let ur_dl = (INPUT.as_bytes()[ur] == b'M' && INPUT.as_bytes()[dl] == b'S')
            || (INPUT.as_bytes()[dl] == b'M' && INPUT.as_bytes()[ur] == b'S');

        if ul_dr && ur_dl {
            total += 1;
        }
    }

    total
}
