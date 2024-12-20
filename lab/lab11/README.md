# Lab 11

I chose Rust for this lab

For this task, Data2 seemed to be the most difficult one, so I set it as the default file to read from.

do cargo run to see it in action!

## The task

1. **Decipher the Inscriptions**: Write a program to read the alien inscriptions from the **standard input**. Each line of input provides a single inscription.
2. **Extract 2-digit numbers**: Each inscription consists of a series of alpha-numerical symbols. The first and the last decimal digit represent the left and right parts of the two-digit numerical value in decimal.
3. **Digits as english words**: Some of the digits are actually spelled out with letters: `one, two, three, four, five, six, seven, eight`, and `nine` also count as valid "digits".

For example:
```
two3nineight
eightwofour
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the extracted 2-digit values are `28, 84, 13, 24, 42, 14, 76`.
Adding these together produces `282`.
Calculating `281 mod 360` gives us `281` which is the new bearing component that we need to report back.



