const PipeLiteral = {
    Vertical: "|",
    Horizontal: "-",
    TopToRight: "L",
    TopToLeft: "J",
    BottomToLeft: "7",
    BottomToRight: "F",
} as const;

type PipeLiteral = typeof PipeLiteral[keyof typeof PipeLiteral];

const Tile = {
    Animal: "S",
    Ground: ".",
    ...PipeLiteral
} as const;

type Tile = typeof Tile[keyof typeof Tile];

const Direction = ["North", "East", "South", "West"] as const;
type Direction = typeof Direction[number];

type Connections = {
    [key in PipeLiteral]: Direction[];
}

const Connections: Connections = {
    [PipeLiteral.Vertical]: ["North", "South"],
    [PipeLiteral.Horizontal]: ["East", "West"],
    [PipeLiteral.TopToRight]: ["North", "East"],
    [PipeLiteral.TopToLeft]: ["North", "West"],
    [PipeLiteral.BottomToLeft]: ["West", "South"],
    [PipeLiteral.BottomToRight]: ["East", "South"]
};

type Grid = Tile[][];
type Loop = Pipe[];

type Coordinate = {
    row: number,
    column: number
}

type Steps = {
    [key in Direction]: Coordinate;
}

const Steps: Steps = {
    "North": { row: -1, column: 0 },
    "East": { row: 0, column: 1 },
    "South": { row: 1, column: 0 },
    "West": { row: 0, column: -1 }
};

type Neighbor = { tile: Tile, direction: Direction, coordinate: Coordinate };
type MaybeNeighbor = Neighbor | null;

type ConnectedPipe = { pipe: PipeLiteral, from: Direction, coordinate: Coordinate };

type Pipe = {
    previous: ConnectedPipe,
    next: ConnectedPipe,
    value: PipeLiteral
    coordinate: Coordinate,
    hasAnimal: boolean,
}

function isTile(raw: string): raw is Tile {
    return Object.values(Tile).includes(raw as Tile);
}

function isPipe(raw: string): raw is PipeLiteral {
    return Object.values(PipeLiteral).includes(raw as PipeLiteral);
}

function equalCoordinates(left: Coordinate, right: Coordinate): boolean {
    return left.row === right.row && left.column == right.column;
}

function invertDirection(direction: Direction): Direction {
    return Direction[(Direction.indexOf(direction) + 2) % 4];
}

function connectsTo(pipe: PipeLiteral, direction: Direction): boolean {
    return Connections[pipe].includes(direction);
}

function toConnectedPipe(neighbor: Neighbor, replacementPipe: PipeLiteral | null): ConnectedPipe {
    let pipe;

    if (!isPipe(neighbor.tile)) {
        if (replacementPipe === null)
            throw new Error("No replacement pipe provided for non pipe neighbor.");
        pipe = replacementPipe as PipeLiteral;
    } else {
        pipe = neighbor.tile;
    }

    return { pipe, from: neighbor.direction, coordinate: neighbor.coordinate };
}

function parseInput(input: string): Grid {
    let grid: Tile[][] = [];

    input.split("\n").forEach((line) => {
        if (Array.from(line).some((tile) => !isTile(tile)))
            throw new Error("The grid contains invalid tile values.");

        grid.push([...line as unknown as Tile[]]);
    });

    return grid;
}

function findAnimalCoordinate(grid: Grid): Coordinate {
    const row = grid.findIndex((row) => row.includes(Tile.Animal));
    return {
        row: row,
        column: grid[row].findIndex((tile) => tile == Tile.Animal)
    };

}

/**
 * Search through the directions which each pipe can connect to and compare
 * them to the neighbors of the animal to find the exact pipe which connects
 * to the two neighbors.
 */
function findPipeOfAnimal(next: Neighbor, previous: Neighbor): PipeLiteral {
    const directions = [invertDirection(next.direction), invertDirection(previous.direction)];
    return Object.keys(Connections).find(
        (key) => Connections[key as PipeLiteral]
            .every(direction => directions.includes(direction))
    ) as PipeLiteral;
}

function getNeighbors(grid: Grid, tile: Coordinate): Neighbor[] {
    const { row, column } = tile;
    let neighbors: Neighbor[] = [];

    if (grid.length > row + 1)
        neighbors.push({ tile: grid[row + 1][column], direction: "North", coordinate: { row: row + 1, column: column } });

    if (column > 0)
        neighbors.push({ tile: grid[row][column - 1], direction: "East", coordinate: { row: row, column: column - 1 } });

    if (row > 0)
        neighbors.push({ tile: grid[row - 1][column], direction: "South", coordinate: { row: row - 1, column: column } });

    if (grid[row].length > column + 1)
        neighbors.push({ tile: grid[row][column + 1], direction: "West", coordinate: { row: row, column: column + 1 } });

    return neighbors;
}

