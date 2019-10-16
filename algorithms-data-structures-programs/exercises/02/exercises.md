pg 122

# 2.1

| Program | Name                    | Stable? |
| ------- | ----------------------- | ------- |
| 2.1     | straight insertion sort | ☑       |
| 2.2     | binary insertion sort   | ☑       |
| 2.3     | straight selection sort | ☒       |
| 2.4     | bubblesort              | ☑       |
| 2.5     | shakersort              | ☑       |
| 2.6     | shellshort              | ☒       |
| 2.8     | heapsort                | ☒       |
| 2.10    | quicksort (recursive)   | ☒       |
| 2.13    | straight mergesort      | ☒       |

# 2.5

Any set of values causes the partition program to enter an infinite loop. On each iteration of the loop, the indices `i` and `j` converge and the values at their positions in the array are swapped. When `i = j`, the 'swap' step has no effect and on the next iteration of the loop neither of the `while` conditions evaluate to `true` and neither `i` nor `j` change value.
