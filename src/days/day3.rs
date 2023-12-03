use std::collections::{HashMap};
use regex::Regex;

// Returns the 
fn find_numbers_in_grid(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), u32> {
    let number_pattern = Regex::new("[0-9]+").expect("Invalid Regex. ");

    // Find runs of one or more numerical characters in each line. 
    grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            number_pattern.find_iter(row.iter().collect::<String>().as_str() ).map(|m| {
                ((i, m.start()), m.as_str().parse::<u32>().expect("Number is not a number. "))
            }).collect::<Vec<((usize, usize), u32)>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {

    use std::{
        path::Path,
        collections::HashSet,
    };
    use super::find_numbers_in_grid;

    #[test]
    fn day3_part1() {
        let schematic = aoctk::io::grid_from_file(Path::new("data/day3/data.txt")).expect("Unable to read grid from file. ");
        let map = find_numbers_in_grid(&schematic);
        
        let final_sum: u32 = map.iter().map(|(coord, number)| {
            // Nice suggestion from: https://stackoverflow.com/a/69298721
            let number_len = number.checked_ilog10().unwrap_or(0) + 1;

            let is_part_number = (0..number_len).map(|j_offset| {
                let mut neighbours = aoctk::grid::GridNeighbours::get_neighbours_for_coord(&schematic, coord.0, coord.1 + j_offset as usize);
                // Handle the case where the east / west cases are the other parts of the number.
                if j_offset > 0 {
                    neighbours.w = None;
                }

                if j_offset < number_len - 1 {
                    neighbours.e = None;
                }

                Into::<Vec<_>>::into(neighbours)
                    .iter()
                    .any(|x| x.unwrap_or('.') != '.')
            }).any(|x| x);

            if is_part_number {
                number.clone()
            } else {
                0
            }
        }).sum();

        assert_eq!(final_sum, 527446);
    }

    #[test]
    fn day3_part2() {
        let schematic = aoctk::io::grid_from_file(Path::new("data/day3/data.txt")).expect("Unable to read grid from file. ");
        let map = find_numbers_in_grid(&schematic);
        let mut geargrid = aoctk::grid::new_grid::<Option<u32>>(schematic.len(), schematic[0].len());
        
        for (coord, number) in map.iter() {
            // Nice suggestion from: https://stackoverflow.com/a/69298721
            let number_len = number.checked_ilog10().unwrap_or(0) + 1;

            let is_part_number = (0..number_len).map(|j_offset| {
                let mut neighbours = aoctk::grid::GridNeighbours::get_neighbours_for_coord(&schematic, coord.0, coord.1 + j_offset as usize);
                // Handle the case where the east / west cases are the other parts of the number.
                if j_offset > 0 {
                    neighbours.w = None;
                }

                if j_offset < number_len - 1 {
                    neighbours.e = None;
                }

                Into::<Vec<_>>::into(neighbours)
                    .iter()
                    .any(|x| x.unwrap_or('.') != '.')
            }).any(|x| x);

            if is_part_number {
                for j_offset in 0..number_len {
                    geargrid[coord.0][coord.1 + j_offset as usize] = Some(*number);
                }
            }
        }

        let cog_coords = aoctk::grid::find_coords_for(&schematic, '*');
        let ratio_sums: u64 = cog_coords.iter().map(|(i, j)| {
            let ratios: Vec<u32> = aoctk::grid::GridNeighbours::get_neighbours_for_coord(&geargrid, *i, *j)
                .into_vec()
                .iter()
                .filter_map(|item| item.unwrap())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
        
            if ratios.len() == 2 {
                ratios.iter().map(|x| *x as u64).product()
            } else {
                0
            }
        }).sum();

        assert_eq!(ratio_sums, 73201705);
    }

}