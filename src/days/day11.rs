#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Galaxy {
    i: i64,
    j: i64,
}

pub fn nodes_with_expansion(lines: &Vec<Vec<char>>, expansion: i64) -> Vec<Galaxy> {
    // We assemble a list of the indi that contain empty rows.
    let i_expand: Vec<i64> = lines
        .iter()
        .enumerate()
        .filter_map(|(no, line)| {
            if !line.contains(&'#') {
                Some(no as i64)
            } else {
                None
            }
        })
        .collect();

    // Do the same as above, but factoring in the empty columns.
    let j_expand: Vec<i64> = (0..lines.len() - 1)
        .map(|j| {
            let col = lines.iter().map(|line| line[j]).all(|c| c != '#');

            if col {
                Some(j as i64)
            } else {
                None
            }
        })
        .filter_map(std::convert::identity)
        .collect();

    // Calculate the indices of all of the galaxies in the data, factoring in the expansion.
    let nodes: Vec<Galaxy> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, c)| {
                    if *c == '#' {
                        let gali = i as i64
                            + i_expand.iter().filter(|&&x| x < i as i64).count() as i64
                                * (expansion - 1);
                        let galj = j as i64
                            + j_expand.iter().filter(|&&x| x < j as i64).count() as i64
                                * (expansion - 1);
                        Some(Galaxy {
                            i: gali as i64,
                            j: galj as i64,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .flatten()
        .collect();
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    const INPUT: &str = include_str!("../../data/day11/input.txt");

    #[test]
    fn day11_part1() {
        let input = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let nodes = nodes_with_expansion(&input, 2);
        let dist_sum: i64 = nodes
            .into_iter()
            .combinations_with_replacement(2)
            .map(|pair| (pair[0].i - pair[1].i).abs() + (pair[0].j - pair[1].j).abs())
            .sum();
        assert_eq!(dist_sum, 9509330);
    }

    #[test]
    fn day11_part2() {
        let input = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        // I saw this coming, so it was simply a case op uppering the expansion coefficient.
        let nodes = nodes_with_expansion(&input, 1_000_000);
        let dist_sum: i64 = nodes
            .into_iter()
            .combinations_with_replacement(2)
            .map(|pair| (pair[0].i - pair[1].i).abs() + (pair[0].j - pair[1].j).abs())
            .sum();
        assert_eq!(dist_sum, 635832237682);
    }
}
