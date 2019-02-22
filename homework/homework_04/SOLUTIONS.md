Derek Sifford

# Homework 4

## Problem 1

### `0011` (3pts)

Final tape: `ee11` \
TM **does not** accept.

### `101010` (3pts)

Final tape: `1e1e10` \
TM **does not** accept.

### `110` (3pts)

Final tape: `11e` \
TM **does** accept.

### `00100` (3pts)

Final tape: `ee100` \
TM **does not** accept.

## Problem 2

### What happens to the TM if the first symbol of the input is a `1`? (3pts)

The TM gets locked up and bounces back and forth between state `q1` and `qt` infinitely many times. AKA, the language is not accepted.

### What is the final string if the input is `0011`? (3pts)

Final string: `$$ee`

### Which four symbol strings does the TM accept? (6pts)

The 4 tape symbols are `1`, `0`, `$`, and `e`.

### What is the language of this TM? (6pts)

The language of this TM is `0+1+` or, put another way, 1 or more `0`s followed by 1 or more `1`s.

## Problem 3

    (q0,1)->(q1,e,R); (q0,0)->(q0,e,R);
    (q1,0)->(q2,e,R); (q1,1)->(q1,e,R);
    (q2,1)->(F,e,R);  (q2,0)->(q0,e,R);

## Problem 4

    (q0,a)->(q1,a,R); (q0,b)->(q0,b,R); (q0,e)->(q2,e,L);
    (q1,a)->(q1,a,R); (q1,b)->(F,b,R);  (q1,e)->(q2,e,L);
    (q2,a)->(q3,a,L);
    (q3,b)->(F,b,L);

## Problem 5

Basically, this can be done in the following way:

1. Read the first input symbol, write empty, and move to state that depends on that input symbol.
2. Move all the way to the right (to first empty), then move left 1, check if the symbol matches the symbol determined by the state that we're currently in, if it does, write an empty string.
3. Move left 1, read the symbol, write, empty, and move to a state that depends on that input symbol.
4. Move all the way to the left (to first empty), move right 1, check if the symbol matches the symbol determined by the state that we're currently in, if it does, write an empty string.
5. Lather, rinse, repeat until there are no symbols left in the tape. If at any point the symbols don't match where expected, move to a fail state (optional) or just halt.

### Rules

    (GetL,1)   ->(Has1L,e,R); (GetL,0) ->(Has0L,e,R); (GetL,e) ->(Accept,e,R);
    (GetR,1)   ->(Has1R,e,L); (GetR,0) ->(Has0R,e,L); (GetR,e) ->(Accept,e,L);
    (Has1L,1)  ->(Has1L,1,R); (Has1L,0)->(Has1L,0,R); (Has1L,e)->(Check1R,e,L);
    (Has0L,1)  ->(Has0L,1,R); (Has0L,0)->(Has0L,0,R); (Has0L,e)->(Check0R,e,L);
    (Has1R,1)  ->(Has1R,1,L); (Has1R,0)->(Has1R,0,L); (Has1R,e)->(Check1L,e,R);
    (Has0R,1)  ->(Has0R,1,L); (Has0R,0)->(Has0R,0,L); (Has0R,e)->(Check0L,e,R);
    (Check1R,1)->(GetR,e,L);
    (Check0R,0)->(GetR,e,L);
    (Check1L,1)->(GetL,e,R);
    (Check0L,0)->(GetL,3,R);

**Note to grader:** You can check these rules by plugging the following into https://turingmachine.io.

```yml
input: '01100110'
blank: _
start state: GetL
table:
    GetL:
        1: { write: _, R: Has1L }
        0: { write: _, R: Has0L }
        _: { R: Accept }
    GetR:
        1: { write: _, L: Has1R }
        0: { write: _, L: Has0R }
        _: { L: Accept }
    Has1L:
        1: R
        0: R
        _: { L: Check1R }
    Has0L:
        1: R
        0: R
        _: { L: Check0R }
    Has1R:
        1: L
        0: L
        _: { R: Check1L }
    Has0R:
        1: L
        0: L
        _: { R: Check0L }
    Check1R:
        1: { write: _, L: GetR }
    Check0R:
        0: { write: _, L: GetR }
    Check1L:
        1: { write: _, R: GetL }
    Check0L:
        0: { write: _, R: GetL }
    Accept:
```

## Problem 6

### Rules

    (CheckL,0)->(CheckEmptyL,0,R); (CheckL,1)->(PassL,1,R);
    (CheckR,0)->(CheckEmptyR,0,L); (CheckR,1)->(PassR,1,L);
    (CheckEmptyL,1)->(ClearL,1,L); (CheckEmptyL,0)->(ClearL,0,L); (CheckEmptyL,e)->(Accept,e,L);
    (CheckEmptyR,1)->(ClearR,1,R); (CheckEmptyR,0)->(ClearR,0,R); (CheckEmptyR,e)->(Accept,e,R);
    (PassL,1)->(ClearL,1,L); (PassL,0)->(ClearL,0,L); (PassL,e)->(ClearL,e,L);
    (PassR,1)->(ClearR,1,R); (PassR,0)->(ClearR,0,R); (PassR,e)->(ClearR,e,R);
    (ClearL,1)->(MoveR,e,R); (ClearL,0)->(MoveR,e,R);
    (ClearR,1)->(MoveL,e,L); (ClearR,0)->(MoveL,e,L);
    (MoveL,1)->(MoveL,1,L); (MoveL,0)->(MoveL,0,L); (MoveL,e)->(CheckL,e,R);
    (MoveR,1)->(MoveR,1,R); (MoveR,0)->(MoveR,0,R); (MoveR,e)->(CheckR,e,L);

**Note to grader:** You can check these rules by plugging the following into https://turingmachine.io.

```yml
input: '0110111'
blank: _
start state: CheckL
table:
    CheckL:
        0: { R: CheckEmptyL }
        1: { R: PassL }
    CheckR:
        0: { L: CheckEmptyR }
        1: { L: PassR }
    CheckEmptyL:
        1: { L: ClearL }
        0: { L: ClearL }
        _: { L: Accept }
    CheckEmptyR:
        1: { R: ClearR }
        0: { R: ClearR }
        _: { R: Accept }
    PassL:
        1: { L: ClearL }
        0: { L: ClearL }
        _: { L: ClearL }
    PassR:
        1: { R: ClearR }
        0: { R: ClearR }
        _: { R: ClearR }
    ClearL:
        1: { write: _, R: MoveR }
        0: { write: _, R: MoveR }
    ClearR:
        1: { write: _, L: MoveL }
        0: { write: _, L: MoveL }
    MoveL:
        1: L
        0: L
        _: { R: CheckL }
    MoveR:
        1: R
        0: R
        _: { L: CheckR }
    Accept:
```

