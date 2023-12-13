import { test } from "node:test";
import assert from "node:assert";

import { add } from "./lib";

test("add should work", () => {
    assert.equal(30, add(20, 10));
});

