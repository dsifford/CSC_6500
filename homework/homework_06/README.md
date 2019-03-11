# Homework 6 (Due March 13, 2019)

## Problem 1 (30pts)

This problem is for multiprocessor scheduling. There are five modules
(subroutines) 1, 2, 3, 4, and 5 of a program which can be run on five
processors (P1, P2, P3, P4 and P5). The cost of running each module on
an assigned processor is as follows:

| Module |  P1 |  P2 |  P3 |  P4 |  P5 |
| -----: | --: | --: | --: | --: | --: |
|      1 |   4 |  15 |  20 |   8 |  12 |
|      2 |   9 |   5 |   2 |  10 |  15 |
|      3 |  15 |   9 |   5 |   6 |   3 |
|      4 |  20 |   6 |   7 |   4 |  10 |
|      5 |   8 |   6 |  12 |  20 |  17 |

However, there are inter-module communications costs if two modules are
run on separate processors. These costs are as follows:

|     |   1 |   2 |   3 |   4 |   5 |
| --: | --: | --: | --: | --: | --: |
|   1 |   0 |   6 |  10 |   3 |   8 |
|   2 |   6 |   0 |   8 |  10 |   9 |
|   3 |  10 |   8 |   0 |   9 |   7 |
|   4 |   3 |  10 |   9 |   0 |   5 |
|   5 |   8 |   9 |   7 |   5 |   0 |

The goal is to find the assignment of modules which will minimize the
cost. Note the following:

1. If there was no inter-module communication cost, the optimal
   assignment will be for each module to run on the processor with
   minimum cost. Thus `Module 1->P1`, `Module 2->P3`, `Module 3->P5`,
   `Module 4->P4`, and `Module 5->P2`, with `total cost = 4+2+3+4+6 = 19`.
   But with the communication cost, one has the intercommunication costs
   for `12(6)`, `13(10)`, `14(3)`, `15(8)`, `23(8)`, `24(10)`, `25(9)`,
   `34(9)`, `35(7)`, and `45(5)`. Which is equal to `6+10+3+8+8+10+9+9+7+5 = 75`.
   Thus the total cost will be `75+19 = 94`.
2. One can assign all modules to one processor. Assigning to Processor
   1 cost will be `(4+9+15+20+8) = 56`, with similar costs for assigning
   to Processor 2, to Processor 3, to Processor 4, and to Processor 5.
3. Thus there are many ways of assigning Modules to Processors. Each
   Module can be assigned to one of the five processors. Thus, there
   are `5x5x5x5x5 = 3125` ways of assigning the modules to processors.
   Write a program with 5 for loops to find the optimal assignment.
4. If the number of modules is say hundred and there are hundred
   processors, this exhaustive method will take a long time.
5. Use the _Monte Carlo_ method to randomly assign each module to one of
   the processors and calculate the total cost. Repeat the process
   10000 times and ignore the choice which leads to a higher cost.
   Compare your best answer with the correct answer obtained by
   exhaustive enumeration.

## Problem 2 (30pts)

1.  Generate randomized integers from 0 to 99,999 (100,000 numbers).
    Sort them using a _Quicksort_ algorithm. Use a command which prints
    out the computer time used. Repeat the exercise with one million
    numbers.
2.  Now redo (a) using a _Randomized Quicksort_ algorithm (where the pivot
    is chosen randomly). What comments can you make about the computer
    times for the two algorithms? (This is an example of Las Vegas
    algorithm)

Here is the outline of Quicksort but you can choose your own routine

```
Quicksort:
    Input: a list L = (a_1,... a_n) of integers

    If n <= 1
        then return L
    Else
        Let i = 1          <------ 1: this line is different in below problem

        Let L_1 be the subset of L whose elements are <= a_i
        Let L_2 be the subset of L whose elements are == a_i
        Let L_3 be the subset of L whose elements are >= a_i

        recursively Quicksort L_1 and L_3

        return L as the concatenation of the list L_1, L_2, and L_3

end.
```

### Randomized Quicksort

```
{ (same as deterministic Quicksort, except line 1:}
1: choose a random integer i, 1 <= i <= n;
```

## Problem 3 (15pts)

<!-- See here: https://en.wikipedia.org/wiki/Boolean_satisfiability_problem -->

Solve the following SAT problems using backtracking method discussed in
the class (here z' is the complement of z). Give your solution or show
that it has no solution.

```
(a) (x OR y OR z') AND (x' OR y) AND (x' OR y' OR z')

(b) (x OR y') AND (x' OR z) AND (y OR z') AND (x' OR y') AND (y OR z)
```

## Problem 4 (25pts)

<!-- See here: https://en.wikipedia.org/wiki/Simulated_annealing -->

Minimize the function `E(x) = x^2 – 15x + 54 = 0`, using
_simulated annealing_ method. Take an initial random guess `x1`
between 0 and 99. Set the value of Temperature `T = 100`. Calculate
`E(x1)`. Choose another random guess `x2` (again between 0 and 99).
Calculate `E(x2)`. Calculate the probability `P = exp(-[E(x2)-E(x1)]/T)`.
If `p > 1`, accept `x2`, otherwise choose another random guess.
Continue 5 iterations. Then reduce the temperature to `90` and repeat
the procedure. Then reduce the temperature to `80, 70, ..., 10`.
Determine the optimum value of x.

Submit the output (and the program as hard copy or softcopy) to the TA
Tuan Minh Nguyen (tuan.minh.nguyen@wayne.edu)
