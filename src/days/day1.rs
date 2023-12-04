#[cfg(test)]
mod tests {

    use aoctk::io::read_string_col;
    use std::path::Path;

    #[test]
    fn day1_part1() {
        // First we read in the lines of the input file.
        let lines = read_string_col(Path::new("data/day1/data.txt")).unwrap();

        // Find the integers in the lines.
        let answer: u32 = lines
            .iter()
            .map(|line| {
                let digits = line
                    .chars()
                    .filter_map(|char| char.to_digit(10))
                    .collect::<Vec<u32>>();

                // Combine the found numberic digits into integers.
                let first_digit = digits.first().unwrap();
                let second_digit = digits.last().unwrap();

                // Assemble to find the number.
                let found_number = first_digit * 10 + second_digit;
                found_number
            })
            .sum();

        // Check the answer.
        assert_eq!(answer, 53080);
    }

    #[test]
    fn day1_part2() {
        /// This is the same problem as part 1, with the exception that we now have to
        /// detect textual numbers as well as just digits. I think we can implement
        /// this inside the main digit loop with a basic state machine.
        const TEXTUAL_DIGITS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        const NUMERICAL_DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

        // First we read in the lines of the input file.
        let lines = read_string_col(Path::new("data/day1/data.txt")).unwrap();

        // Find the integers in the lines.
        let answer: u32 = lines
            .iter()
            .map(|line| {
                let mut digits: Vec<u32> = Vec::new();
                let mut digit_indices: Vec<usize> = Vec::new();

                // first, look through the line for textual digits.
                for (index, digit) in TEXTUAL_DIGITS.iter().enumerate() {
                    let indices = aoctk::string::find_instances_of(line, *digit);
                    if indices.len() > 0 {
                        digit_indices.extend(indices.clone());
                        digits.extend(vec![(index + 1) as u32; indices.len()]);
                    }
                }

                // Now, look through the line for numerical digits.
                for (index, digit) in NUMERICAL_DIGITS.iter().enumerate() {
                    let indices = aoctk::string::find_instances_of(line, *digit);
                    if indices.len() > 0 {
                        digit_indices.extend(indices.clone());
                        digits.extend(vec![(index + 1) as u32; indices.len()]);
                    }
                }

                // Sort the digits and indices.
                let sortidx = aoctk::sorting::argsort(digit_indices.as_slice());
                digits = sortidx.iter().map(|&idx| digits[idx]).collect();

                println!("Digits: {:?}", digits);
                // Combine the found numberic digits into integers.
                let first_digit = digits.first().unwrap();
                let second_digit = digits.last().unwrap();

                // Assemble to find the number.
                let found_number = first_digit * 10 + second_digit;
                found_number
            })
            .sum();

        // Check the answer.
        assert_eq!(answer, 53268);
    }
}
