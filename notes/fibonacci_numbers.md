# Fibonacci Number

## Problem

Calculate F(n) modulo 10^9+7, where F is the Fibonacci sequence (F0=0, F1=1, Fn=Fn-2+Fn-1). n can be up to 10^18.

## Constraints

- 0 ≤ n ≤ 10^18

## Example

Input:

```text
10
```

Output:

```text
55
```

## Solution

- Pattern: math, matrix exponentiation
- Technique: [[0,1],[1,1]]^(n-1), read top-right element
- Time: O(log n)
- Space: O(1)

## Key Insight

Matrix identity: [[F(n), F(n-1)], [F(n-1), F(n-2)]] = [[0,1],[1,1]]^(n-1). Binary exponentiation reduces multiplications to O(log n).

## Notes

- Matrix stored as flat tuple (a, b, c, d) for simplicity
- Modulo 10^9+7 applied at each multiplication step
