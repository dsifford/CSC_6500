---
title: Homework 9
author: Derek Sifford
date: 04/06/2019
---

## Problem 1 (30 pts)

> Show that surrounding a CNOT gate with Hadamard gates switches the role
> of the control-bit and target-bit of the CNOT: $(H \otimes H)CNOT(H \otimes H)$ is the
> 2-qubit gate where the second bit controls whether the first bit is
> negated (i.e., flipped).

![prob1]

\pagebreak

## Problem 2 (30 pts)

> Compute the result of applying a Hadamard transform to both qubits of
> $|0 \rangle \otimes | 1 \rangle$ in two ways (the first way using tensor product of vectors,
> the second using tensor product of matrices), and show that the two
> results are equal: $H|0 \rangle \otimes H|1\rangle = (H \otimes H) (|0 \rangle \otimes |1 \rangle)$.

**Note:** Assume input is $|00\rangle$

\begin{align*}
\frac{1}{\sqrt{2}}
\begin{bmatrix}
1 & 1 \\
1 & -1
\end{bmatrix}
\otimes
\begin{bmatrix}
1 & 1 \\
1 & -1
\end{bmatrix}
&= \frac{1}{2}
\begin{bmatrix}
1 & 1 & 1 & 1 \\
1 & -1 & 1 & -1 \\
1 & 1 & -1 & -1 \\
1 & -1 & -1 & 1
\end{bmatrix} \\
\frac{1}{2}
\begin{bmatrix}
1 & 1 & 1 & 1 \\
1 & -1 & 1 & -1 \\
1 & 1 & -1 & -1 \\
1 & -1 & -1 & 1
\end{bmatrix}
\begin{bmatrix}
1 \\
0 \\
0 \\
0 \\
\end{bmatrix}
&=
\begin{bmatrix}
0.5 \\
0.5 \\
0.5 \\
0.5 \\
\end{bmatrix} \\
\left(
\frac{|0 \rangle + |1 \rangle}{\sqrt{2}}
\right)_A \otimes
\left(
\frac{|0 \rangle + |1 \rangle}{\sqrt{2}}
\right)_B
&=
\begin{bmatrix}
0.5 \\
0.5 \\
0.5 \\
0.5 \\
\end{bmatrix}
\end{align*}

## Problem 3 (40 pts)

> Construct an XOR circuit in quantum computer using CNOT and Hadamard gates.
> Implement your circuit on real quantum computer on the following
> website: https://quantumexperience.ng.bluemix.net/

**Answer:** The `CNOT` gate already acts as an `XOR` gate so either this question does not make sense or it's a trick question.

[prob1]: bin/problem1.png
