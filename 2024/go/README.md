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
However, the locations are not listed by name, but by a unique number called the **location ID**.
The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.
By holding the two lists up **side by side** (your puzzle input), it quickly becomes clear that the lists aren't very similar.

```
3   4
4   3
2   5
1   3
3   9
3   3
```

### Part 1

Pair up the numbers and measure how far apart they are. Pair up the **smallest number in the left list** with the **smallest number in the right list**,
then the **second-smallest left number** with the **second-smallest right number**, and so on.
Witin each pair, figure out **how far apart** the two numbers are; you'll need to **add up all of those distances**.
For example, if you pair up a `3` from the left list with a `7` from the right list, the distance apart is `4`; if you pair up a `9` with a `3`, the distance apart is `6`.
For the sample input, the total distance is `11` (`2 + 1 + 0 + 1 + 2 + 5`).

### Part 2

Figure out how often each number from the left list appears in the right list and calculate a total similarity score.
Add up each number in the left list after multiplying it by the number of times that number appears in the right list.
For the sample input, the total similarity score is `31` (`9 + 4 + 0 + 0 + 9 + 9`).

## Day 2: Red-Nosed Reports

The first location is the Red-Nosed Reindeer nuclear fusion/fission plant, but there's no sign of the Chief Historian.
However, the Red-Nosed reactor is producing some unusal data and the engineers want your help with analyzing it.
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
