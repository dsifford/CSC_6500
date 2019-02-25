# Homework 5 (Due March 6, 2019)

## Problem 1. (40pts)

For every `x` in the language `{0,1}*`, let `val(x)` be the natural number for which `x` is a
binary expression; for completeness, let `val(e) = 0`.

### Part 1. (10pts)

Give a TM that implements a binary–to-unary conversion function `btu`, where `btu(x) = 1^val(x)`.

### Part 2. (10pts)

Let `utb` be a unary-to-binary conversion function (an inverse of the `btu`
function). Give a TM that implements `utb`.

### Part 3. (20pts)

Write a computer program (in C or its cousins or in Java to implement your two TMs). Test your
program by an input for `x = 57`.

## Problem 2 (24pts)

For each of the following Post correspondence system, either find a
solution or show that none exists.

<!-- NOTE: See here for more info: https://en.wikipedia.org/wiki/Post_correspondence_problem -->

### Part 1 (4pts)

| Tile 1 | Tile 2 | Tile 3 |
| ------ | ------ | ------ |
| a      | bb     | a      |
| aa     | b      | bb     |

### Part 2 (4pts)

| Tile 1 | Tile 2 | Tile 3 |
| ------ | ------ | ------ |
| a      | aab    | abaa   |
| aaa    | b      | ab     |

### Part 3 (4pts)

| Tile 1 | Tile 2 | Tile 3 | Tile 4 |
| ------ | ------ | ------ | ------ |
| ab     | ba     | b      | ba     |
| a      | bab    | aa     | ab     |

### Part 4 (4pts)

| Tile 1 | Tile 2 | Tile 3 |
| ------ | ------ | ------ |
| ab     | baa    | aba    |
| aba    | aa     | baa    |

### Part 5 (4pts)

| Tile 1 | Tile 2 | Tile 3 |
| ------ | ------ | ------ |
| aa     | bb     | abb    |
| aab    | ba     | b      |

### Part 6 (4pts)

| Tile 1 | Tile 2 | Tile 3 | Tile 4 |
| ------ | ------ | ------ | ------ |
| ab     | aa     | ab     | bb     |
| bb     | ba     | abb    | bab    |

## Problem 3. (20pts)

Using the computer program which has been placed on Canvas (PCP
problem), try the following Post Correspondence System problems to see
if a solution exists.

Indicate how many tiles were needed if you found the solution (you can also
give the solution if it is convenient by either writing the sequence of 0’s and
1’s or by the sequence of tiles used).

### Part 1. (10pts)

| Tile 1 | Tile 2 | Tile 3 |
| -----: | -----: | -----: |
|      0 |     01 |      1 |
|      1 |      0 |    101 |

### Part 2. (5pts)

| Tile 1 | Tile 2 | Tile 3 |
| -----: | -----: | -----: |
|     01 |      1 |    011 |
|      1 |    011 |      0 |

### Part 3. (5pts)

| Tile 1 | Tile 2 | Tile 3 |
| -----: | -----: | -----: |
|   1101 |   0110 |      1 |
|      1 |     11 |    110 |

## Problem 4. (16pts)

    times3 (x: positive integer) =
        while x != 1 do;
            if x is even
                then x = x/2
            else
                x = 3x + 1

            halt if x = power of 2

Write a program for the above pseudo code and test it for `x = 1, 2, ..., 100`. Then test it for `x = 23478`.
