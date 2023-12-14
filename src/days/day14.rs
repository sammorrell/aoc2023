
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

#[cfg(test)]
mod tests {

    use std::path::Path;
    use super::*;

    #[test]
    fn day14_part1() {
        let grid = aoctk::grid::transpose(aoctk::io::grid_from_file(Path::new("data/day14/input.txt")).expect("Could not read grid"));
        let rolled = grid.iter().map(|col| fall(col.clone())).collect::<Vec<Vec<char>>>();
        let rolled = aoctk::grid::transpose(rolled);

        let result = rolled
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter().filter(|&&c| c == 'O').count() * (rolled.iter().count() - i)
            })
            .sum::<usize>();

        assert_eq!(result, 136);
    }
}