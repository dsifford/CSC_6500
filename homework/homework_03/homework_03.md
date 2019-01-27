Derek Sifford

# Homework 3

## Problem 1

```
S  -> %T
T  -> ABTAB
T  -> #
BA -> AB
%A -> a
aA -> aa
bB -> bb
#A -> a
%# -> e
```

## Problem 2

I solved this problem using epsilon closure.

| NFA State     | DFA State | `a` | `b` |
| ------------- | --------- | --- | --- |
| `{1,2,4}`     | `A`       | `B` | `C` |
| `{1,2,3,4,5}` | `B`       | `B` | `D` |
| `{4,5}`       | `C`       | `E` | `F` |
| `{2,4,5}`     | `D`       | `G` | `C` |
| `{5}`         | `E`       | `T` | `T` |
| `{4}`         | `F`       | `E` | `F` |
| `{3,5}`       | `G`       | `T` | `H` |
| `{2}`         | `H`       | `I` | `E` |
| `{3}`         | `I`       | `T` | `H` |
| Trap          | `T`       | `T` | `T` |

![problem_02]

## Problem 3

![problem_03]

## Problem 4

![problem_04]

## Problem 5

**Regular Expression:** `(a|bb|baa)*bab(a|b)*`

[problem_02]: ./bin/problem02.png
[problem_03]: ./bin/problem03.png
[problem_04]: ./bin/problem04.png

