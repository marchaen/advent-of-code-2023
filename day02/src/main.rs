fn main() {
    let input = include_str!("../../input/Day02.txt");
    println!("Solution for part one: {}", solution::part_one(input));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(solution::part_one(input), 8);
    }
}
