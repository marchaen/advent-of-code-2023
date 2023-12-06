fn main() {
    let input = include_str!("../../input/Day02.txt");
    println!("Solution for part one: {}", solution::part_one(input));
    println!("Solution for part two: {}", solution::part_two(input));
}

mod solution {
    use std::str::FromStr;

    struct Game {
        id: i32,
        pulls: Vec<Pull>,
    }

    struct Pull {
        red: i32,
        green: i32,
        blue: i32,
    }

    impl FromStr for Game {
        type Err = Box<dyn std::error::Error>;

        fn from_str(raw_game: &str) -> Result<Self, Self::Err> {
            // Example: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 Green
            let (raw_game_id, raw_pulls) = raw_game
                .split_once(':')
                .ok_or("Invalid raw game input (no colon)")?;

            Ok(Self {
                id: raw_game_id[5..].parse()?,
                pulls: raw_pulls
                    .split(';')
                    .map(str::parse)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl FromStr for Pull {
        type Err = Box<dyn std::error::Error>;

        fn from_str(raw_pull: &str) -> Result<Self, Self::Err> {
            // Example: 3 blue, 4 red
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for raw_cube in raw_pull.split(',') {
                // Example: 3 blue
                let (amount, color) = raw_cube
                    .trim()
                    .split_once(' ')
                    .ok_or("Invalid raw game input (amount cube pair)")?;

                let amount: i32 = amount.parse()?;

                match color {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => {
                        return Err(format!("Invalid raw game input (unknown color {color})").into())
                    }
                }
            }

            Ok(Pull { red, green, blue })
        }
    }

    /// Implementation of the solution for the following problem
    ///
    /// # Problem description from aoc
    ///
    /// As you walk, the Elf shows you a small bag and some cubes which are
    /// either red, green, or blue. Each time you play this game, he will hide
    /// a secret number of cubes of each color in the bag, and your goal is to
    /// figure out information about the number of cubes.
    ///
    /// To get information, once a bag has been loaded with cubes, the Elf will
    /// reach into the bag, grab a handful of random cubes, show them to you,
    /// and then put them back in the bag. He'll do this a few times per game.
    ///
    /// You play several games and record the information from each game
    /// (your puzzle input). Each game is listed with its ID number (like the
    /// `11` in `Game 11: ...`) followed by a semicolon-separated list of
    /// subsets of cubes that were revealed from the bag (like `3 red, 5 green,
    /// 4 blue`).
    ///
    /// For example, the record of a few games might look like this:
    ///
    /// ```
    /// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    /// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    /// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    /// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    /// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    /// ```
    ///
    /// In game 1, three sets of cubes are revealed from the bag (and then put
    /// back again). The first set is 3 blue cubes and 4 red cubes; the second
    /// set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is
    /// only 2 green cubes.
    ///
    /// The Elf would first like to know which games would have been possible
    /// if the bag contained **only 12 red cubes, 13 green cubes, and 14 blue
    /// cubes**?
    ///
    /// In the example above, games 1, 2, and 5 would have been **possible** if
    /// the bag had been loaded with that configuration. However, game 3 would
    /// have been **impossible** because at one point the Elf showed you 20 red
    /// cubes at once; similarly, game 4 would also have been **impossible**
    /// because the Elf showed you 15 blue cubes at once. If you add up the IDs
    /// of the games that would have been possible, you get `8`.
    ///
    /// Determine which games would have been possible if the bag had been
    /// loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. **What
    /// is the sum of the IDs of those games?**
    pub fn part_one(input: &str) -> i32 {
        let games: Vec<Game> = input
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .expect("Parsing the puzzle input failed");

        const MAX_RED: i32 = 12;
        const MAX_GREEN: i32 = 13;
        const MAX_BLUE: i32 = 14;

        let mut valid_games = Vec::<i32>::new();

        for game in games.into_iter() {
            let all_pulls_valid = game.pulls.into_iter().all(|pull| {
                pull.red <= MAX_RED && pull.green <= MAX_GREEN && pull.blue <= MAX_BLUE
            });

            if all_pulls_valid {
                valid_games.push(game.id);
            }
        }

        valid_games.into_iter().sum()
    }

    /// Implementation of the solution for the following problem
    ///
    /// # Problem description from aoc
    ///
    /// As you continue your walk, the Elf poses a second question: in each game
    /// you played, what is the **fewest number of cubes of each color** that
    /// could have been in the bag to make the game possible?
    ///
    /// Again consider the example games from earlier:
    ///
    /// ```
    /// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    /// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    /// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    /// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    /// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    /// ```
    ///
    /// - In game 1, the game could have been played with as few as 4 red, 2
    /// green, and 6 blue cubes. If any color had even one fewer cube, the game
    /// would have been impossible.
    /// - Game 2 could have been played with a minimum of 1 red, 3 green, and 4
    /// blue cubes.
    /// - Game 3 must have been played with at least 20 red, 13 green, and 6
    /// blue cubes.
    /// - Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    /// - Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the
    /// bag.
    ///
    /// The **power** of a set of cubes is equal to the numbers of red, green,
    /// and blue cubes multiplied together. The power of the minimum set of
    /// cubes in game 1 is `48`. In games 2-5 it was `12`, `1560`, `630`, and
    /// `36`, respectively. Adding up these five powers produces the sum **`
    /// 2286`**.
    ///
    /// For each game, find the minimum set of cubes that must have been
    /// present. **What is the sum of the power of these sets?**
    pub fn part_two(input: &str) -> i32 {
        let games: Vec<Game> = input
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .expect("Parsing the puzzle input failed");

        games
            .into_iter()
            .map(|game| {
                let mut biggest_red = 1;
                let mut biggest_green = 1;
                let mut biggest_blue = 1;

                for pull in game.pulls.into_iter() {
                    if pull.red > biggest_red {
                        biggest_red = pull.red;
                    }

                    if pull.green > biggest_green {
                        biggest_green = pull.green;
                    }

                    if pull.blue > biggest_blue {
                        biggest_blue = pull.blue;
                    }
                }

                biggest_red * biggest_green * biggest_blue
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part_one() {
        assert_eq!(solution::part_one(INPUT), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solution::part_two(INPUT), 2286);
    }
}
