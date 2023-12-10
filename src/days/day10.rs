pub fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found!");
}

fn next_pos(grid: &Vec<Vec<char>>, pos: &(usize, usize), prev: &(usize, usize)) -> (usize, usize) {
    println!("Pos: {:?}, Prev: {:?}, Char: {:?}", pos, prev, grid[pos.0][pos.1]);
    match grid[pos.0][pos.1] {
        '|' => if prev.0 < pos.0 {
            (pos.0 + 1, pos.1)
        } else {
            (pos.0 - 1, pos.1)
        },
        '-' => if prev.1 < pos.1 {
            (pos.0, pos.1 + 1)
        } else {
            (pos.0, pos.1 - 1)
        },
        'L' => if prev.0 < pos.0 {
            (pos.0, pos.1 + 1)
        } else {
            (pos.0 - 1, pos.1)
        },
        'J' => if prev.1 < pos.1 {
            (pos.0 - 1, pos.1)
        } else {
            (pos.0, pos.1 - 1)
        },
        '7' => if prev.1 < pos.1 {
            (pos.0 + 1, pos.1)
        } else {
            (pos.0, pos.1 - 1)
        },
        'F' => if prev.1 > pos.1 {
            (pos.0 + 1, pos.1)
        } else {
            (pos.0, pos.1 + 1)
        },
        'S' => {
            let neighbours = aoctk::grid::GridNeighbours::get_neighbours_for_coord(grid, pos.0, pos.1);
            if neighbours.n.is_some() && ['|', 'F', '7'].contains(&neighbours.n.unwrap()) {
                (pos.0 - 1, pos.1)
            } else if neighbours.s.is_some() && ['|', 'J', 'L'].contains(&neighbours.s.unwrap()) {
                (pos.0 + 1, pos.1)
            } else if neighbours.e.is_some() && ['-', 'J', '7'].contains(&neighbours.e.unwrap()) {
                (pos.0, pos.1 + 1)
            } else if  neighbours.e.is_some() && ['-', 'L', 'F'].contains(&neighbours.w.unwrap()) {
                (pos.0, pos.1 - 1)
            } else {
                panic!("No neighbours!")
            }
        }
        '.' => panic!("Ended in the ground!"),
        _ => panic!("Invalid character!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../data/day10/data.txt");

    #[test]
    fn day10_part1() {
        let pipe_map = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let dest = find_start(&pipe_map);
        let  (mut i, mut j) = dest.clone();
        let mut prev = dest.clone();
        let mut dist = 0;
        while (i, j) != dest || dist == 0 {
            let (newi, newj) = next_pos(&pipe_map, &(i, j), &prev);
            dist += 1;
            prev = (i, j);
            (i, j) = (newi, newj);
        }

        assert_eq!(dist / 2, 100);
    }
}