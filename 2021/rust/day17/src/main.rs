use std::num::ParseIntError;
use std::str::FromStr;
use utils::read_single_line_stdin;

#[derive(Debug, PartialEq)]
struct TargetArea {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl FromStr for TargetArea {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&['=', '.', ','][..]);
        parts.next();
        let x1 = parts.next().unwrap().parse()?;
        parts.next();
        let x2 = parts.next().unwrap().parse()?;
        parts.next();
        // NOTE: Flipped
        let y2 = parts.next().unwrap().parse()?;
        parts.next();
        let y1 = parts.next().unwrap().parse()?;

        Ok(TargetArea { x1, x2, y1, y2 })
    }
}

#[derive(Debug, PartialEq)]
struct Projectile {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

impl Projectile {
    fn new(v_x: i32, v_y: i32) -> Projectile {
        Projectile {
            x: 0,
            y: 0,
            v_x,
            v_y,
        }
    }
    fn step(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
        self.v_x += if self.v_x > 0 {
            -1
        } else if self.v_x < 0 {
            1
        } else {
            0
        };
        self.v_y -= 1;
    }

    fn past_target_area(&self, target: &TargetArea) -> bool {
        (self.x > target.x2) || (self.y < target.y2)
    }
    fn in_target_area(&self, target: &TargetArea) -> bool {
        (target.x1 <= self.x)
            && (self.x <= target.x2)
            && (target.y1 >= self.y)
            && (self.y >= target.y2)
    }
}

fn part_1(s: &str) -> i32 {
    let target = TargetArea::from_str(s).unwrap();
    let mut global_max_y = 0;
    let mut global_max_v = (0, 0);
    for y in 0..1000 {
        for x in 0..1000 {
            let mut projectile = Projectile::new(x, y);
            let mut has_hit = false;
            let mut local_max_y = 0;
            let mut local_max_v = (0, 0);
            while !projectile.past_target_area(&target) {
                projectile.step();
                if projectile.in_target_area(&target) {
                    has_hit = true;
                }
                if projectile.y > local_max_y {
                    local_max_y = projectile.y;
                    local_max_v = (x, y)
                }
            }
            if has_hit && local_max_y > global_max_y {
                global_max_y = local_max_y;
                global_max_v = local_max_v;
            }
        }
    }

    println!("{:?}", global_max_v);
    global_max_y
}

fn part_2(s: &str) -> i32 {
    let target = TargetArea::from_str(s).unwrap();
    let mut hit_count = 0;
    for y in -1000..1000 {
        for x in -100..1000 {
            let mut projectile = Projectile::new(x, y);
            let mut has_hit = false;
            while !projectile.past_target_area(&target) {
                projectile.step();
                if projectile.in_target_area(&target) {
                    has_hit = true;
                }
            }
            if has_hit {
                hit_count += 1;
            }
        }
    }
    hit_count
}

fn main() {
    let raw_input = read_single_line_stdin();
    println!("{:?}", part_1(raw_input.as_str()));
    println!("{:?}", part_2(raw_input.as_str()));
}

#[test]
fn examples() {
    let raw_input = "target area: x=20..30, y=-10..-5";

    assert_eq!(part_1(raw_input), 45);
    assert_eq!(part_2(raw_input), 112);
}
