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
            if  neighbours.w.is_some() && ['-', 'L', 'F'].contains(&neighbours.w.unwrap()) {
                (pos.0, pos.1 - 1)
            } else if neighbours.s.is_some() && ['|', 'J', 'L'].contains(&neighbours.s.unwrap()) {
                (pos.0 + 1, pos.1)
            } else if neighbours.e.is_some() && ['-', 'J', '7'].contains(&neighbours.e.unwrap()) {
                    (pos.0, pos.1 + 1)
            } else if neighbours.n.is_some() && ['|', 'F', '7'].contains(&neighbours.n.unwrap()) {
                (pos.0 - 1, pos.1)
            } else {
                panic!("No neighbours!")
            }
        }
        '.' => panic!("Ended in the ground!"),
        _ => panic!("Invalid character!"),
    }

}

use std::io::Write;
fn count_inside(grid: &Vec<Vec<char>>, boundary: &Vec<Vec<bool>>) -> usize {
    let mut inside_map = aoctk::grid::new_grid::<usize>(grid.len(), grid[0].len());
    let res = (0..grid.len())
        .map(|i| {
            (0..grid[0].len())
                .map(|j| {
                    let crossings = trace_boundary_ray(grid, boundary, &(i, j));
                    if !boundary[i][j] && crossings % 2 == 1 {
                        inside_map[i][j] = 1;
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();
    res
}

fn trace_boundary_ray(grid: &Vec<Vec<char>>, boundary: &Vec<Vec<bool>>, pos: &(usize, usize)) -> usize {
    let i = pos.0 as i64;
    let mut j = pos.1 as i64;

    let mut crossings: usize = 0;
    while i >= 0 && j >= 0 && i < boundary.len() as i64 && j < boundary[0].len() as i64 {
        if boundary[i as usize][j as usize] && ['|', 'J', 'L', 'S'].contains(&grid[i as usize][j as usize]) {
            crossings += 1;
        }
        j -= 1;
    }
    crossings
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

        assert_eq!(dist / 2, 6682);
    }

    #[test]
    fn day10_part2() {
        let pipe_map = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        
        let mut boundary_map = aoctk::grid::new_grid::<bool>(pipe_map.len(), pipe_map[0].len());
        let dest = find_start(&pipe_map);
        let  (mut i, mut j) = dest.clone();
        let mut prev = dest.clone();
        let mut dist = 0;
        while (i, j) != dest || dist == 0 {
            boundary_map[i][j] = true;
            let (newi, newj) = next_pos(&pipe_map, &(i, j), &prev);
            prev = (i, j);
            dist += 1;
            (i, j) = (newi, newj);
        }
        let area = count_inside(&pipe_map, &boundary_map);
       
        assert_eq!(area + 1, 353); // Too High. 
    }
}