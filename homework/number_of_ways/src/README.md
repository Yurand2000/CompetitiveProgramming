# Number of ways

You've got array `a[1]`, `a[2]`, ..., `a[n]`, consisting of `n` integers. Count the number of ways to split all the elements of the array into three contiguous parts so that the sum of elements in each part is the same.

More formally, you need to find the number of such pairs of indices `i,j` (`2 ≤ i ≤ j ≤ n - 1`), that `sum(a[1..=i-1])` = `sum(a[i..=j])` = `sum(a[j+1..=n])`

## Examples
```
A = [1 2 3 0 3]
Output = 2
```
```
A = [0 1 -1 0]
Output = 1
```
```
A = [4 1]
Output = 0
```