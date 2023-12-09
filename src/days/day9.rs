pub fn extrapolate_next(series: &Vec<i64>) -> i64 {
    let diffs = series
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i64>>();

    if diffs.iter().sum::<i64>() == 0 {
        series.last().unwrap().clone()
    } else {

        extrapolate_next(&diffs) + series.last().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const INPUT: &str = include_str!("../../data/day9/data.txt");

    #[test]
    fn day9_part1() {
        let measurements = INPUT
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();
        
        let extrap_sum: i64 = measurements
            .iter()
            .map(extrapolate_next)
            .sum();

        assert_eq!(extrap_sum, 1806615041);
    }
}