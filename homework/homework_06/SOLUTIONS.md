Derek Sifford

# Homework 6

## Problem 1

> **Note:** code for this problem can be found here: https://github.com/dsifford/CSC_6500/tree/master/homework/homework_06/processor-communication

The assignment with the lowest cost is simply placing every module in processor 2.

### Program Output

```sh
Brute force:
  Lowest cost = 38
  Best combination = [2, 2, 2, 2, 2]
Monte carlo (10 iterations):
  Lowest cost = 85
  Best combination = [0, 0, 2, 3, 2]
Monte carlo (100 iterations):
  Lowest cost = 47
  Best combination = [3, 3, 3, 3, 3]
Monte carlo (1000 iterations):
  Lowest cost = 38
  Best combination = [2, 2, 2, 2, 2]
Monte carlo (10000 iterations):
  Lowest cost = 38
  Best combination = [2, 2, 2, 2, 2]
```

## Problem 2

> **Note:** code for this problem can be found here: https://github.com/dsifford/CSC_6500/tree/master/homework/homework_06/sort-integers

I think the learning point here was supposed to have me conclude that using a randomly selected pivot point for quicksort is supposed to produce faster sorts. However, that's not what I found in my implementation (for a majority of the cases).

### Here's a few runs for size 100,000

```sh
# Run 1
Standard Quicksort (size 100000):
  Duration: 393118724 nanoseconds
Randomized Quicksort (size 100000):
  Duration: 449062870 nanoseconds

# Run 2
Standard Quicksort (size 100000):
  Duration: 390404690 nanoseconds
Randomized Quicksort (size 100000):
  Duration: 430821856 nanoseconds

# Run 3
Standard Quicksort (size 100000):
  Duration: 396291651 nanoseconds
Randomized Quicksort (size 100000):
  Duration: 425620270 nanoseconds
```

### Here's a few runs for size 1,000,000

```sh
# Run 1
Standard Quicksort (size 1000000):
  Duration: 4411382638 nanoseconds
Randomized Quicksort (size 1000000):
  Duration: 4931852884 nanoseconds

# Run 2
Standard Quicksort (size 1000000):
  Duration: 4424284348 nanoseconds
Randomized Quicksort (size 1000000):
  Duration: 4828422443 nanoseconds

# Run 3
Standard Quicksort (size 1000000):
  Duration: 4642049809 nanoseconds
Randomized Quicksort (size 1000000):
  Duration: 4909243213 nanoseconds
```

## Problem 3

### Part a)

**Solution:** `x=1`, `y=1`, `z=0`.

### Part b)

**No Solution.**

## Problem 4

> **Note:** code for this problem can be found here: https://github.com/dsifford/CSC_6500/tree/master/homework/homework_06/simulated-annealing

### Output from 5 separate runs of the program

```sh
# Run 1
Optimum value of x = 7

# Run 2
Optimum value of x = 9

# Run 3
Optimum value of x = 5

# Run 4
Optimum value of x = 2

# Run 5
Optimum value of x = 5
```

