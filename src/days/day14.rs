pub fn fall(col: Vec<char>) -> Vec<char> {
    let mut result = col.clone();
    let mut nchange = 1;
    while nchange > 0 {
        nchange = 0;
        for i in 1..col.len() {
            if result[i] == 'O' && result[i - 1] == '.' {
                result[i - 1] = 'O';
                result[i] = '.';
                nchange += 1;
            }
        }
    }
    result
}

pub fn do_cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = grid.clone();
    for _ in 0..3 {
        res = aoctk::grid::transpose(res);
        res = res
            .iter()
            .map(|col| fall(col.clone()))
            .collect::<Vec<Vec<char>>>();
        res = res
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect()
    }
    res
}

pub fn grid_hash(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
}

pub fn find_cycle_len(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut result = grid.clone();
    let mut i = 0;
    let mut seen_states: Vec<String> = Vec::new();

    loop {
        let new_grid = do_cycle(&result);
        result = new_grid;

        if seen_states.contains(&grid_hash(&result)) {
            break;
        }

        seen_states.push(grid_hash(&result));
        i += 1;
    }

    let offset = seen_states
        .iter()
        .position(|el| *el == grid_hash(&result))
        .unwrap();
    let cycle_len = i - offset;

    (offset, cycle_len)
}

pub fn spin_n(grid: &Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut result = grid.clone();
    for _ in 0..cycles {
        let new_grid = do_cycle(&result);
        result = new_grid;
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::Path;

    #[test]
    fn day14_part1() {
        let grid = aoctk::grid::transpose(
            aoctk::io::grid_from_file(Path::new("data/day14/input.txt"))
                .expect("Could not read grid"),
        );
        let rolled = grid
            .iter()
            .map(|col| fall(col.clone()))
            .collect::<Vec<Vec<char>>>();
        let rolled = aoctk::grid::transpose(rolled);

        let result = rolled
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (rolled.iter().count() - i))
            .sum::<usize>();

        assert_eq!(result, 112048);
    }

    #[test]
    fn day14_part2() {
        const TARGET_CYCLES: usize = 1000000000;
        let grid = aoctk::io::grid_from_file(Path::new("data/day14/input.txt"))
            .expect("Could not read grid");
        let (offset, cycle_len) = find_cycle_len(&grid);
        let spun = spin_n(&grid, offset + (TARGET_CYCLES - offset) % cycle_len);

        let result = spun
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (spun.iter().count() - i))
            .sum::<usize>();

        assert_eq!(result, 105606);
    }
}
