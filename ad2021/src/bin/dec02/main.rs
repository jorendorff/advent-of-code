/*!
--- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down
2, or up 3:

*   `forward X` increases the horizontal position by X units.
*   `down X` increases the depth by X units.
*   `up X` decreases the depth by X units.

Note that since you're on a submarine, down and up affect your **depth**, and
so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You
should probably figure out where it's going. For example:

```
forward 5
down 5
forward 8
up 3
down 8
forward 2
```

Your horizontal position and depth both start at 0. The steps above would then
modify them as follows:

*   `forward 5` adds 5 to your horizontal position, a total of 5.
*   `down 5` adds 5 to your depth, resulting in a value of 5.
*   `forward 8` adds 8 to your horizontal position, a total of 13.
*   `up 3` decreases your depth by 3, resulting in a value of 2.
*   `down 8` adds 8 to your depth, resulting in a value of 10.
*   `forward 2` adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal position of 15
and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the
planned course. What do you get if you multiply your final horizontal position
by your final depth?

--- Part Two ---

Based on your calculations, the planned course doesn't seem to make any sense.
You find the submarine manual and discover that the process is actually
slightly more complicated.

In addition to horizontal position and depth, you'll also need to track a third
value, aim, which also starts at 0. The commands also mean something entirely
different than you first thought:

    down X increases your aim by X units.
    up X decreases your aim by X units.
    forward X does two things:
        It increases your horizontal position by X units.
        It increases your depth by your aim multiplied by X.

Again note that since you're on a submarine, down and up do the opposite of
what you might expect: "down" means aiming in the positive direction.

Now, the above example does something different:

*   `forward 5` adds 5 to your horizontal position, a total of 5. Because your
    aim is 0, your depth does not change.

*   `down 5` adds 5 to your aim, resulting in a value of 5.

*   `forward 8` adds 8 to your horizontal position, a total of 13. Because your
    aim is 5, your depth increases by 8*5=40.

*   `up 3` decreases your aim by 3, resulting in a value of 2.

*   `down 8` adds 8 to your aim, resulting in a value of 10.

*   `forward 2` adds 2 to your horizontal position, a total of 15. Because your
    aim is 10, your depth increases by 2*10=20 to a total of 60.

After following these new instructions, you would have a horizontal position of
15 and a depth of 60. (Multiplying these produces 900.)

Using this new interpretation of the commands, calculate the horizontal
position and depth you would have after following the planned course. What do
you get if you multiply your final horizontal position by your final depth?

*/

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

use Command::*;

fn parse_input(text: &str) -> anyhow::Result<Vec<Command>> {
    text.lines()
        .map(|line| {
            let fields = line.split_whitespace().collect::<Vec<&str>>();
            anyhow::ensure!(fields.len() == 2, "each line should have two fields on it");
            let arg = fields[1].parse::<u64>()?;
            Ok(match fields[0] {
                "forward" => Forward(arg),
                "down" => Down(arg),
                "up" => Up(arg),
                other => anyhow::bail!("unrecognized command {:?}", other),
            })
        })
        .collect()
}

fn part_1(commands: &[Command]) -> u64 {
    let (x, d) = commands.iter().fold((0, 0), |(x, d), cmd| match cmd {
        Forward(n) => (x + n, d),
        Down(n) => (x, d + n),
        Up(n) => (x, d - n),
    });
    x * d
}

fn part_2(commands: &[Command]) -> u64 {
    let (x, d, _a) = commands.iter().fold((0, 0, 0), |(x, d, a), cmd| match cmd {
        Forward(n) => (x + n, d + a * n, a),
        Down(n) => (x, d, a + n),
        Up(n) => (x, d, a - n),
    });
    x * d
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 900);
    }
}

fn main() -> anyhow::Result<()> {
    let data = parse_input(include_str!("input.txt"))?;
    println!("Part 1: {:?}", part_1(&data));
    println!("Part 2: {:?}", part_2(&data));
    Ok(())
}
