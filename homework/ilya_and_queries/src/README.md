# Ilya and Queries

Ilya the Lion wants to help all his friends with passing exams. They need to solve the following problem to pass the IT exam.

You've got string `s` = `s_1 s_2 ... s_n` (`n` is the length of the string), consisting only of characters `.` and `#` and `m` queries. Each query is described by a pair of integers `l, r` (`1 ≤ l < r ≤ n`). The answer to the query `l, r` is the number of such integers `i` (`l ≤ i < r`), that `s[i]` = `s[i + 1]`.

## Examples
```
String = ......
Query 1 = (3, 4) => 1
Query 2 = (2, 3) => 1
Query 3 = (1, 6) => 5
Query 4 = (2, 6) => 4
```
```
String = #..###
Query 1 = (1, 3) => 1
Query 2 = (5, 6) => 1
Query 3 = (1, 5) => 2
Query 4 = (3, 6) => 2
Query 4 = (3, 4) => 0
```