function findConnectedPipes(tile: Tile, neighbors: Neighbor[]): { first: MaybeNeighbor, second: MaybeNeighbor } {
    let first: MaybeNeighbor = null;
    let second: MaybeNeighbor = null;

    neighbors.forEach((neighbor) => {
        // The animal is always a valid neighbor when searching for the loop
        // Otherwise make sure the pipe can connect towards the current tile
        if (neighbor.tile === Tile.Animal ||
            (isPipe(neighbor.tile) && connectsTo(neighbor.tile, neighbor.direction))) {

            // Make sure that the tile itself also can connect to the neighbor
            if (isPipe(tile) && !connectsTo(tile, invertDirection(neighbor.direction)))
                return;

            // The neighbor is indeed connected to the current tile and can be
            // set as the first or second connected pipe.
            if (first == null) {
                first = neighbor;
                return;
            }

            second = neighbor;
        }
    });

    return { first, second: second };
}

function findLoop(grid: Grid): Loop {
    const animalCoordinate = findAnimalCoordinate(grid);

    const { first: animalNext, second: animalPrevious } = findConnectedPipes(
        Tile.Animal, getNeighbors(grid, animalCoordinate)
    );

    if (animalNext == null || animalPrevious == null)
        throw new Error("The animal is not connected to two pipes");

    const animalPipe: Pipe = {
        next: toConnectedPipe(animalNext, null),
        previous: toConnectedPipe(animalPrevious, null),
        value: findPipeOfAnimal(animalNext, animalPrevious),
        coordinate: animalCoordinate,
        hasAnimal: true,
    };

    let loop = [animalPipe];
    let previousPipe = animalPipe;

    while (true) {
        const neighbors = getNeighbors(grid, previousPipe.next.coordinate);
        const { first, second } = findConnectedPipes(previousPipe.next.pipe, neighbors);

        if (first == null || second == null)
            throw new Error("There is no loop that is closed in istself in the grid.");

        let previousNeighbor;
        let nextNeighbor;

        if (equalCoordinates(previousPipe.coordinate, first.coordinate)) {
            previousNeighbor = first;
            nextNeighbor = second;
        } else {
            previousNeighbor = second;
            nextNeighbor = first;
        }

        let currentPipe: Pipe = {
            next: toConnectedPipe(nextNeighbor, animalPipe.value),
            previous: toConnectedPipe(previousNeighbor, animalPipe.value),
            value: previousPipe.next.pipe,
            coordinate: previousPipe.next.coordinate,
            hasAnimal: false,
        };

        loop.push(currentPipe);

        if (equalCoordinates(currentPipe.next.coordinate, animalPipe.coordinate)) {
            break;
        }

        previousPipe = currentPipe;
    }

    return loop;
}

function isInBounds(grid: Grid, coordinate: Coordinate) {
    return coordinate.row >= 0 && coordinate.row < grid.length &&
        coordinate.column >= 0 && coordinate.column < grid[0].length;
}

function countPipes(grid: Grid, walkingDirection: Direction, until: Coordinate): number {
    const pipeToCount = ["North", "South"].includes(walkingDirection) ?
        PipeLiteral.Horizontal : PipeLiteral.Vertical;

    const startingPosition = structuredClone(until);
    const invertedDirection = invertDirection(walkingDirection);

    while (isInBounds(grid, startingPosition)) {
        startingPosition.row += Steps[invertedDirection].row;
        startingPosition.column += Steps[invertedDirection].column;
    }

    // The loop goes until the starting position is invalid so removing the
    // last step is needed.
    startingPosition.row -= Steps[invertedDirection].row;
    startingPosition.column -= Steps[invertedDirection].column;

    let pipes = 0;
    const currentPosition = startingPosition;

    while (!equalCoordinates(currentPosition, until)) {
        if (grid[currentPosition.row][currentPosition.column] === pipeToCount)
            pipes++;

        currentPosition.row += Steps[walkingDirection].row;
        currentPosition.column += Steps[walkingDirection].column;
    }

    return pipes;
}

function isInsideLoop(grid: Grid, tile: Coordinate): boolean {
    const neighbors = getNeighbors(grid, tile);

    if (neighbors.length != 4)
        return false;

    return neighbors.every((neighbor) => {
        return countPipes(grid, neighbor.direction, tile) % 2 !== 0;
    });
}

