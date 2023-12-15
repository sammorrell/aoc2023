
#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../../data/day15/input.txt");

    #[test]
    fn day15_part1() {
        let output = INPUT.split(",")
            .map(|seg| {
                seg.chars()
                .fold(0, |accum: i32, val| {
                    let mut tmp = accum + (val as u8) as i32;
                    tmp *= 17;
                    tmp = tmp % 256;
                    tmp
                })
            }).collect::<Vec<i32>>();
        assert_eq!(output.iter().sum::<i32>(), 511498);
    }
}