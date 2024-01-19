import { test } from "node:test";
import assert from "node:assert";

import { solvePartOne } from "./lib";

const input = `..F7.
.FJ|.
SJ.L7
|F--J
LJ...`;

test("part one", () => {
    assert.equal(solvePartOne(input), 8);
});

