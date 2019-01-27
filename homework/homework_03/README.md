# Homework 3 (Due January 30, 2019)

## Problem 1 (25pts)

Give the unrestricted grammar for the following language:
{a*<sup>n</sup>*b*<sup>n</sup>*a*<sup>n</sup>*b*<sup>n</sup>* : _n_ &ge;
0}.

> **Hint:** Generate a string %ABAB….\# ABAB .. (with n pairs of AB on each
> side of middle marker \#). Move B’s to the right of A’s. Convert A’s
> into a’s and B’s to b’s. Do take care of the case when no A’s and B’s
> are generated

## Problem 2 (20pts)

Convert the NFA defined by the following transition table to the
corresponding DFA

| q   | a     | b     | &lambda; |
| --- | ----- | ----- | -------- |
| 1   | {1}   | &Fcy; | {2,4}    |
| 2   | {3}   | {5}   | &Fcy;    |
| 3   | &Fcy; | {2}   | &Fcy;    |
| 4   | {5}   | {4}   | &Fcy;    |
| 5   | &Fcy; | &Fcy; | &Fcy;    |

## Problem 3 (20pts)

Design an NFA to accept the set of strings of 0’s and 1’s that either
(a) end in 010 and have 011 somewhere preceding , or (b) end in 101 and
have 100 somewhere preceding

## Problem 4 (15pts)

Give an NFA for the following expression: ((aaUb)\*(aba)\*bab)\*.

## Problem 5 (20pts)

Write a regular expression for the language recognized by the following
FA.

![problem_05]

[problem_05]: ./bin/problem05.png
