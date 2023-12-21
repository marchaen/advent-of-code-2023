fn main() {
    let input = include_str!("../../input/Day08.txt");
    println!("{}", solution::part_one(input));
}

mod solution {
    use std::collections::HashMap;

    #[derive(Debug)]
    enum Instruction {
        Left,
        Right,
    }

    fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
        let (raw_instructions, raw_map_points) = input
            .split_once("\n\n")
            .expect("Expect instructions then an empty line and then the map points.");

        let instructions = raw_instructions
            .chars()
            .map(|raw_instruction| match raw_instruction {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Instructions have to be either L or R."),
            })
            .collect();

        let map_points = raw_map_points
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|raw_map_point| {
                let (name, connected_points) = raw_map_point
                    .split_once(" = (")
                    .expect("Expect map point in format 'name = (left, right)'.");

                let (left, right) = connected_points
                    .strip_suffix(')')
                    .expect("Expect closing brace at the end of a map point.")
                    .split_once(", ")
                    .expect("Expect connected points to be comma space separated.");

                (name.to_owned(), (left.to_owned(), right.to_owned()))
            })
            .collect();

        (instructions, map_points)
    }

    /// Implementation of the solution for the following problem
    ///
    /// # Problem description from aoc
    ///
    /// One of the camel's pouches is labeled "maps" - sure enough, it's full
    /// of documents (your puzzle input) about how to navigate the desert. At
    /// least, you're pretty sure that's what they are; one of the documents
    /// contains a list of left/right instructions, and the rest of the
    /// documents seem to describe some kind of **network** of labeled nodes.
    ///
    /// It seems like you're meant to use the **left/right** instructions to
    /// **navigate the network**. Perhaps if you have the camel follow the same
    /// instructions, you can escape the haunted wasteland!
    ///
    /// After examining the maps for a bit, two nodes stick out: `AAA` and
    /// `ZZZ`. You feel like `AAA` is where you are now, and you have to follow
    /// the left/right instructions until you reach `ZZZ`.
    ///
    /// This format defines each **node** of the network individually. For
    /// example:
    ///
    /// ```
    /// RL
    ///
    /// AAA = (BBB, CCC)
    /// BBB = (DDD, EEE)
    /// CCC = (ZZZ, GGG)
    /// DDD = (DDD, DDD)
    /// EEE = (EEE, EEE)
    /// GGG = (GGG, GGG)
    /// ZZZ = (ZZZ, ZZZ)
    /// ```
    ///
    /// Starting with `AAA`, you need to **look up the next element** based on
    /// the next left/right instruction in your input. In this example, start
    /// with `AAA` and go **right** (`R`) by choosing the right element of
    /// `AAA`, **`CCC`**. Then, `L` means to choose the **left** element of
    /// `CCC`, **`ZZZ`**. By following the left/right instructions, you reach
    /// `ZZZ` in **`2`** steps.
    ///
    /// Of course, you might not find `ZZZ` right away. If you run out of
    /// left/right instructions, repeat the whole sequence of instructions as
    /// necessary: `RL` really means `RLRLRLRLRLRLRLRL...` and so on. For
    /// example, here is a situation that takes **`6`** steps to reach `ZZZ`:
    ///
    /// ```
    /// LLR
    ///
    /// AAA = (BBB, BBB)
    /// BBB = (AAA, ZZZ)
    /// ZZZ = (ZZZ, ZZZ)
    /// ```
    ///
    /// Starting at `AAA`, follow the left/right instructions. **How many steps
    /// are required to reach `ZZZ`?**
    pub fn part_one(input: &str) -> u32 {
        let (instructions, map_points) = parse_input(input);

        let mut next_instruction = 0usize;
        let mut steps = 0;
        let mut current_point = "AAA";

        loop {
            if current_point == "ZZZ" {
                break;
            }

            match instructions[next_instruction] {
                Instruction::Left => current_point = &map_points[current_point].0,
                Instruction::Right => current_point = &map_points[current_point].1,
            }

            next_instruction += 1;

            if next_instruction >= instructions.len() {
                next_instruction = 0;
            }

            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solution::part_one(INPUT), 6)
    }
}
