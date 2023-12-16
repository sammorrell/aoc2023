pub fn hashfunc(seg: &str) -> i32 {
    seg.chars().fold(0, |accum: i32, val| {
        let mut tmp = accum + (val as u8) as i32;
        tmp *= 17;
        tmp = tmp % 256;
        tmp
    })
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../../data/day15/input.txt");
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn day15_part1() {
        let output = INPUT.split(",").map(hashfunc).collect::<Vec<i32>>();
        assert_eq!(output.iter().sum::<i32>(), 511498);
    }

    #[test]
    fn day15_part2() {
        let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

        for seg in INPUT.split(",") {
            // First find the box that we are modifying, using the hash function.
            let label = match seg.split_once("=") {
                Some((label, _)) => label.to_string(),
                None => seg.replace("-", "").to_string(),
            };
            let hash = (hashfunc(&label) as usize);
            let mut thisbox = match boxes.get(&hash) {
                Some(b) => b.clone(),
                None => Vec::new(),
            };

            // Then perform the command on the box.
            match seg.split_once("=") {
                None => {
                    let lens = seg.replace("-", "");
                    if let Some(lens_no) = thisbox.iter().position(|(label, _)| *label == lens) {
                        thisbox.remove(lens_no);
                    }
                }
                Some((lens, focus_str)) => {
                    let focus = focus_str.parse().expect("focus was not number. ");
                    // Recall that we need to either replace the existing lens, or push a new one to the back.
                    if let Some(lens_no) = thisbox.iter().position(|(label, _)| *label == lens) {
                        thisbox[lens_no] = (label.to_string(), focus);
                    } else {
                        thisbox.push((label.to_string(), focus));
                    }
                }
            }

            boxes.insert(hash, thisbox);
        }

        let tot: usize = boxes
            .iter()
            .map(|(key, boxx)| {
                boxx.iter()
                    .enumerate()
                    .map(|(idx, (_, val))| (key + 1) * (idx + 1) * val)
                    .sum::<usize>()
            })
            .sum();

        assert_eq!(tot, 284674);
    }
}
