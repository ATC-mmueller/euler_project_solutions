# Largest Prime Factor

## Problem

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?

## Solution approach

Except for 2 and 3, prime numbers are of the form 

> **p = 6n + 1** or **p = 6n + 5**

Furthermore, a prime factor of a number can not be greater than the square root of that number.


So to find the highest prime factor we check numbers of the above form starting from the square root of 600851475143 (rounded up to the next integer) and moving our way down to 1.

The first factor is the number we are looking for.

Calculation time: under 200ms. 

## Optimization ideas

- Create a list of primes and test against the entries from that list. Use Sieve of Eratosthenes or similar to fill the list.
- Use better primality tests, for instance Miller(-Rabin) test.
