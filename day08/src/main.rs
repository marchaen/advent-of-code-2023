fn main() {
    let input = include_str!("../../input/Day08.txt");
    println!("{}", solution::part_one(input));
    println!("{}", solution::part_two(input));
}

mod solution {
    use std::{collections::HashMap, ops::Index};

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
    pub fn part_one(input: &str) -> u64 {
        let (instructions, points) = parse_input(input);
        count_steps_until(&instructions, &points, "AAA", |point| point == "ZZZ")
    }

    /// Implementation of the solution for the following problem (day 08 part two)
    ///
    /// # Problem description from aoc
    ///
    /// The sandstorm is upon you and you aren't any closer to escaping the
    /// wasteland. You had the camel follow the instructions, but you've barely
    /// left your starting position. It's going to take **significantly more
    /// steps** to escape!
    ///
    /// What if the map isn't for people - what if the map is for **ghosts**?
    /// Are ghosts even bound by the laws of spacetime? Only one way to find
    /// out.
    ///
    /// After examining the maps a bit longer, your attention is drawn to a
    /// curious fact: the number of nodes with names ending in `A` is equal to
    /// the number ending in `Z`! If you were a ghost, you'd probably just
    /// **start at every node that ends with `A`** and follow all of the paths
    /// at the same time until they all simultaneously end up at nodes that end
    /// with `Z`.
    ///
    /// For example:
    ///
    /// ```
    /// LR
    ///
    /// 11A = (11B, XXX)
    /// 11B = (XXX, 11Z)
    /// 11Z = (11B, XXX)
    /// 22A = (22B, XXX)
    /// 22B = (22C, 22C)
    /// 22C = (22Z, 22Z)
    /// 22Z = (22B, 22B)
    /// XXX = (XXX, XXX)
    /// ```
    ///
    /// Here, there are two starting nodes, `11A` and `22A` (because they both
    /// end with `A`). As you follow each left/right instruction, use that
    /// instruction to **simultaneously** navigate away from both nodes you're
    /// currently on. Repeat this process until **all** of the nodes you're
    /// currently on end with `Z`. (If only some of the nodes you're on end with
    /// `Z`, they act like any other node and you continue as normal.) In this
    /// example, you would proceed as follows:
    ///
    /// - Step 0: You are at `11A` and `22A`.
    /// - Step 1: You choose all of the **left** paths, leading you to `11B`
    /// and `22B`.
    /// - Step 2: You choose all of the **right** paths, leading you to
    /// **`11Z`** and `22C`.
    /// - Step 3: You choose all of the **left** paths, leading you to `11B` and
    /// **`22Z`**.
    /// - Step 4: You choose all of the **right** paths, leading you to
    /// **`11Z`** and `22B`.
    /// - Step 5: You choose all of the **left** paths, leading you to `11B` and
    /// `22C`.
    /// - Step 6: You choose all of the **right** paths, leading you to
    /// **`11Z`** and **`22Z`**.
    ///
    /// So, in this example, you end up entirely on nodes that end in `Z` after
    /// **`6`** steps.
    ///
    /// Simultaneously start on every node that ends with `A`. **How many steps
    /// does it take before you're only on nodes that end with `Z`?**
    pub fn part_two(input: &str) -> u64 {
        let (instructions, points) = parse_input(input);

        let minimum_steps = points
            .keys()
            .filter(|point| point.ends_with('A'))
            .map(|point| {
                count_steps_until(&instructions, &points, point, |point| point.ends_with('Z'))
            })
            .collect::<Vec<_>>();

        find_least_common_multiple(&minimum_steps)
    }

    fn count_steps_until<'point, M>(
        instructions: &[Instruction],
        points: &'point M,
        point: &'point str,
        should_stop: fn(&str) -> bool,
    ) -> u64
    where
        M: Index<&'point str, Output = (String, String)>,
    {
        let mut next_instruction = 0usize;
        let mut steps = 0;
        let mut current_point = point;

        loop {
            if should_stop(current_point) {
                break;
            }

            match instructions[next_instruction] {
                Instruction::Left => current_point = &points[current_point].0,
                Instruction::Right => current_point = &points[current_point].1,
            }

            next_instruction += 1;

            if next_instruction >= instructions.len() {
                next_instruction = 0;
            }

            steps += 1;
        }

        steps
    }

    fn greatest_common_divisor(mut left: u64, mut right: u64) -> u64 {
        while right != 0 {
            if left > right {
                std::mem::swap(&mut left, &mut right);
            }
            right %= left;
        }

        left
    }

    fn find_least_common_multiple(numbers: &[u64]) -> u64 {
        match numbers.len() {
            0 => panic!("Can't get least common multiple of empty slice."),
            1 => numbers[0],
            _ => {
                let left = numbers[0];
                let right = find_least_common_multiple(&numbers[1..]);

                left * right / greatest_common_divisor(left, right)
            }
        }
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

    #[test]
    fn part_two() {
        const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solution::part_two(INPUT), 6)
    }
}
