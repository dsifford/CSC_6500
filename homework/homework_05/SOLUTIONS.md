Derek Sifford

# Homework 5

## Problem 1

### Part 1

Binary to unary TM Rules (starting state: `init`):

    (init,0)->(init,0,R); (init,1)->(init,1,R); (init,e)->(sub,e,L);
    (sub,0)->(sub,0,L); (sub,1)->(shift,0,R); (sub,e)->(done,e,R);
    (shift,0)->(shift,1,R); (shift,1)->(shift,0,R); (shift,e)->(add,e,R);
    (add,1)->(add,1,R); (add,e)->(reset,1,L);
    (reset,1)->(reset,1,L); (reset,e)->(sub,e,L);

**Note to grader:** This can be tested by inputting the following code in https://turingmachine.io

```yml
input: "1010"
blank: _
start state: init
table:
    init:
        0: R
        1: R
        _: { L: sub }
    sub:
        0: L
        1: { write: 0, R: shift }
        _: { R: done }
    shift:
        0: { write: 1, R }
        1: { write: 0, R }
        _: { R: add }
    add:
        1: R
        _: { write: 1, L: reset }
    reset:
        1: L
        _: { L: sub }
    done:
```

\pagebreak

### Part 2

Unary to binary TM Rules (starting state: `scan`):

    (scan,1)->(scan,1,R); (scan,e)->(take,e,L);
    (take,1)->(moveL,e,L);
    (moveL,1)->(moveL,1,L); (moveL,e)->(add,e,L);
    (add,e)->(reset,1,R); (add,0)->(reset,1,R); (add,1)->(carry,0,L);
    (carry,e)->(reset,1,R); (carry,0)->(reset,1,R); (carry,1)->(carry,0,L);
    (reset,1)->(reset,1,R); (reset,0)->(reset,0,R); (reset,e)->(scan,e,R);

**Note to grader:** This can be tested by inputting the following code in https://turingmachine.io

```yml
input: "1111111111"
blank: _
start state: scan
table:
    scan:
        1: R
        _: { L: take }
    take:
        1: { write: _, L: moveL }
    moveL:
        1: L
        _: { L: add }
    add:
        _: { write: 1, R: reset }
        0: { write: 1, R: reset }
        1: { write: 0, L: carry }
    carry:
        _: { write: 1, R: reset }
        0: { write: 1, R: reset }
        1: { write: 0, L }
    reset:
        1: R
        0: R
        _: { R: scan }
```

### Part 3

**Note:** This portion is being turned in via email to the TA.

#### Test Output

**Binary to Unary**

```
Running for input 57 (binary 111001)
        b2u = 111111111111111111111111111111111111111111111111111111111
        u2b = 111001
```

**Unary to Binary**

```
Running for input 57 (unary 111111111111111111111111111111111111111111111111111111111)
        u2b = 111001
        b2u = 111111111111111111111111111111111111111111111111111111111
```

## Problem 2

### Part 1

**Solution:** Tile 1, Tile 3, Tile 2, Tile 2

### Part 2

**Solution:** Tile 1, Tile 2

### Part 3

**Solution:** No solution

### Part 4

**Solution:** No solution

### Part 5

**Solution:** Tile 1, Tile 2, Tile 1, Tile 3

### Part 6

**Solution:** No solution

## Problem 3

### Part 1

**Solution:** 21221212212232233211322123221123222132323313

### Part 2

**Solution:** No solution

### Part 3

**Solution:** No solution

## Problem 4

**Note:** This portion is being turned in via email to the TA.

### Test outputs

|   Input | Output                                  |
| ------: | --------------------------------------- |
|     `8` | Final number is 4 after 1 iterations    |
|    `19` | Final number is 16 after 16 iterations  |
|    `28` | Final number is 16 after 14 iterations  |
|    `47` | Final number is 16 after 100 iterations |
|    `85` | Final number is 256 after 1 iterations  |
| `23478` | Final number is 16 after 47 iterations  |