/*
 * Scanning the area, you discover that the entire field you're standing on is
 * densely packed with pipes; it was hard to tell at first because they're the
 * same metallic silver color as the "ground". You make a quick sketch of all
 * of the surface pipes you can see (your puzzle input).
 *
 * The pipes are arranged in a two-dimensional grid of **tiles**:
 *
 * - `|` is a **vertical pipe** connecting north and south.
 * - `-` is a **horizontal pipe** connecting east and west.
 * - `L` is a **90-degree bend** connecting north and east.
 * - `J` is a **90-degree bend** connecting north and west.
 * - `7` is a **90-degree bend** connecting south and west.
 * - `F` is a **90-degree bend** connecting south and east.
 * - `.` is **ground**; there is no pipe in this tile.
 * - `S` is the **starting position** of the animal; there is a pipe on this
 *   tile, but your sketch doesn't show what shape the pipe has.
 *
 * Based on the acoustics of the animal's scurrying, you're confident the pipe
 * that contains the animal is **one large, continuous loop**.
 *
 * For example, here is a square loop of pipe:
 *
 * ```
 * .....
 * .F-7.
 * .|.|.
 * .L-J.
 * .....
 * ```
 *
 * If the animal had entered this loop in the northwest corner, the sketch would
 * instead look like this:
 *
 * ```
 * .....
 * .S-7.
 * .|.|.
 * .L-J.
 * .....
 * ```
 *
 * In the above diagram, the `S` tile is still a 90-degree `F` bend: you can
 * tell because of how the adjacent pipes connect to it.
 * 
 * Unfortunately, there are also many pipes that **aren't connected to the
 * loop**! This sketch shows the same loop as above:
 *
 * ```
 * -L|F7
 * 7S-7|
 * L|7||
 * -L-J|
 * L|-JF
 * ```
 *
 * In the above diagram, you can still figure out which pipes form the main
 * loop: they're the ones connected to `S`, pipes those pipes connect to, pipes
 * **those** pipes connect to, and so on. Every pipe in the main loop connects
 * to its two neighbors (including `S`, which will have exactly two pipes
 * connecting to it, and which is assumed to connect back to those two pipes).
 *
 * Here is a sketch that contains a slightly more complex main loop:
 *
 * ``` 
 * ..F7.
 * .FJ|.
 * SJ.L7
 * |F--J
 * LJ...
 * ```
 *
 * Here's the same example sketch with the extra, non-main-loop pipe tiles also
 * shown:
 *
 * ```
 * 7-F7-
 * .FJ|7
 * SJLL7
 * |F--J
 * LJ.LJ
 * ```
 *
 * If you want to **get out ahead of the animal**, you should find the tile in
 * the loop that is **farthest** from the starting position. Because the animal
 * is in the pipe, it doesn't make sense to measure this by direct distance.
 * Instead, you need to find the tile that would take the longest number of
 * steps **along the loop** to reach from the starting point - regardless of
 * which way around the loop the animal went.
 *
 * In the first example with the square loop:
 *
 * ```
 * .....
 * .S-7.
 * .|.|.
 * .L-J.
 * .....
 * ```
 *
 * You can count the distance each tile in the loop is from the starting point
 * like this:
 *
 * ```
 * .....
 * .012.
 * .1.3.
 * .234.
 * .....
 * ```
 *
 * In this example, the farthest point from the start is `4` steps away.
 *
 * Here's the more complex loop again:
 *
 * ```
 * ..F7.
 * .FJ|.
 * SJ.L7
 * |F--J
 * LJ...
 * ```
 *
 * Here are the distances for each tile on that loop:
 *
 * ```
 * ..45.
 * .236.
 * 01.78
 * 14567
 * 23...
 * ```
 *
 * Find the single giant loop starting at `S`. **How many steps along the loop
 * does it take to get from the starting position to the point farthest from the
 * starting position?**
*/
export function solvePartOne(input: string): number {
    return findLoop(parseInput(input)).length / 2;
}

