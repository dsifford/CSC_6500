Derek Sifford

# Homework 1

## 1. Find a grammar for the following languages:

### (a) set of binary numerals that represent odd natural numbers.

**Alphabet:** `{0,1}`

**Grammar:**

```
S -> Aa
A -> 0A | 1A | e
a -> 1
```

### (b) set of binary numerals that represent even natural numbers.

**Alphabet:** `{0,1}`

**Grammar:**

```
S -> Aa
A -> 0A | 1A | e
a -> 0
```

## 2. Construct a grammar that generates the following language:

```
L = (a^nb^(n+m)a^m|n, m=0,1,2,...).
```
**Alphabet:** `{a,b}`

**Grammar:**

```
S -> AB
A -> aAb | e
B -> bBa | e
```

## 3. For an alphabet consisting of {a,b}, give a CFG generating a languaage with twice as many a's as b's.

```
S -> A
A -> aaAb | e
```

## 4. What language is accepted by the following DFA?

Any string starting with the alphabet `{a,b}` with the following conditions:

1. If the string begins with `a`, then the string can not have any instance of `bb`.
2. The string must never have any instance of `bab` regardless of what it starts with.

## 5. Give a DFA for the following language:

```
{w E {a,b,c}* : word does not end in a}
```

![dfa-diagram]

[dfa-diagram]: bin/dfa.png

