import { readFileSync } from "fs";

import { solvePartOne } from "./lib";

let input: string = readFileSync("../input/Day04.txt", { encoding: "utf8" });
console.log(`Solution for part one: ${solvePartOne(input)}`);