/*
 * You quickly reach the farthest point of the loop, but the animal never
 * emerges. Maybe its nest is **within the area enclosed by the loop**?
 *
 * To determine whether it's even worth taking the time to search for such a
 * nest, you should calculate how many tiles are contained within the loop. For
 * example:
 *
 * ```
 * ...........
 * .S-------7.
 * .|F-----7|.
 * .||.....||.
 * .||.....||.
 * .|L-7.F-J|.
 * .|..|.|..|.
 * .L--J.L--J.
 * ...........
 * ```
 *
 * The above loop encloses merely **four tiles** - the two pairs of `.` in the
 * southwest and southeast (marked `I` below). The middle `.` tiles (marked `O`
 * below) are **not** in the loop. Here is the same loop again with those
 * regions marked:
 *
 * ```
 * ...........
 * .S-------7.
 * .|F-----7|.
 * .||OOOOO||.
 * .||OOOOO||.
 * .|L-7OF-J|.
 * .|II|O|II|.
 * .L--JOL--J.
 * .....O.....
 * ```
 *
 * In fact, there doesn't even need to be a full tile path to the outside for
 * tiles to count as outside the loop - squeezing between pipes is also
 * allowed! Here, `I` is still within the loop and `O` is still outside the
 * loop:
 *
 * ```
 * ..........
 * .S------7.
 * .|F----7|.
 * .||OOOO||.
 * .||OOOO||.
 * .|L-7F-J|.
 * .|II||II|.
 * .L--JL--J.
 * ..........
 * ```
 *
 * In both of the above examples, **`4`** tiles are enclosed by the loop.
 *
 * Here's a larger example:
 *
 * ```
 * .F----7F7F7F7F-7....
 * .|F--7||||||||FJ....
 * .||.FJ||||||||L7....
 * FJL7L7LJLJ||LJ.L-7..
 * L--J.L7...LJS7F-7L7.
 * ....F-J..F7FJ|L7L7L7
 * ....L7.F7||L7|.L7L7|
 * .....|FJLJ|FJ|F7|.LJ
 * ....FJL-7.||.||||...
 * ....L---J.LJ.LJLJ...
 * ```
 *
 * The above sketch has many random bits of ground, some of which are in the
 * loop (`I`) and some of which are outside it (`O`):
 *
 * ```
 * OF----7F7F7F7F-7OOOO
 * O|F--7||||||||FJOOOO
 * O||OFJ||||||||L7OOOO
 * FJL7L7LJLJ||LJIL-7OO
 * L--JOL7IIILJS7F-7L7O
 * OOOOF-JIIF7FJ|L7L7L7
 * OOOOL7IF7||L7|IL7L7|
 * OOOOO|FJLJ|FJ|F7|OLJ
 * OOOOFJL-7O||O||||OOO
 * OOOOL---JOLJOLJLJOOO
 * ```
 *
 * In this larger example, **`8`** tiles are enclosed by the loop.
 *
 * Any tile that isn't part of the main loop can count as being enclosed by the
 * loop. Here's another example with many bits of junk pipe lying around that
 * aren't connected to the main loop at all:
 *
 * ```
 * FF7FSF7F7F7F7F7F---7
 * L|LJ||||||||||||F--J
 * FL-7LJLJ||||||LJL-77
 * F--JF--7||LJLJ7F7FJ-
 * L---JF-JLJ.||-FJLJJ7
 * |F|F-JF---7F7-L7L|7|
 * |FFJF7L7F-JF7|JL---7
 * 7-L-JL7||F7|L7F-7F7|
 * L.L7LFJ|||||FJL7||LJ
 * L7JLJL-JLJLJL--JLJ.L
 * ```
 *
 * Here are just the tiles that are **enclosed by the loop** marked with `I`:
 *
 * ```
 * FF7FSF7F7F7F7F7F---7
 * L|LJ||||||||||||F--J
 * FL-7LJLJ||||||LJL-77
 * F--JF--7||LJLJIF7FJ-
 * L---JF-JLJIIIIFJLJJ7
 * |F|F-JF---7IIIL7L|7|
 * |FFJF7L7F-JF7IIL---7
 * 7-L-JL7||F7|L7F-7F7|
 * L.L7LFJ|||||FJL7||LJ
 * L7JLJL-JLJLJL--JLJ.L
 * ```
 *
 * In this last example, **`10`** tiles are enclosed by the loop.
 *
 * Figure out whether you have time to search for the nest by calculating the
 * area within the loop. **How many tiles are enclosed by the loop?**
*/
export function solvePartTwo(input: string): number {
    const grid = parseInput(input);
    const loop = findLoop(grid);

    const loopOnly: Grid = grid.map((row) => Array.from(Tile.Ground.repeat(row.length)) as Tile[]);

    loop.forEach((pipe) => {
        loopOnly[pipe.coordinate.row][pipe.coordinate.column] = pipe.value;
    })

    const loopAreaGrid = grid.map((row) => Array.from(Tile.Ground.repeat(row.length)));

    loop.forEach((pipe) => {
        loopAreaGrid[pipe.coordinate.row][pipe.coordinate.column] = "X";
    })

    let insideCount = 0;

    for (let row = 0; row < grid.length; row++) {
        for (let column = 0; column < grid[0].length; column++) {
            if (loopAreaGrid[row][column] === "X")
                continue;

            if (isInsideLoop(loopOnly, { row, column }))
                insideCount++;
        }
    }

    return insideCount;
}
