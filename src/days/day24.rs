use itertools::Itertools;
use num::traits::Pow;
type Hailstone = ((f64, f64, f64), (f64, f64, f64));

pub fn parse_input(instr: &str) -> Vec<Hailstone> {
    instr
        .lines()
        .map(|line| {
            let (pos_str, vel_str) = line.split_once(" @ ").expect("Invalid format input. ");
            let pos: (f64, f64, f64) = pos_str
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .expect("Invalid position");
            let vec: (f64, f64, f64) = vel_str
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .expect("Invalid velocity");
            (pos, vec)
        })
        .collect()
}

/// Checks for decreasing distance to see if collision is the past.
/// Likely a better way to do this, but it works for this. 
pub fn point_in_future_2d(t: &(f64, f64, f64), p: &(f64, f64, f64), v: &(f64, f64, f64)) -> bool {
    let init_dist: f64 = f64::sqrt((t.0 - p.0).pow(2) + (t.1 - p.1).pow(2));
    let final_dist: f64 = f64::sqrt((t.0 - p.0 - v.0).pow(2) + (t.1 - p.1 - v.1).pow(2));
    init_dist >= final_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../data/day24/input.txt");

    #[test]
    fn day24_part1() {
        let hailstones: Vec<Hailstone> = parse_input(INPUT);
        let (testx, testy) = ((200000000000000_f64, 400000000000000_f64), (200000000000000_f64, 400000000000000_f64));
        //let (testx, testy) = ((7.0, 27.0), (7.0, 27.0));

        let mut tot_intersect = 0;
        for pair in hailstones.iter().combinations(2) {
            let (p1, v1) = pair.first().unwrap();
            let (p2, v2) = pair.last().unwrap();

            // Don't compare the same hailstone. 
            if (p1, v1) == (p2, v2) {
                continue;
            }

            // Check if the lines are parallel. 
            let a = v1.1 / v1.0;
            let b = v2.1 / v2.0;
            if a == b {
                // The lines are parallel, they will not meet. 
                continue;
            }

            // Find their intersection point. 
            let c = p1.1 - a * p1.0;
            let d = p2.1 - b * p2.0;
            let x_intersect = (d - c) / (a - b);
            let y_intersect = a * x_intersect + c;

            // Check for the intersection in the test area.
            if x_intersect >= testx.0 && x_intersect <= testx.1 && y_intersect >= testy.0 && y_intersect <= testy.1 {
                if point_in_future_2d(&(x_intersect, y_intersect, 0.0), p1, v1) && point_in_future_2d(&(x_intersect, y_intersect, 0.0), p2, v2) {
                    tot_intersect += 1;
                }
            }
        }

        assert_eq!(tot_intersect, 17776);
    }

}