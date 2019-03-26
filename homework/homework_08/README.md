# Homework 8

## Problem 1 (40pts)

### (a) Evaluate gcd (8765, 23485). (5 points)

**Answer:** 5

### (b) Using Extended Euclidean Algorithm, find integers x and y with `65537x+3511y = 1`. (5 points)

### (c) Find the last five digits of 3^1234567. (5 points)

### (d) Solve `314x = 271(mod11111)` (5 points)

**Answer:** 271/314

> **Note:** This question doesn't make sense. 271 mod 11111 (or any number greater than 271) is just 271. Am I reading this correctly?

### (e) Using p = 71, q = 101, and E = 11, find D, the number which is to be used for decryption. (5 points)

### (f) Using p = 2003, q = 4001, and E = 1003, determined the encrypted value C for the plaintext message P = 100000. By calculating, D, decrypt the value C and check that you indeed obtain C back. (10 points)

## Problem 2 (60 pts)

This problem describes a heuristic Type I algorithm (runs in P time,
when it finds a solution, it is always correct and it doesn’t always
find a solution but almost always does.) by D. Angluin and L.Valiant
((J. Computer and System Sciences, 18, 155-193, 1979) for Hamiltonian
problem. The algorithm is given below (taken from the paper)

![image1]
![image2]
![image3]

Use this algorithm to find a Hamiltonian cycle in the icosahedron graph (shown below) and defined by the following connectivity:

-   Vertex 1 connected to 2, 3, 4, 5, 9; Vertex 2 connected to 1, 3, 7, 8, 9;
-   Vertex 3 connected to 1,2, 5, 6,7; Vertex 4 connected to 1, 5, 9, 10,12;
-   Vertex 5 connected to 1,3, 4, 6,12; Vertex 6 connected to 3, 5, 7, 11,12;
-   Vertex 7 connected to 2, 3, 6,8,11; Vertex 8 connected to 2,7,9,10,11;
-   Vertex 9 connected to 1,2, 4,8,10; Vertex 10 connected to 4,8,9,11 ,12;
-   Vertex 11 connected to 6,7,8,10,12; Vertex 12 connected to 4,5,6,10,11;

![image4]

> **Note:** There is another algorithm by these authors dubbed as `ubc`

[image1]: assets/image1.png
[image2]: assets/image2.png
[image3]: assets/image3.png
[image4]: assets/image4.png
