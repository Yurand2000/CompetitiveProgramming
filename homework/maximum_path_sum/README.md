Given a binary tree in which each node element contains a number. Find the maximum possible path sum from one leaf node to another leaf node.

Expected Time Complexity: `O(N)`\
Expected Auxiliary Space: `O(Height of Tree)`
## Examples
```
Input:      
         3                              
        / \                          
       4   5                     
      / \      
    -10  4                          
Output: 16
Explanation:
Maximum Sum lies between leaf node 4 and 5.
4 + 4 + 3 + 5 = 16.
```
```
Input:    
         -15                               
        /   \                          
       5     6                      
      / \   / \
    -8   1 3   9
   /  \         \
  2   -3         0
                / \
               4  -1
                  /
                 10  
Output:  27
Explanation:
The maximum possible sum from one leaf node 
to another is (3 + 6 + 9 + 0 + -1 + 10 = 27)
```