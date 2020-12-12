use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let(mut cur_dy, mut cur_dx) = (0, 1);
    let mut cur_amt;
    let (mut cur_y, mut cur_x) = (0, 0);
    for line in input.lines() {
        let (dir, amt) = line.split_at(1);
        cur_amt = amt.parse::<i32>().unwrap();
        let mut not_rot = false;
        let (tmp_dy, tmp_dx) = match dir {
            "N" => {
                not_rot = true;
                (1, 0)
            },
            "S" => {
                not_rot = true;
                (-1, 0)
            },
            "E" => {
                not_rot = true;
                (0, 1)
            },
            "W" => {
                not_rot = true;
                (0, -1)
            },
            "L" => {
                for _ in 0..cur_amt / 90 {
                    let tmp = -cur_dy;
                    cur_dy = cur_dx;
                    cur_dx = tmp;
                }
                (cur_dy, cur_dx)
            }
            "R" => {
                for _ in 0..cur_amt / 90 {
                    let tmp = cur_dy;
                    cur_dy = -cur_dx;
                    cur_dx = tmp;
                }
                (cur_dy, cur_dx)
            }
            "F" => {
                not_rot = true;
                (cur_dy, cur_dx)
            },
            _ => continue,
        };
        if not_rot {
            cur_y += cur_amt * tmp_dy;
            cur_x += cur_amt * tmp_dx;
        }
    }
    cur_y.abs() as usize + cur_x.abs() as usize
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut ship_y = 0;
    let mut ship_x = 0;
    let mut wp_y = 1;
    let mut wp_x = 10;
    for line in input.lines() {
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<i32>().unwrap();
        match dir {
            "N" => {
                wp_y += amt;
            },
            "S" => {
                wp_y -= amt;
            },
            "E" => {
                wp_x += amt;
            },
            "W" => {
                wp_x -= amt;
            },
            "L" => {
                for _ in 0..amt / 90 {
                    let tmp = -wp_y;
                    wp_y = wp_x;
                    wp_x = tmp;
                }
            }
            "R" => {
                for _ in 0..amt / 90 {
                    let tmp = wp_y;
                    wp_y = -wp_x;
                    wp_x = tmp;
                }
            }
            "F" => {
                ship_y += amt * wp_y;
                ship_x += amt * wp_x;
            },
            _ => {},
        };
    }
    ship_y.abs() as usize + ship_x.abs() as usize
}
