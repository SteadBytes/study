# CodeKata 18: Transitive Dependencies

http://codekata.com/kata/kata18-transitive-dependencies/

Input:

Set of lines representing the direct dependencies of an item.

- Each line lists the items that that *first* item in the line depends on

```
A   B   C // A depends on B and C
B   C   E // B depends on C and E
C   G     // C depends on G
D   A   F
E   F
F   H
```

Output:

Full set of dependencies for all items.

For the example input above:

```
A   B C E F G H
B   C E F G H
C   G
D   A B C E F G H
E   F H
F   H
```
