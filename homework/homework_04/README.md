# Homework 4

## Problem 1 (12pts)

Consider the following TM (q1, is the starting state, e is
blank; F is the accepting state; S means R/W stays)

    (q1,1)->(q1,1,R); (q1,0)->(q2,e,R);
    (q2,1)->(q2,1,R); (q2,0)->(q3,e,L); (q2,e)->(F,e,S)
    (q3,0)->(q3,0,L); (q3,1)->(q3,1,L); (q3,e)->(q1,e,S)

For each of these strings, show the final tape and say whether the TM
accepts it or not.

### `0011;` (3pts)

### `101010;` (3pts)

### `110;` (3pts)

### `00100` (3pts)

## Problem 2 (18pts)

Consider the following TM over the language {0,1} (q1, is the
starting state, e is blank; F is the accepting state; S means R/W stays)

    (q1,1)->(q2,1,L); (q1,0)->(q3,$,R); (q1,e)->(F,e,S)
    (q2,$)->(q2,0,L); (q2,e)->(q1,e,R)
    (q3,0)->(q3,0,R); (q3,1)->(q4,1,R);
    (q4,1)->(q4,1,R); (q4,e)->(q5,e,L)
    (q5,1)->(q6,e,L)
    (q6,1)->(q6,1,L); (q6,0)->(q6,0,L); (q6,$)->(q1,$,R);

### What happens to the TM if the first symbol of the input is a `1`? (3pts)

### What is the final string if the input is `0011`? (3pts)

### Which four symbol strings does the TM accept? (6pts)

### What is the language of this TM? (6pts)

## Problem 3 (15pts)

Give a TM which will accept the regular language consisting of all
binary strings that contain the substring `101`.

## Problem 4 (15pts)

Give a TM that will accept the following regular expression (or language):

    {a,b}*({ab}{a,b}*U{ba})

## Problem 5 (30pts)

### Give a high level description of a TM which will accept even-length palindromes over the alphabet `{0,1}`. (15pts)

### Give specific transition rules and check your rules against a simple string `01100110`. (15pts)

## Problem 6 (10pts)

Define (give rules) a TM that accepts all binary strings of odd length
with a `0` in the middle.
