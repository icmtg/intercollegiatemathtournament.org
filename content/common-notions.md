+++
title = "Common mathematical notions"
date = 2026-02-28
+++

1. Answers should be exact and simplified. For more information on valid answer formats, see the [Acceptable Answer Formats](/answer-formats.pdf) document.

2. If a diagram is given with a problem, it is not necessarily drawn to scale.

3. In a triangle $ABC$, the vertices are called $A$, $B$, and $C$. The sides are called $a$, $b$, and $c$, with side $a$ opposite vertex $A$, side $b$ opposite vertex $B$, and side $c$ opposite vertex $C$. If a polygon is called $ABCDEF$, its vertices will occur in that order around the polygon. This convention holds for all namings of polygons.

4. Unless otherwise noted, polygons (including triangles) are simple and non-degenerate.

5. If complex numbers are used in a problem, $i$ denotes $\sqrt{-1}$.

6. $\mathbb{Z}$ denotes the set of integers, $\mathbb{Q}$ the set of rational numbers, $\mathbb{R}$ the set of real numbers, and $\mathbb{C}$ the set of complex numbers.

7. Logs are base $e$ unless otherwise indicated. When logs are used in a different base, a subscript will be used, as in $\log_{10} 2$. Base $e$ logs may also be written with $\ln$, as in $\ln 2$.

8. The exponential function $e^x$ may also be denoted $\exp(x)$.

9. The word *prime* refers to positive integers only. Note that 1 is not a prime.

10. Divisors and factors of a positive integer refer to positive numbers only. Proper divisors of a positive integer refer to divisors that are less than that integer.

11. The notation $|S|$ denotes the number of elements in a finite set $S$.

12. If $A$ and $B$ are sets, the notation $A \subseteq B$ means that every element of $A$ is also an element of $B$. In this case, $A$ is called a subset of $B$. If additionally $A \neq B$, then $A$ is called a proper subset of $B$, and this may be denoted by $A \subset B$.

13. If a problem refers to the digits of a number, those digits are underlined to distinguish the digits of a number from the product of the digits. For example, $\underline{3\;1\;A\;B}$ refers to a four digit number and not the product $3 \cdot 1 \cdot A \cdot B$.

14. Combinations will be denoted by $\binom{n}{k}$; this is the number of ways to choose $k$ unordered things from $n$ things.

15. The expressions $\arcsin x$, $\sin^{-1} x$, $\arccos x$, $\cos^{-1} x$, $\arctan x$, $\tan^{-1} x$ refer to the principal values of these inverse trigonometric functions. This means that

    $$-\frac{\pi}{2} \leq \sin^{-1} x \leq \frac{\pi}{2}, \qquad 0 \leq \cos^{-1} x \leq \pi, \qquad -\frac{\pi}{2} \leq \tan^{-1} x \leq \frac{\pi}{2}.$$

16. If a trigonometric problem does not specify the use of degrees, all trigonometric expressions are given in radians.

17. The floor function (or greatest integer function) is denoted by $\lfloor x \rfloor$, and it is defined as $\lfloor x \rfloor = n$ when $n$ is an integer and $n \leq x < n + 1$. Similarly, the ceiling function (or least integer function) is denoted by $\lceil x \rceil$, and it is defined as $\lceil x \rceil = n$ when $n$ is an integer and $n - 1 < x \leq n$.

18. The fractional part is denoted by $\\{x\\}$, and it is defined as $\\{x\\} = x - \lfloor x \rfloor$.

19. The Euler totient function is denoted by $\varphi(n)$, and it is defined as the number of positive integers less than or equal to $n$ that are relatively prime to $n$. That is,

    $$\varphi(n) = |\{ k \in \mathbb{N} : 1 \leq k \leq n \text{ and } \gcd(k, n) = 1 \}|.$$

20. Intervals are written as a pair of numbers. Round brackets indicate that the endpoint is excluded, while square brackets indicate that the endpoint is included. For example, the interval $(2, 3]$ denotes $\\{x : 2 < x \leq 3\\}$.

21. The greatest lower bound of a set is the largest number that is less than or equal to every number of the set. For example, the greatest lower bound of the intervals $(2, 3)$ and $[2, 3]$ are both $2$. The least upper bound of a set is the smallest number that is greater than or equal to every number of the set. For example, the least upper bound of the intervals $(2, 3)$ and $[2, 3]$ are both $3$.

22. $\max(a_1, a_2, \ldots, a_n)$ denotes the largest element in the set of real numbers $\\{a_1, a_2, \ldots, a_n\\}$. Similarly, $\min(a_1, a_2, \ldots, a_n)$ denotes the smallest element in the set of real numbers $\\{a_1, a_2, \ldots, a_n\\}$.

23. The expression $\sqrt{x}$ denotes the nonnegative square root of $x$.

24. Functions are assumed to be well-defined wherever they are used.

25. The absolute value of a real or complex number $z$ is denoted by $|z|$.

26. The real part and the imaginary part of a complex number $z$ are denoted by $\operatorname{Re} z$ and $\operatorname{Im} z$ respectively. If $z = a + bi$ where $a$ and $b$ are real, then $\operatorname{Re} z = a$ and $\operatorname{Im} z = b$.

27. When the complex logarithm is used, $\log(z)$ denotes the principal branch unless otherwise specified:

    $$\log(z) = \ln |z| + i \operatorname{Arg}(z), \qquad -\pi < \operatorname{Arg}(z) \leq \pi.$$

28. The empty set is denoted by $\emptyset$.

29. The empty product is equal to $1$, and the empty sum is equal to $0$.

30. An abelian group is a group such that the group operation is commutative.

31. Unless otherwise specified, a ring is assumed to have distinct additive and multiplicative identities; in particular, $0 \neq 1$.

32. If $q = p^k$ for a prime $p$ and positive integer $k$, $\mathbb{F}_q$ denotes the finite field with $q$ elements.

33. The notation $f : A \to B$ denotes a function from $A$ to $B$.

34. The determinant of a square matrix $A$ is denoted by $\det(A)$.

35. For a matrix $M$, the entry in the $i$th row and $j$th column is denoted $m_{ij}$, and we equivalently denote the matrix by $(m_{ij})$.

36. If $\pi$ is a permutation of a finite set, its sign, denoted $\operatorname{sgn}(\pi)$, is defined by

    $$\operatorname{sgn}(\pi) = \begin{cases} +1 & \text{if } \pi \text{ can be written as a product of an even number of transpositions,} \\ -1 & \text{if } \pi \text{ can be written as a product of an odd number of transpositions.} \end{cases}$$

    A permutation with sign $+1$ is called even, and one with sign $-1$ is called odd.
