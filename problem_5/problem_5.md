# Smallest multiple

## Problem 

**2520** is the smallest number that can be divided by each of the numbers from **1** to **10** without any remainder.
What is the smallest positive number that is ***evenly divisible*** by all the numbers from **1** to **20**?

## Solution approach 

Since a number is divisible by another number, if it is divisible by the highest powers of the factors of its' prime factorization, we look to the product of the highest powers of prime factors of all the numbers from **1** to **20**.

Example: 

$$
2 = 2^1, 3 = 3^1, 4 = 2^2, 5 = 5^1
$$

Contained prime numbers: **2**,**3** and **5**.

Highest powers of the prime factors:

$$
2^2, 3^1 \text{ and } 5^1 
$$

Smallest multiple of the numbers from **1** to **5**: 

$$
2^2 * 3^1 * 5^1 = 60
$$