## Why Go?

## How to run code

Example

```sh
# run day 1 code
go run day01/day01.go
```

## AoC Puzzles Setting for AoC 2024

A group of senior historians has asked you to accompany them as they check the places they think the **Chief Historian** has most likely to visited, but that recently has gone missing.
As each location is visited, they will mark it on their list with a **star**. They figure that the Chief Historian **must** be in one of the first fifty places they'll look,
so in order to save Christmas, you need to help them get **fifty stars** on their list before Santa takes off on December 25th.
Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first.
Each puzzle grants **one star**.

## Day 1: Historian Hysteria

The list of locations to check is currently **empty** but there are notes and lists of historically significant locations left in the office of the **Chief Historian**.
The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.

The puzzle input is two lists **side by side** of locations.

```
3   4
4   3
2   5
1   3
3   9
3   3
```

### Part 1

Pair up the numbers and measure how far apart they are.
Pair up the **smallest number in the left list** with the **smallest number in the right list**, then the **second-smallest left number** with the **second-smallest right number**, and so on.
Within each pair, figure out **how far apart** the two numbers are; you'll need to **add up all of those distances**.

For example, if you pair up a `3` from the left list with a `7` from the right list, the distance apart is `4`; if you pair up a `9` with a `3`, the distance apart is `6`.
For the sample input, the total distance is `11` (`2 + 1 + 0 + 1 + 2 + 5`).

### Part 2

Figure out how often each number from the left list appears in the right list and calculate a total similarity score.
Add up each number in the left list after multiplying it by the number of times that number appears in the right list.
For the sample input, the total similarity score is `31` (`9 + 4 + 0 + 0 + 9 + 9`).

## Day 2: Red-Nosed Reports

The first location is the Red-Nosed Reindeer nuclear fusion/fission plant.
The Red-Nosed reactor is producing some unusal data and the engineers want your help with analyzing it.
The unusual data (your puzzle input) consists of **many reports**, one report per line.
Each report is a list of numbers called **levels** that are seperated by spaces.

