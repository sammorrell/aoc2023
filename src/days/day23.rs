use std::collections::{HashMap, HashSet};

fn find_nodes(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut nodes = Vec::new();
    for i in 0..grid.len() as i64 {
        for j in 0..grid[0].len() as i64 {
            if grid[i as usize][j as usize] == '#' { continue; }

            let mut n_neigh = 0;
            for neigh in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
                if neigh.0 > 0 && neigh.1 > 0 && (neigh.0 as usize) < grid.len() && (neigh.1 as usize) < grid[0].len() && grid[neigh.0 as usize][neigh.1 as usize] != '#' {
                    n_neigh += 1;
                }
            }

            if n_neigh > 2 {
                nodes.push((i as usize, j as usize));
            }
        }
    }
    nodes
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use std::collections::{VecDeque, HashSet, HashMap};

    const INPUT: &str = include_str!("../../data/day23/input.txt");

    #[test]
    fn day23_part1() {
        let grid: Vec<Vec<char>> = INPUT
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let start = (0_usize, grid[0].iter().position(|c| *c == '.').unwrap());
        let end = (grid.len() - 1, grid.last().unwrap().iter().position(|c| *c == '.').unwrap());

        let mut nodes = vec![start, end];
        nodes.append(&mut find_nodes(&grid));


        let avail_dirs = |c: char| -> Vec<(i64, i64)> { 
            match c {
                '^' => vec![(-1, 0)],
                '>' => vec![(0, 1)],
                'v' => vec![(1, 0)],
                '<' => vec![(0, -1)],
                '.' => vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
                _ => panic!("Unexpected char. "),
            }
        };

        // Create a structure to hold the graph connections. 
        let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), u64>> = HashMap::new();
        graph.insert(end, HashMap::new());

        for &(si, sj) in nodes.iter() {
            let mut stack = VecDeque::from([(0, si, sj)]);
            let mut seen = HashSet::from([(si, sj)]);

            while let Some((n, i, j)) = stack.pop_back() {
                if n != 0 && nodes.contains(&(i, j)) {
                    if graph.contains_key(&(si, sj)) {
                        graph.get_mut(&(si, sj)).unwrap().insert((i, j), n);
                    } else {
                        graph.insert((si, sj), HashMap::from_iter([((i, j), n as u64)]));
                    }
                    continue;
                }

                for (di, dj) in avail_dirs(grid[i][j]) {
                    let (ni, nj) = (i as i64 + di, j as i64 + dj);
                    if ni >= 0 && nj >= 0 && (ni as usize) < grid.len() && (nj as usize) < grid[0].len() && grid[ni as usize][nj as usize] != '#' && !seen.contains(&(ni as usize, nj as usize)) {
                        stack.push_back((n + 1, ni as usize, nj as usize));
                        seen.insert((ni as usize, nj as usize));
                    }
                }
            }
        }

        let mut seen = HashSet::new();

        let max_route = dfs(&mut seen, &graph, start);

        assert_eq!(max_route, 94);
    }

}

pub fn dfs(seen: &mut HashSet<(usize, usize)>, graph: &HashMap<(usize, usize), HashMap<(usize, usize), u64>>, key: (usize, usize)) -> u64 {
    seen.insert(key);
    let max = graph
        .get(&key)
        .unwrap()
        .iter()
        .map(|(k, v)| {
            if !seen.contains(k) {  dfs(seen, graph, k.clone()) + v  } else { 0 }
        })
        .max().unwrap_or(0);
    seen.remove(&key);
    max
}