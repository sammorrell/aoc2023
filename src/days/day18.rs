#[cfg(test)]
pub fn parse_instruction(instruction: &str) -> (i64, char) {
    let steps =
        i64::from_str_radix(&instruction.chars().skip(2).take(5).collect::<String>(), 16).unwrap();
    let dirchar = match instruction.chars().rev().skip(1).next().unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Invalid instruction code. "),
    };

    (steps, dirchar)
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../data/day18/input.txt");

    #[test]
    fn day18_part1() {
        let mut curr_coord = (0, 0);
        let mut verts = vec![(0, 0)];
        let mut boundary_len = 0;
        let mut other_verts = INPUT
            .lines()
            .map(|line| {
                let segs: Vec<&str> = line.split_whitespace().collect();
                let length: i64 = segs[1].parse().unwrap();
                boundary_len += length;
                let dir = match segs[0] {
                    "U" => (-length, 0),
                    "R" => (0, length),
                    "D" => (length, 0),
                    "L" => (0, -length),
                    _ => panic!("Unexpected direction. "),
                };

                curr_coord.0 += dir.0;
                curr_coord.1 += dir.1;
                curr_coord
            })
            .collect();
        verts.append(&mut other_verts);

        // Now shoelace the vertices to find the interior area.
        let area = verts
            .windows(2)
            .map(|points| {
                println!("{:?}", points);
                points[0].0 * points[1].1 - points[1].0 * points[0].1
            })
            .sum::<i64>()
            / 2;

        // Fix the missing parts of the area using Pick's Theorem.
        let i = area.abs() - boundary_len / 2 + 1;

        assert_eq!(boundary_len + i, 46334);
    }

    #[test]
    fn day18_part2() {
        let mut curr_coord = (0, 0);
        let mut verts = vec![(0, 0)];
        let mut boundary_len = 0;
        let mut other_verts = INPUT
            .lines()
            .map(|line| {
                let instruction: &str = line.split_whitespace().last().unwrap();
                let (length, dirchar) = parse_instruction(instruction);
                boundary_len += length;
                let dir = match dirchar {
                    'U' => (-length, 0),
                    'R' => (0, length),
                    'D' => (length, 0),
                    'L' => (0, -length),
                    _ => panic!("Unexpected direction. "),
                };

                curr_coord.0 += dir.0;
                curr_coord.1 += dir.1;
                curr_coord
            })
            .collect();
        verts.append(&mut other_verts);

        // Now shoelace the vertices to find the interior area.
        let area = verts
            .windows(2)
            .map(|points| {
                println!("{:?}", points);
                points[0].0 * points[1].1 - points[1].0 * points[0].1
            })
            .sum::<i64>()
            / 2;

        // Fix the missing parts of the area using Pick's Theorem.
        let i = area.abs() - boundary_len / 2 + 1;

        assert_eq!(boundary_len + i, 102000662718092);
    }
}