```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

### Part 1

The engineers are trying to figure out which reports are **safe**.
The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing.
So, a report only counts as safe if both of the following are true:

- the levels are either **all increasing** or **all decreasing**
- any two adjacent levels differ by **at least one** and **at most three**

In the example, `2` reports (line 1 and 6) are safe.

### Part 2

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems **tolerate a single bad level** in what would otherwise be a safe report.
Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

In the example, `4` reports (line 1 and 6 are still safe, lines 4 and 5 can be corrected) are actually safe.

## Day 3: Mull It Over

The next stop is the North Pole Toboggan Rental Shop.
Their computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted.

```
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
```

### Part 1

It seems like the goal of the program is just to **multiply some numbers**.
It does that with instructions like `mul(X,Y)`, where `X` and `Y` are each 1-3 digit numbers.
For instance, `mul(44,46)` multiplies `44` by `46` to get a result of `2024`.

However, because the program's memory has been corrupted, there are also many invalid characters that should be **ignored**, even if theu look like part of a `mul` instruction.
Sequences like `mul(4*`, `mul(6,9)`, `?(12,34)`, or `mul ( 2 , 4 )` do **nothing**.

For the example, adding up the result of each valid instruction produces `161` (`2*4 + 5*5 + 11*8 + 8*5`).

### Part 2

To get an even more accurate result, you have to handle some of the uncorrupted conditional statements in the program.
There are two new instructions you'll need to handle:

- the `do()` instruction **enables** future `mul` instructions
- the `don't()` instruction **disables** future `mul` instructions

Only the **most recent** `do()` or `don't()` instruction applies. At the beginning of the program, `mul` instructions are **enabled**.

`xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`

For the example, `mul(5,5)` and `mul(11,8)` are **disabled** because there is a `don't()` instruction before them.

## Day 4: Ceres Search

The next stop is the Ceres monitoring station.
A small Elf wants you to help her with her **word search** (your puzzle input).

### Part 1

She only has to find one word: `XMAS`.
This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words.
It's a little unusual, though, as you don't merely need to find one instance of `XMAS` - you need to find **all of them**.

Example:

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

Gets solved to and includes `XMAS` a total of `18` times:

```
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
```

### Part 2

It's actually an `X-MAS` puzzle, not an `XMAS` puzzle.
You have to find two `MAS` in the shape of an `X`.
One way to achieve this is like this:

```
M.S
.A.
M.S
```

Here's the same example from above, but this time all the `X-MAS`es have been kept instead.

```
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
```

In this example, an `X-MAS` appears `9` times.

## Day 5: Print Queue

Next stop is at the North Pole printing department.
An Elf explains to you that the new **sleigh launch safety manual** updates won't print correctly.
Safety protocols dictate that new pages for the safety manuals must be printed in a **very specific order**.
The notation `X|Y` means that if both page number `X` and page number `Y` are to be produced as part of an update, page number `X` **must** be printed at some point before page number `Y`.

The Elf has both the **page ordering rules** and the **pages to produce in each update** (your puzzle input), but can't figure out whether each update has the pages in the right order.

For example:

```
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
```

The first section specifies the **page ordering rules**, one per line.
The first rule, `47|53`, means that if an update includes both page number `47` and page number `53`, then page number `47` must be printed at some point before page number `53`.
(`47` doesn't necessarily have to be **immediately** before `53`; other pages are allowed to be between them)

The second section specifies the page numbers of each **update**.
Becaue most safety manuals are different, the pages needed in the updates are different too.
The first update, `75,47,61,53,29`, means that the update consists of page numbers `75`, `47`, `61`, `53`, and `29`.

### Part 1

To get the printers going as soon as possible, start by identifying **which updates are already in the right order**.

For some reason. the Elves need to know the **middle page number** of each update being printed.
Because you are currently only printing the correctly-ordered updates, you will need to find the middle page number of each correctly-ordered update.

For the example above:

```
75,47,61,53,29
97,61,53,29,13
75,29,13
```

These have middle pages of `61`, `53`, and `29` respectively. Adding these numbers together gives `143`.

Determine which updates are already in the correct order.
**What do you get if you add up the middle page number from those correctly-ordered udates?**

### Part 2

For each of the **incorrectly-ordered updates**, use the page ordering rules to put the page numbers in the right order.

For the example above, here are the three incorrectly-ordered updates and how they should be reordered:

- `75,97,47,61,53` becomes `97,75,47,61,53`
- `61,13,29` becomes `61,29,13`
- `97,13,75,29,47` becomes `97,75,47,29,13`

After taking **only the incorrectly-ordered updates** and ordering them correctly, their middle page numbers are `47`, `29`, and `47`.
**What do you get if you add up the middle numbers after correctly ordering just those updates?**

## Day 6: Guard Gallivant

The next stop is the North Pole prototype suit manfucaturing lab in the year 1518.
You have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while the Historians search for the Chief.
Unfortunately, a single **guard** is patrolling this part of the lab.

Maybe you can predict where the guard will go ahead of time so that The Historians can search safely?
You start by making a map (your puzzle input) of the situation.

For example:

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

The map shows the current position of the guard with `^` (to indicate the guard is facing up from the perspective of the map).
Any obstructions - crates, desks, alchemical reactors, etc. - are shown as `#`.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

- if there is something directly in front of you, turn right 90 degrees
- otherwise, take a step forward

### Part 1

By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path.
**Including the guard's starting position**, the positions visited by the guard before leaving the are marked with an `X`:

```
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
```

In this example, the guard will visit `41` distinct positions on your map.

Predict the path of the guard. **How many distinct positions will the guard visit beofre leaving the mapped area?**

### Part 2

The Historians explain that the guard's patrol area is simply too large for them to safely search the lab to search.
Fortunately, they are **pretty sure** that adding a single new obstruction **won't** cause a time paradox.
They'd like to place the new obstruction in such a way that the guard will get **stuck in a loop**, making the rest of the lab safe to search.
To have the lowest chance of creating a time paradox, The Historians would like to know **all** of the possible positions for such an obstruction.
The new obstruction can't be placed at the guard's starting position - the guard is there right now and would notice.

In the example above, there are only `6` different positions where a new obstruction would cause the guard to get stuck in a loop.

You need to get the guard stuck in a loop by adding a single new obstruction. **How many different positions could you choose for this obstruction?**

## Day 7: Bridge Repair

There's a bridge that needs to be prepared.
The puzzle input is a calibration manual where the operators have been taken out of the equations.

Each line represents a single equation.
The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.

Operators are **always evaluated left-to-right**, **not** according to precedence rules.
Furthermore, numbers in the equations cannot be rearranged.

Example:

```
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

## Part 1

There are two operators, **add** (`+`) and **multiply** (`*`).

In the example above, only three of the above equations can be made true by inserting operators:

- `190: 10 19`
- `3267: 81 40 27`
- `292: 11 6 16 20`

You need to calculate the **total calibration result**, which is the sum of the test values from just the equations that could possibly be true.
In the example above, the sum of the test values for the three equations listed is `3749`.

**What is the total calibration result?**

## Part 2

Now, there is a **third type of operator**.
The concatenation operator (`||`) combines the digits from its left and right inputs into a single number.
For example, `12 || 345` would become `12345`.

Three additional equations become possible with this new operator in the example:

- `156: 15 6`
- `7290: 6 8 6 15`
- `192: 17 8 14`

With this new operator, adding up all test values (including the previously valid ones!) produces a total calibration result of `11387`.

**What is the total calibration result now?**

## Day 8: Resonant Collinearity

The next stop is on the roof of a top-secret Easter Bunny installation.
There are a lot of antennas that are reconfigured to create an nefarious effect.
The puzzle input is a map of these **antennas**.
Each antenna is tuned to a specific **frequency** indicated by a single lowercase letter, uppercase letter, or digit.

Example:

```
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
```

The signal only applies its effect at specific **antinodes** based on the resonant frequencies of the antennas.

### Part 1

An antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but **only when one of the antenna** is **twice as far away** as the other.
This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

Antennas with different frequencies don't create antinodes; `A` and `a` count as different frequencies.
However, antinodes **can** occur at locations that contain antennas.

The example has antennas with two different frequencies, and there are a total of `14` unique locations that contain an antinode.

Calculate the impact of the signal.
**How many unique locations within the bounds of the map contain an antinode?**

### Part 2

Now an antinode occors **at any grid position** exactly in line with at least two antennas of the same frequency, regardless of distance.

The example now has `34` antinodes, including the antinodes that appear on every antenna.

Calculate the impact of the signal using the updated model.
**How many unique locations within the bounds of the map contain an antinode?**
