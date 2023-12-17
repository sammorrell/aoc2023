#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub hl: usize,
    pub pos: (i32, i32),
    pub dir: (i32, i32),
    pub dir_steps: usize,
}

impl State {
    pub fn summary(&self) -> ((i32, i32), (i32, i32), usize) {
        (self.pos, self.dir, self.dir_steps)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.hl.cmp(&self.hl)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {

    use std::{
        path::Path,
        collections::{BinaryHeap, HashSet},
    };
    use super::*;

    #[test]
    fn day17_part1() {
        let grid = aoctk::io::grid_from_file(Path::new("data/day17/input.txt"))
            .expect("Unable to find grid file.");
        let grid: Vec<Vec<usize>> = grid
            .iter()
            .map(|row| row.iter().map(|val| val.to_string().parse::<usize>().unwrap() ).collect())
            .collect();

        let target = (grid.len() as i32 - 1 , grid[0].len() as i32 - 1);
        let mut states = BinaryHeap::from(vec![State{ hl: 0, pos: (0, 0), dir: (0, 0), dir_steps: 0 }]);
        let mut seen_states = HashSet::new();
        let mut hl = 0;

        while let Some(curr) = states.pop() {
            hl = curr.hl;

            if curr.pos == target {
                break;
            }

            if seen_states.contains(&curr.summary()) { continue; }

            seen_states.insert(curr.summary());

            if curr.dir != (0, 0) && curr.dir_steps < 3 {
                let mut next = curr.clone();
                next.pos.0 = curr.pos.0 + curr.dir.0;
                next.pos.1 = curr.pos.1 + curr.dir.1;
                next.dir_steps += 1;
                if next.pos.0 >= 0 
                        && next.pos.1 >= 0
                        && next.pos.0 < grid.len() as i32 
                        && next.pos.1 < grid[0].len() as i32
                {
                    next.hl += grid[next.pos.0 as usize][next.pos.1 as usize];
                    states.push(next);
                }
            }

            for dir in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if dir != curr.dir && dir != (-curr.dir.0, -curr.dir.1) {
                    let mut next = curr.clone();
                    next.dir = dir;
                    next.pos.0 = next.pos.0 + next.dir.0;
                    next.pos.1 = next.pos.1 + next.dir.1;
                    next.dir_steps = 1;

                    if next.pos.0 >= 0 
                        && next.pos.1 >= 0
                        && next.pos.0 < grid.len() as i32 
                        && next.pos.1 < grid[0].len() as i32
                        {
                            next.hl += grid[next.pos.0 as usize][next.pos.1 as usize];
                            states.push(next);
                    }
                }
            }
        }

        assert_eq!(hl, 1195);
    }

    #[test]
    fn day17_part2() {
        let grid = aoctk::io::grid_from_file(Path::new("data/day17/input.txt"))
            .expect("Unable to find grid file.");
        let grid: Vec<Vec<usize>> = grid
            .iter()
            .map(|row| row.iter().map(|val| val.to_string().parse::<usize>().unwrap() ).collect())
            .collect();

        let target = (grid.len() as i32 - 1 , grid[0].len() as i32 - 1);
        let mut states = BinaryHeap::from(vec![State{ hl: 0, pos: (0, 0), dir: (0, 0), dir_steps: 0 }]);
        let mut seen_states = HashSet::new();
        let mut hl = 0;

        while let Some(curr) = states.pop() {
            hl = curr.hl;

            if curr.pos == target {
                break;
            }

            if seen_states.contains(&curr.summary()) { continue; }

            seen_states.insert(curr.summary());

            if curr.dir != (0, 0) && curr.dir_steps < 10 {
                let mut next = curr.clone();
                next.pos.0 = curr.pos.0 + curr.dir.0;
                next.pos.1 = curr.pos.1 + curr.dir.1;
                next.dir_steps += 1;
                if next.pos.0 >= 0 
                        && next.pos.1 >= 0
                        && next.pos.0 < grid.len() as i32 
                        && next.pos.1 < grid[0].len() as i32
                {
                    next.hl += grid[next.pos.0 as usize][next.pos.1 as usize];
                    states.push(next);
                }
            }

            if curr.dir_steps >= 4 || curr.dir == (0, 0) {
                for dir in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if dir != curr.dir && dir != (-curr.dir.0, -curr.dir.1) {
                        let mut next = curr.clone();
                        next.dir = dir;
                        next.pos.0 = next.pos.0 + next.dir.0;
                        next.pos.1 = next.pos.1 + next.dir.1;
                        next.dir_steps = 1;
    
                        if next.pos.0 >= 0 
                            && next.pos.1 >= 0
                            && next.pos.0 < grid.len() as i32 
                            && next.pos.1 < grid[0].len() as i32
                            {
                                next.hl += grid[next.pos.0 as usize][next.pos.1 as usize];
                                states.push(next);
                        }
                    }
                }
            }
        }

        assert_eq!(hl, 1347);
    }
}