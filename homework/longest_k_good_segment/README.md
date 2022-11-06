The array `a` with `n` integers is given. Let's call 'segment' the sequence of one or more consecutive elements in `a`. Also let's call the segment `k-good` if it contains no more than `k` different values.

Find any longest `k`-good segment.

## Examples
```
Input: A: [1, 2, 3, 4, 5] K: 5
Output: A[0..=4] = A
```
```
Input: A: [6, 5, 1, 2, 3, 2, 1, 4, 5] K: 3
Output: A[2..=6] = [1 2 3 2 1]
```
```
Input: A: [1, 2, 3] K: 1
Output: A[0..=0] = [1]
```