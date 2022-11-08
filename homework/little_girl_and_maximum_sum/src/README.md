# Little Girl and Maximum Sum

The little girl loves the problems on array queries very much.

One day she came across a rather well-known problem: you've got an array of `n`
elements (the elements of the array are indexed starting from 1); also, there are `q` queries, each one is defined by a pair of integers `l, r` (`1 ≤ l ≤ r ≤ n`). You need to find for each query the sum of elements of the array with indexes from `l` to `r`, inclusive.

The little girl found the problem rather boring. She decided to reorder the array elements before replying to the queries in a way that makes the sum of query replies maximum possible. Your task is to find the value of this maximum sum.

## Examples
```
A = [5, 3, 2]
Queries = (1, 2), (2, 3), (1, 3)
Output = 25
```
```
A = [5, 2, 4, 1, 3]
Queries = (1, 5), (2, 3), (2, 3)
Output = 33
```