Derek Sifford

# Homework 8

## Problem 1 (40pts)

### (a) Evaluate gcd (8765, 23485). (5 points)

```
GCD(23485, 8765)
=> 23485 = 2 * 8765 + 5955
   GCD(8765, 5995)
   => 8765 = 1 * 5995 + 2770
      GCD(5995, 2770)
      => 5995 = 2 * 2770 + 455
         GCD(2770, 455)
         => 2770 = 6 * 455 + 40
            GCD(455, 40)
            => 455 = 11 * 40 + 15
               GCD(40, 15)
               => 40 = 2 * 15 + 10
                  GCD(15, 10)
                  => 15 = 1 * 10 + 5
                     GCD(10, 5)
                     => 10 = 2 * 5 + 0
                        GCD(5, 0)
                        => 5
```

**Answer:** 5

\pagebreak

### (b) Using Extended Euclidean Algorithm, find integers x and y with `65537x+3511y = 1`. (5 points)

```
GCD(65537, 3511)
=> 65537 = 18 * 3511 + 2339
   GCD(3511, 2339)
   => 3511 = 1 * 2339 + 1172
      GCD(2339, 1172)
      => 2339 = 1 * 1172 + 1167
         GCD(1172, 1167)
         => 1172 = 1 * 1167 + 5
            GCD(1167, 5)
            => 1167 = 233 * 5 + 2
               GCD(5, 2)
               => 5 = 2 * 2 + 1
                  GCD(2, 1)
                  => 2 = 2 * 1 + 0
1 = 5 - 2 * 2
1 = 5 - 2 * (1167 - 233 * 5)
1 = 467 * 5 - 2 * 1167
1 = 467 * (1172 - 1 * 1167) - 2 * 1167
1 = 467 * 1172 - 469 * 1167
1 = 467 * 1172 - 469 * (2339 - 1 * 1172)
1 = 936 * 1172 - 469 * 2339
1 = 936 * (3511 - 1 * 2339) - 469 * 2339
1 = 936 * 3511 - 1405 * 2339
1 = 936 * 3511 - 1405 * (65537 - 18 * 3511)
1 = 26226 * 3511 - 1405 * 65537
```

**Answer:** `x = -1405`, `y = 26226`

### (c) Find the last five digits of 3^1234567. (5 points)

**Answer:** 40587

### (d) Solve `314x = 271(mod11111)` (5 points)

<!-- TODO... -->

### (e) Using p = 71, q = 101, and E = 11, find D, the number which is to be used for decryption. (5 points)

```
n = pq = 71 * 101 = 7071

phi(n) = (p - 1)(q - 1) = 70 * 100 = 7000

e = 11

d = 11 (mod 7000) = 5091
```

**Answer:** 5091

\pagebreak

## Problem 2 (60 pts)

This problem describes a heuristic Type I algorithm (runs in P time,
when it finds a solution, it is always correct and it doesn’t always
find a solution but almost always does.) by D. Angluin and L.Valiant
((J. Computer and System Sciences, 18, 155-193, 1979) for Hamiltonian
problem. The algorithm is given below (taken from the paper)

### DHC Algorithm

As input the algorithm takes a directed graph `G` of `n` nodes, together with
two specified nodes `s` and `t` (which may be the same). The algorithm attempts
to find a directed HP from `s` to `t` in `G`. If it succeeds, it returns
`success`, otherwise it returns `failure`. During execution, it maintains a
_partial path_ `P` which consists either of a simple directed path, or of the
disjoint union of a simple directed path and a simple directed cycle. In either
case, the terminal endpoint of the path is kept in a variable called `ndp` (for
_next departure point_) and the algorithm attempts to extend `P` by calling
`SELECT` to explore edges at random out of the node `ndp`.

![image1]

![image2]

Use this algorithm to find a Hamiltonian cycle in the icosahedron graph (shown below) and defined by the following connectivity:

-   Vertex 1 connected to 2, 3, 4, 5, 9; Vertex 2 connected to 1, 3, 7, 8, 9;
-   Vertex 3 connected to 1,2, 5, 6,7; Vertex 4 connected to 1, 5, 9, 10,12;
-   Vertex 5 connected to 1,3, 4, 6,12; Vertex 6 connected to 3, 5, 7, 11,12;
-   Vertex 7 connected to 2, 3, 6,8,11; Vertex 8 connected to 2,7,9,10,11;
-   Vertex 9 connected to 1,2, 4,8,10; Vertex 10 connected to 4,8,9,11 ,12;
-   Vertex 11 connected to 6,7,8,10,12; Vertex 12 connected to 4,5,6,10,11;

![image3]

> **Note:** There is another algorithm by these authors dubbed as `ubc`

[image1]: bin/image1.png
[image2]: bin/image2.png
[image3]: bin/image3.png
