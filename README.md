# OS_110_MedExercism
This repository is for my assignment on Operating System course at State University of Jakarta. It is about completing five medium problem in Exercism's rust.

i have completed about 7 medium problems for now, here is the list:
- clock
- hamming
- isbn-verifier
- pascals-triangle
- perfect-numbers
- scrabble-score
- triangle

Now i will explain on how i solve ISBN Verifier problem
___
# ISBN Verifier

## The Problem
ISBN Verifier as explained on exercism, we have to determines wheter a string is a valid isbn number or not. Some constraint applied, such as the string must be exactly 10 characters in lenght (without dashes), we could only take integer and the last character could be an 'X' wich represent 10.

I've managed to solve the problem with two different-rather-the-same approaches:
1. The first one is through converting the string into an array of characters and,
2. Directly iterate over the string using method like **chars()** and **enumerate()**

## My Solutio
