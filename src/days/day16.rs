#[derive(Debug, Clone)]
pub struct Beam {
    pub pos: (i32, i32),
    pub dir: (i32, i32),
}

impl Beam {
    pub fn new() -> Beam {
        Beam {
            pos: (0, 0),
            dir: (0, 1),
        }
    }
}

fn get_tile_energies(grid: &Vec<Vec<char>>, in_beam: Beam) -> Vec<Vec<usize>> {
    let mut energy = aoctk::grid::new_grid(grid.len(), grid[0].len());
    let mut splits: Vec<(i32, i32)> = vec![];
    let mut beam_stack: Vec<Beam> = vec![in_beam.clone()];

    while let Some(mut beam) = beam_stack.pop() {
        while beam.pos.0 >= 0 && beam.pos.0 < grid.len() as i32 && beam.pos.1 >= 0 && beam.pos.1 < grid[0].len() as i32 {
            // First, energise the grid at this point. 
            energy[beam.pos.0 as usize][beam.pos.1 as usize] += 1;
            
            let cell = grid[beam.pos.0 as usize][beam.pos.1 as usize];   
            match cell {
                '.' => {},
                '\\' => {
                    let tmp = beam.dir.1;
                    beam.dir.1 = beam.dir.0;
                    beam.dir.0 = tmp;
                },
                '/' => {
                    let tmp = -beam.dir.1;
                    beam.dir.1 = -beam.dir.0;
                    beam.dir.0 = tmp;
                },
                cell if cell == '|' || cell == '-' => {
                    if (cell == '|' && beam.dir.1 != 0) || (cell == '-' && beam.dir.0 != 0) {
                        
                        // Check that we have not already sampled this branch. 
                        if splits.contains(&beam.pos) {
                            break;
                        }

                        let tmp = beam.dir.1;
                        beam.dir.1 = beam.dir.0;
                        beam.dir.0 = tmp;

                        // Put our existing beam on the stack, and create a new one going the opposite direction. 
                        splits.push(beam.pos);
                        let mut new_beam = beam.clone();
                        new_beam.dir.0 = -beam.dir.0;
                        new_beam.dir.1 = -beam.dir.1;
                        beam_stack.push(beam);
                        beam = new_beam;
                    }
                }
                _ => panic!("Invalid cell. "),
            }

            // Now move the beam in the new direction.
            beam.pos.0 += beam.dir.0;
            beam.pos.1 += beam.dir.1;
        }
    }

    energy
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn day16_part1() {
        let grid = aoctk::io::grid_from_file(Path::new("data/day16/input.txt")).expect("Unable to find grid file.");
        let beam = Beam::new();
        let energy = get_tile_energies(&grid, beam);

        let energised_tiles: usize = energy.iter().map(|r| {
            r.iter().filter(|val| **val > 0).count()
        }).sum();

        assert_eq!(energised_tiles, 7562);
    }

}