package day11;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    // @formatter:off
    static final String INPUT = """
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
""";
// @formatter:on

    @Test
    void partOne() {
        assertEquals(374, Solution.partOne(INPUT));
    }

}
