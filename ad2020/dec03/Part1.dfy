/*
// --- Day 3: Toboggan Trajectory ---

// With the toboggan login problems resolved, you set off toward the
// airport. While travel by toboggan might be easy, it's certainly not safe:
// there's very minimal steering and the area is covered in trees. You'll need
// to see which angles will take you near the fewest trees.
//
// Due to the local geology, trees in this area only grow on exact integer
// coordinates in a grid. You make a map (your puzzle input) of the open
// squares (.) and trees (#) you can see. For example:
//
//     ..##.......
//     #...#...#..
//     .#....#..#.
//     ..#.#...#.#
//     .#...##..#.
//     ..#.##.....
//     .#.#.#....#
//     .#........#
//     #.##...#...
//     #...##....#
//     .#..#...#.#
//
// These aren't the only trees, though; due to something you read about once
// involving arboreal genetics and biome stability, the same pattern repeats to
// the right many times:
//
//     ..##.........##.........##.........##.........##.........##.......  --->
//     #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//     .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//     ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//     .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//     ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//     .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//     .#........#.#........#.#........#.#........#.#........#.#........#
//     #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//     #...##....##...##....##...##....##...##....##...##....##...##....#
//     .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
// 
// You start on the open square (.) in the top-left corner and need to reach
// the bottom (below the bottom-most row on your map).
// 
// The toboggan can only follow a few specific slopes (you opted for a cheaper
// model that prefers rational numbers); start by counting all the trees you
// would encounter for the slope right 3, down 1:
// 
// From your starting position at the top-left, check the position that is
// right 3 and down 1. Then, check the position that is right 3 and down 1 from
// there, and so on until you go past the bottom of the map.
// 
// The locations you'd check in the above example are marked here with O where
// there was an open square and X where there was a tree:
//
//     ..##.........##.........##.........##.........##.........##.......  --->
//     #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//     .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//     ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//     .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//     ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//     .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//     .#........#.#........X.#........#.#........#.#........#.#........#
//     #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//     #...##....##...##....##...#X....##...##....##...##....##...##....#
//     .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
// 
// In this example, traversing the map using this slope would cause you to
// encounter 7 trees.
// 
// Starting at the top-left corner of your map and following a slope of right 3
// and down 1, how many trees would you encounter?


// --- Specification

function countIf<T>(pred: T -> bool, elems: seq<T>): nat {
    |(set i | 0 <= i < |elems| && pred(elems[i]))|
}

predicate hasWidth<T>(grid: seq<seq<T>>, width: nat)
{
    forall i :: 0 <= i < |grid| ==> |grid[i]| == width
}

function countTrees(grid: seq<seq<bool>>, width: nat, invslope: nat): nat
    requires width > 0
    requires hasWidth(grid, width)
{
    if |grid| == 0
        then 0
    else var last := |grid| - 1;
        var width := |grid[0]|;
        countTrees(grid[..last], width, invslope) +
            if grid[last][invslope * last % width] then 1 else 0
}

// --- Implementation

// "In contrast to one-dimensional arrays, there is no operation to convert
// stretches of elements from a multi-dimensional array to a sequence."
// --Dafny Reference Manual

function contentsOfRowUpTo(grid: array2<bool>, i: nat, len: nat): (result: seq<bool>)
    reads grid
    requires 0 <= i < grid.Length0
    requires 0 <= len <= grid.Length1
    ensures |result| == len
    ensures forall j :: 0 <= j < len ==> result[j] == grid[i, j]
{
    if len == 0
        then []
    else contentsOfRowUpTo(grid, i, len - 1) + [grid[i, len - 1]]
}


function contentsOfRow(grid: array2<bool>, i: nat): (result: seq<bool>)
    reads grid
    requires 0 <= i < grid.Length0
    ensures |result| == grid.Length1
    ensures forall j :: 0 <= j < grid.Length1 ==> result[j] == grid[i, j]
{
    contentsOfRowUpTo(grid, i, grid.Length1)
}

function contentsUpTo(grid: array2<bool>, rows: nat): (result: seq<seq<bool>>)
    reads grid
    requires 0 <= rows <= grid.Length0
    ensures |result| == rows
    ensures hasWidth(result, grid.Length1)
    ensures forall i, j :: 0 <= i < rows && 0 <= j < grid.Length1 ==> result[i][j] == grid[i, j]
{
    if rows == 0
        then []
    else contentsUpTo(grid, rows - 1) + [contentsOfRow(grid, rows - 1)]
}

function contents(grid: array2<bool>): (result: seq<seq<bool>>)
    reads grid
    ensures |result| == grid.Length0
    ensures hasWidth(result, grid.Length1)
    ensures forall i, j :: 0 <= i < grid.Length0 && 0 <= j < grid.Length1 ==> result[i][j] == grid[i, j]
{
    contentsUpTo(grid, grid.Length0)
}
*/

lemma mulAddModLemma1(a: nat, b: nat, d: nat)
    requires d > 0
    ensures (a * d + b) % d == b % d
{
    if a == 0 {
        assert a * d + b == b;
        assert (a * d + b) % d == b % d;
    } else {
        mulAddModLemma1(a - 1, b, d);
        assert ((a - 1) * d + b) % d == b % d;
        assert (a * d + b) % d == b % d;
    }
}
/*
lemma mulAddModLemma(a: nat, b: nat, modulus: nat)
    requires modulus > 0
    ensures (a + b) % modulus == (a % modulus + b) % modulus
{
    var qa, ra :| qa * modulus + ra == a;
    assert a % modulus == ra;
    var qb, rb :| qb * modulus + rb == b;
    assert b % modulus == rb;

    calc {
        (a + b) % modulus;
        == ((qa * modulus + ra) + (qb * modulus + rb)) % modulus;
        == ((qa + qb) * modulus + (ra + rb)) % modulus;
        == (ra + rb) % modulus;
        == (qb * modulus + (ra + rb)) % modulus;
        == (a % modulus + b) % modulus;
    }
}

lemma mulModStep(nsteps: nat, stepsize: nat, modulus: nat, result: nat)
    requires modulus > 0
    ensures (nsteps + 1) * stepsize % modulus == ((nsteps * stepsize % modulus) + stepsize) % modulus
{
    calc {
        (nsteps + 1) * stepsize % modulus;
        ==
        (nsteps * stepsize + stepsize) % modulus;
        == { mulAddModLemma(nsteps * stepsize, stepsize, modulus); }
        ((nsteps * stepsize % modulus) + stepsize) % modulus;
    }
}

/*

method CountTrees(grid: array2<bool>, invslope: nat)
    returns (count: nat)
    requires grid.Length1 > 0
    ensures count == countTrees(contents(grid), grid.Length1, invslope)
{
    ghost var grid_seq := contents(grid);
    count := 0;

    var x := 0;
    var y := 0;
    while y < grid.Length0
        invariant 0 <= y <= grid.Length0
        invariant x == y * invslope % grid.Length1
        invariant count == countTrees(grid_seq[..y], grid.Length1, invslope)
    {
        assert grid[y, x] == grid_seq[y][x];
        if grid[y, x] {
            count := count + 1;
        }

        x := (x + invslope) % grid.Length1;
        y := y + 1;
    }

    assert grid_seq[..y] == grid_seq;
}
*/
   
