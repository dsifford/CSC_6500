# Homework 2

## Problem 1 (25pts)

### Part 1.

Draw an automaton which will accept all integers as powers of 4, represented in
binary form (this problem was done in the class).

![dfa]

\pagebreak

### Part 2.

Now write a code in a high-level language (C++, C\# or Java or ??) for the
automaton. Create an input file with the following integers, represented in the
binary form as separate lines:

#### Integers List

-   4<sup>3</sup>
-   4<sup>6</sup>
-   2<sup>3</sup>
-   5<sup>3</sup>
-   6<sup>2</sup>
-   9<sup>2</sup>

#### File

```
1000000
1000000000000
1000
1111101
100100
1010001
```

Use this input file to the code for your automata. Print your code and results.
If more convenient, e-mail to your TA at `gd0393@wayne.edu`

\pagebreak

## Problem 2 (75pts)

This problem is designed to show you how DFA is used in Compilers. DFA for
finding the tokens.

We have the following input file: (for foreign students the names here are of
well know comics)

```
larry = 27

curly = 19

moe = 8

groucho = 11

harpo = larry+curly

harpo = larry-curly

harpo = larry*curly

harpo = larry/curly

harpo = larry*curly+moe*groucho
```

We need to define a DFA which will give the output various tokens. In this case,
the identifiers will be larry, curly, moe , groucho , and harpo. The operators
will be /, +, -, \*, +, = and integers 27, 19, 8, and 11. They will come as an
output file.

Here is a DFA for this purpose. (`q0` is the starting state, `q1`, `q2` and `q3`
are accepting states (if it is accepted in state q2, the token is an operator,
if it is accepted in `q1`, the token is an identifier for any string that starts
with a letter and then continues with any combination of letters and integer
digits. The maximum number of characters for any identifier should be 10.) .
Operators are accepted in state `q2` and and in `q3` if it is an integer
(`intLit`) – you can limit the number of digits to 10). One should write down
what strings are accepted (token) and list the names of new tokens.

Email your results to the TA as in Problem 1.

[dfa]: bin/dfa.png
