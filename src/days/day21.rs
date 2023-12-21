#[cfg(test)]
mod tests {
    use std::collections::{VecDeque, HashSet};

    const INPUT: &str = include_str!("../../data/day21/input.txt");

    #[test]
    fn day21_part1() {
        let garden: Vec<Vec<char>> = INPUT
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let ni = garden.len() as i64;
        let nj = garden[0].len() as i64;

        // Find the starting position, by searching for S.
        let mut start_pos = (0, 0);
        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if garden[i][j] == 'S' {
                    start_pos = (i as i64, j as i64);
                }
            }
        }

        let target_steps: i64 = 64;
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
                    if garden[new_pos.0 as usize][new_pos.1 as usize] != '#' {
                        if !visited.contains(&new_pos) {
                            visited.insert(new_pos);
                            step_queue.push_back((new_steps, new_pos))
                        }
                    }
                }
            }
        }

        let poss_plots = answers.iter().count();
        assert_eq!(poss_plots, 3746)
    }
}