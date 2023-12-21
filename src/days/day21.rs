use std::collections::{VecDeque, HashSet};

pub fn walk_grid(grid: &Vec<Vec<char>>, start_pos: (i64, i64), target_steps: i64) -> i64 {
    let ni = grid.len() as i64;
    let nj = grid[0].len() as i64;
    let mut step_queue: VecDeque<(i64, (i64, i64))> = VecDeque::new();
    let mut visited = HashSet::new();
    let mut answers = HashSet::new();

    // Place the starting step onto the queue. 
    step_queue.push_back((target_steps, start_pos));

    while let Some((steps, (i, j))) = step_queue.pop_front() {
        if steps % 2 == 0 {
            answers.insert((i, j));
        }

        if steps == 0 {
            continue;
        }
        
        for dir in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (i + dir.0, j + dir.1);
            let new_steps = steps - 1;
            if new_pos.0 >= 0 && new_pos.1 >= 0 && new_pos.0 < ni && new_pos.1 < nj {
                if grid[new_pos.0 as usize][new_pos.1 as usize] != '#' {
                    if !visited.contains(&new_pos) {
                        visited.insert(new_pos);
                        step_queue.push_back((new_steps, new_pos))
                    }
                }
            }
        }
    }

    answers.iter().count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../data/day21/input.txt");

    #[test]
    fn day21_part1() {
        let garden: Vec<Vec<char>> = INPUT
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Find the starting position, by searching for S.
        let mut start_pos = (0, 0);
        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if garden[i][j] == 'S' {
                    start_pos = (i as i64, j as i64);
                }
            }
        }

        let poss_plots = walk_grid(&garden, start_pos, 64);
        
        assert_eq!(poss_plots, 3746)
    }

    #[test]
    fn day21_part2() {
        let garden: Vec<Vec<char>> = INPUT
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Find the starting position, by searching for S.
        let mut start_pos = (0, 0);
        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if garden[i][j] == 'S' {
                    start_pos = (i as i64, j as i64);
                }
            }
        }

        let ni = garden.len() as i64;
        let nj = garden[0].len() as i64;
        let target_steps = 26501365;
        let grid_size = ni;
        
        // Once again, my response is inspired by Hyper-Neutrino's solution. 
        // I tried the quadratic relationship approach, but it was not precise 
        // enough to get the correct answer (it was the correct ballpark through). 
        // - We assume that the distance walked is a multiple of grid size.
        // - The grid is square. 
        // - The vertical / horizontal lines from the start, and boundaries, aren't blocked. 
        // I am however now considering the parities of the grids and the Manhattan
        // distances to the edges of grids and on edges. 
        let grid_width = target_steps / ni - 1;

        let odd_grids = (grid_width / 2 * 2 + 1).pow(2);
        let odd_visits = walk_grid(&garden, start_pos, grid_size * 2 + 1);

        let even_grids = ((grid_width + 1) / 2 * 2).pow(2);
        let even_visits = walk_grid(&garden, start_pos, grid_size * 2);
        
        let top = walk_grid(&garden, (ni - 1, start_pos.1), ni - 1);
        let right = walk_grid(&garden, (start_pos.0, 0), nj - 1);
        let bottom = walk_grid(&garden, (0, start_pos.1), ni - 1 );
        let left = walk_grid(&garden, (start_pos.0, nj - 1), nj - 1);

        let small_tri_tr = walk_grid(&garden, (ni - 1, 0), grid_size / 2 - 1);
        let small_tri_tl = walk_grid(&garden, (ni - 1, nj - 1), grid_size / 2 - 1);
        let small_tri_br = walk_grid(&garden, (0, 0), grid_size / 2 - 1);
        let small_tri_bl = walk_grid(&garden, (0, nj - 1), grid_size / 2 - 1);

        let large_tri_tr = walk_grid(&garden, (ni - 1, 0), 3 * grid_size / 2 - 1);
        let large_tri_tl = walk_grid(&garden, (ni - 1, nj - 1), 3 * grid_size / 2 - 1);
        let large_tri_br = walk_grid(&garden, (0, 0), 3 * grid_size / 2 - 1);
        let large_tri_bl = walk_grid(&garden, (0, nj - 1), 3 * grid_size / 2 - 1);

        let tot = odd_grids * odd_visits + even_grids * even_visits
                + top + right + bottom + left
                + (grid_width + 1) * (small_tri_bl + small_tri_br + small_tri_tl + small_tri_tr)
                + (grid_width) * (large_tri_bl + large_tri_br + large_tri_tl + large_tri_tr);
        
        assert_eq!(tot, 623540829615589);
    }
}