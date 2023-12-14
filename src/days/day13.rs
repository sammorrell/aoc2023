pub fn find_reflect_col(pattern: &Vec<String>, allowed_col_diff: usize) -> Option<usize> {
    (1..pattern[0].chars().count()).find(|mirror_col| {
        let take_n = *vec![*mirror_col, pattern[0].len() - (*mirror_col)]
            .iter()
            .min()
            .unwrap();
        let cols_left = (0..take_n)
            .map(|icol| {
                pattern
                    .iter()
                    .map(|row| row.chars().nth(mirror_col - icol - 1).unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<char>>>();
        let cols_right = (0..take_n)
            .map(|icol| {
                pattern
                    .iter()
                    .map(|row| row.chars().nth(mirror_col + icol).unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<char>>>();

        cols_left
            .iter()
            .zip(cols_right.iter())
            .map(|(left, right)| {
                left.iter().zip(right.iter()).filter(|(l, r)| l != r).count()
            }).sum::<usize>() == allowed_col_diff
    })
}

pub fn find_reflect_row(pattern: &Vec<String>, allowed_row_diff: usize) -> Option<usize> {
    (1..pattern.iter().count()).find(|irow| {
        let take_n = *vec![*irow, pattern.len() - (*irow)].iter().min().unwrap();
        let rows_above = pattern
            .clone()
            .into_iter()
            .skip(irow - take_n)
            .take(take_n)
            .rev()
            .collect::<Vec<String>>();
        let rows_below = pattern
            .clone()
            .into_iter()
            .skip(*irow)
            .take(take_n)
            .collect::<Vec<String>>();

        rows_above
            .iter()
            .zip(rows_below.iter())
            .map(|(above, below)| {
                above.chars().zip(below.chars()).filter(|(l, r)| l != r).count()
            }).sum::<usize>() == allowed_row_diff
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn day13_part1() {
        let patterns = aoctk::io::read_text_chunks(Path::new("data/day13/input.txt"))
            .expect("Invalid input. ");

        let res: Vec<usize> = patterns
            .iter()
            .map(|pat| {
                let col = match find_reflect_col(pat, 0) {
                    Some(col) => col,
                    None => 0,
                };
                let row = match find_reflect_row(pat, 0) {
                    Some(row) => row,
                    None => 0,
                };

                col + 100 * row
            })
            .collect();

        assert_eq!(res.iter().sum::<usize>(), 31739);
    }

    #[test]
    fn day13_part2() {
        let patterns = aoctk::io::read_text_chunks(Path::new("data/day13/input.txt"))
            .expect("Invalid input. ");

        let res: Vec<usize> = patterns
            .iter()
            .map(|pat| {
                let col = match find_reflect_col(pat, 1) {
                    Some(col) => col,
                    None => 0,
                };
                let row = match find_reflect_row(pat, 1) {
                    Some(row) => row,
                    None => 0,
                };

                println!("col: {}, row: {}", col, row);
                col + 100 * row
            })
            .collect();

        assert_eq!(res.iter().sum::<usize>(), 31539);
    }
}
