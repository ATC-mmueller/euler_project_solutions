# Sum Square Difference

## Problem 

The sum of the squares of the first ten natural numbers is,

$$
1^2 + 2^2 + \dots + 10^2 = 385.
$$

The square of the sum of the first ten natural numbers is,

$$
(1 + 2 + \dots + 10)^2 = 55^2 = 3025.
$$

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is **3025 - 385 = 2640**.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

## Solution approach

We use the following formulas:

$$
\sum\limits_{i=1}^n i = \frac{n(n+1)}{2} \quad \text{and} \quad \sum\limits_{i=1}^n i^2 = \frac{n(n+1)(2n+1)}{6}. 
$$

With those we get

$$
\left( \sum\limits_{i=1}^{n} i \right)^2 - \sum\limits_{i=1}^{n} i^2 = \left( \frac{n(n+1)}{2} \right)^2 - \frac{n(n+1)(2n+1)}6
$$

$$
= \frac{n^2(n+1)^2}{4} - \frac{n(n+1)(2n+1)}6
= \frac{(n+1)(3n^2(n+1) - 2n(2n+1))}{12}
$$

$$
= \frac{n(n+1)(3n^2-n-2)}{12}
$$

Inserting **100** gives the result.
