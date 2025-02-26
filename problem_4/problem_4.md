# Largest Palindrome Product

## Problem 

A palindromic number reads the same both ways. 
The largest palindrome made from the product of two **2**-digit numbers is **9009 = 91 x 99**.

Find the largest palindrome made from the product of two **3**-digit numbers.

## Solution approach

Since we are multiplying two **3**-digit numbers, the palindromes need to be in the range **[10000, 998001]**

The smallest and largest palindromes in that range are 10001 and 997799, respectively.

By making the assumption that the largest palindrome has at least six figures it is enough to consider palindromes of the form 

$$
abccba, \text{where } a \in (1, 9) \text{and }  b,c \in (0, 9) \text{such that } 100a + 10b + c \leq 997.
$$

Starting from **997799** we try dividing the palindrome by **3**-digit numbers and check if the result is also a **3**-digit integer.