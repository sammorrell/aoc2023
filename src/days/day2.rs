#[cfg(test)]
mod tests {

    use aoctk;
    use std::path::Path;

    /// A simple solution to part 1.
    /// We need to iterate through the counts of each drawn colour in each turn in
    /// each game and check the number of each colour that is showing at any given
    /// time. If the number that is shown in a given turn is higher than the maximum
    /// (because they are returns, so we don't have to consider between turns), then
    /// the game is not possible.
    #[test]
    fn day2_part1() {
        // Define the maximum number of possible each colour of cube.
        const MAX_RED_CUBES: u32 = 12;
        const MAX_GREEN_CUBES: u32 = 13;
        const MAX_BLUE_CUBES: u32 = 14;

        // First read in the data from the file.
        let lines = aoctk::io::read_string_col(Path::new("data/day2/data.txt")).unwrap();

        let possible_games_id_total: u32 = lines
            .iter()
            .map(|line| {
                // First get the index of the game.
                let segments = line.split(": ").collect::<Vec<&str>>();
                let game_id = segments[0].replace("Game ", "").parse::<u32>().unwrap();

                // Setup our counters for this game.
                let mut red_cubes = 0;
                let mut green_cubes = 0;
                let mut blue_cubes = 0;

                // Now iterate through the turns in the game.
                let turns = segments[1].split("; ").collect::<Vec<&str>>();

                let game_possible = turns
                    .iter()
                    .map(|turn| {
                        let drawn_colours = turn.split(", ").collect::<Vec<&str>>();

                        for draws in drawn_colours {
                            let draw_segments = draws.split(" ").collect::<Vec<&str>>();
                            let cube_count = draw_segments[0].parse::<u32>().unwrap();
                            let cube_colour = draw_segments[1];

                            // Match up the colour and set our count for the currentoly
                            // shown number of cubes to that value.
                            match cube_colour {
                                "red" => red_cubes = cube_count,
                                "green" => green_cubes = cube_count,
                                "blue" => blue_cubes = cube_count,
                                _ => panic!("Unknown cube colour!"),
                            }
                        }

                        if red_cubes <= MAX_RED_CUBES
                            && green_cubes <= MAX_GREEN_CUBES
                            && blue_cubes <= MAX_BLUE_CUBES
                        {
                            true
                        } else {
                            false
                        }
                    })
                    .all(|x| x == true);

                if game_possible {
                    game_id
                } else {
                    0
                }
            })
            .sum();

        assert_eq!(possible_games_id_total, 2348);
    }

    /// A super basic solutoin to part 2.
    /// We effectively want to know the minimum number of cubes of each colour that
    /// we need to show at any given time.
    #[test]
    fn day2_part2() {
        // First read in the data from the file.
        let lines = aoctk::io::read_string_col(Path::new("data/day2/data.txt")).unwrap();

        let total_powers: u32 = lines
            .iter()
            .map(|line| {
                // First get the index of the game.
                let segments = line.split(": ").collect::<Vec<&str>>();

                // Setup our counters for this game.
                let mut red_cubes = 0;
                let mut green_cubes = 0;
                let mut blue_cubes = 0;

                // Now iterate through the turns in the game.
                let turns = segments[1].split("; ").collect::<Vec<&str>>();

                for turn in turns {
                    let drawn_colours = turn.split(", ").collect::<Vec<&str>>();

                    for draws in drawn_colours {
                        let draw_segments = draws.split(" ").collect::<Vec<&str>>();
                        let cube_count = draw_segments[0].parse::<u32>().unwrap();
                        let cube_colour = draw_segments[1];

                        // A simple case of checking to see if the number of cubes
                        // shown in the current turn exceeds our current maximum.
                        // This means we will only estimate the maximum number of
                        // each colour to make the game possible, hence enabling
                        // us to calculate the total power.
                        match cube_colour {
                            "red" => {
                                if cube_count > red_cubes {
                                    red_cubes = cube_count
                                }
                            }
                            "green" => {
                                if cube_count > green_cubes {
                                    green_cubes = cube_count
                                }
                            }
                            "blue" => {
                                if cube_count > blue_cubes {
                                    blue_cubes = cube_count
                                }
                            }
                            _ => panic!("Unknown cube colour!"),
                        }
                    }
                }

                red_cubes * green_cubes * blue_cubes
            })
            .sum();

        assert_eq!(total_powers, 76008);
    }
}
