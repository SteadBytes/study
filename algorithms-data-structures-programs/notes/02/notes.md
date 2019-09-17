# Sorting Intro

Sorting permutes items:

- _a<sub>1</sub>, a<sub>2</sub>, ..., a<sub>n</sub>_

Into an order:

- _a<sub>k1</sub>, a<sub>k2</sub>, ..., a<sub>kn</sub>_

With an ordering function _f_ such that:

- _f(a<sub>k1</sub>) <= f(a<sub>k2</sub>) <= ... <= f(a<sub>kn</sub>)_

Items _a<sub>i</sub>_ represented by a an _item_ record:

```
type item = record key: integer;
                    {other components declared here}
            end
```

- Ordering function stored as an explicit component of an item -> _key_
- "Other components" are **not relevant** to the operation of sorting
  - Only the _key_ matters -> identifying the items
- _key_ doesn't have to be an integer
  - Any type on which a **total ordering relation** is defined can be used

**Stable sort** = relative order of items with _equal keys_ remains unchanged by sorting

# 2.2 Sorting Arrays

Classification of _in situ_ sorting methods:

1. Insertion
2. Selection
3. Exchange

```
type index = 0..n;
var a: array[1..n] of item
```

## 2.2.1 Sorting by Straight Insertion

Items conceptually divided into two sequences:

- Destination: _a<sub>i</sub>...a<sub>i-1</sub>_
- Source: _a<sub>i</sub>...a<sub>n</sub>_

Each step *i*th element of source is picked and transferred into destination by **inserting** it at the correct place

- Starting `i = 2`
- `i` increments each step

Finding correct place alternates between **comparisons and moves**

- Comparisons = **sift** _x_ down by comparing _x_ with next item _a<sub>j</sub>_
- Moves = Inserting _x_ or moving _a<sub>j</sub>_ to the right and proceeding the the left

**Sifting** termination conditions:

- Item _a<sub>j</sub>_ found with key <= key of _x_
- Left end of destination seq is reached
  - **Sentinel item** _a<sub>0</sub> = x_ used to avoid having to check `j > 0` condition

Considers _one_ next item of the _source_ sequence and _all_ items of the _destination_ array to find insertion point.

```
for i := 2 to n do
    begin x := a[i];
        "insert x at the appropriate place in a[1:i]"
    end
```

```
procedure straightinsertion;
    var i,j: index; x:item;
begin
    for i:= 2 to n do
    begin x := a[i]; a[0] := x; j := i - 1;
        while x.key < a[k].key do
        begin a[j + 1] := a[j]; j := j - 1;
        end;
    a[j + 1] := x
    end
end
```

See [straight_insertion.py](./code/straight_selection.py)

### Analysis

_C<sup>i</sup>_ = number of key comparisons in the *i*th sift

- At most `i - 1`, at least `1` and `i/2` on average (assuming all permutations of _n_ keys are equally probable)

_M<sup>i</sup>_ = number moves (assignments of items) in the *i*th sift

- _C<sub>i</sub> + 2_ including sentinel

- _C<sub>min</sub> = n - 1_
- _C<sub>avg</sub> = 1/4(n<sup>2</sup> + n - 2)_
- _C<sub>max</sub> = 1/2(n<sup>2</sup> + n) - 1_

- _M<sub>max</sub> = 2(n - 1)_
- _M<sub>ave</sub> = 1/4(n<sup>2</sup> + 9n - 10)_
- _M<sub>max</sub> = 1/2(n<sup>2</sup> + 3n - 4)_

Already sorted list -> _C<sub>min</sub>/M<sub>min</sub>_
Reverse order list -> _C<sub>max</sub>/M<sub>max</sub>_

## 2.2.2 Sorting by Straight Selection

1. Select the item with the least key
2. Exchange it with the first item a<sub>1</sub>
3. Repeat w/ remaining _n - 1_, then _n -2_... items until one item (largest) is left.

Considers _all_ items of the _source_ array to find item with least key and place it as the _one_ next item of _destination_ sequence.

```
for i := 1 to n - 1 do
    begin "assign index of least item of a[i:n] to k";
    "exchange a[i] and a[k]"
```

See [straight_selection.py](./code/straight_selection.py)

```
procedure straightselection;
    var i, j, k: index; x: item;
begin for i := 1 to n - 1 do
    begin k := i; x := a[i];
        for j := i + 1 to n do
            if a[j].key < x.key then
            begin k:= j; x:= a[j]
            end;
            a[k] := a[i]; a[i] := x;
    end
end
```

### Analysis

- _C = 1/2(n<sup>2</sup> - n)_

  - **Independent** of initial key ordering

- _M<sub>min</sub>3(n - 1)_
  - Keys initially ordered
- _M<sub>max</sub>trunc(n<sup>2</sup>/4) + 3(n - 1)_
  - Keys initially reverse ordered
- _M<sub>ave</sub>_ -> look in book pg 64 fig 2.7 (complex harmonic numbers that I'm not making note of :smile:)

In general better than straight insertion, unless keys are initially sorted/almost sorted in which case straight insertion is slightly faster.

## 2.2.3 Sorting by Straight Exchange

Compare _and_ exchange pairs of **adjacent** items until sorted.

See [straight_exchange.py](./code/straight_exchange.py)

```
procedure bubblesort;
    var i, j: index; x: item;
begin for i := 2 to n do
    begin for j := n downto i do
        if a[j - 1].key > a[j].key then
        begin x := a[j - 1]; a[j - 1]: = a[j]; a[j] := x
        end
    end
end {bubblesort}
```

Can be improved by tracking whether any exchanges made in the previous pass -> list sorted -> terminate

```
procedure bubblesort;
    var i, j: index; x: item; swapped: boolean;
begin for i := 2 to n do
    swapped = false
    begin for j := n downto i do
        if a[j - 1].key > a[j].key then
        begin x := a[j - 1]; a[j - 1]: = a[j]; a[j] := x; swapped = true;
        end
    end
    if not swapped then
        break;
end {bubblesort}
```

### Analysis

- _C = 1/2(n<sup>2</sup> - n)_

- _M<sub>min</sub> = 0_
- _M<sub>ave</sub> = 3/4(n<sup>2</sup> - n)_
- _M<sub>max</sub> = 3/2(n<sup>2</sup> - n)_

## Insertion Sort by Diminishing Increment

Generalization of insertion sort known as **shell sort**

Allows exchange of items that are **far apart**

Each iteration arranges elements so that the sequence of every *h*th element is a sorted list

- An _h_-sorted list
- Performs insertion sort on the _h_ list

The entire array is therefore _h_ interleaved, individually sorted lists

_h_ decreases on each iteration, down to 1; at which point a 'normal' sort is performed

- Ending with _h=1_ **guarantees** the final result will be sorted

Large amounts of _disorder_ are reduced quickly by the larger _h_ sorts, leaving less
work for the smaller _h_ sorts

**Not stable**

Different sequences of _h_ (gap sequences) have been proposed.

- Any sequence ending with _h=1_ will be _correct_
- Different sequences however will have different performance characteristics

See [shell_sort.py](./code/shell_sort.py)

- Includes examples of different gap sequences

## 2.2.5 Tree Sort (Heapsort)

Improved version of straight selection using a **heap** instead of a linear-time search to identify the next sorted element.

Sorting concept is the same as straight selection:

- Array divided into sorted and unsorted sections
- Iteratively shrink unsorted region by identifying the next item in order and placing it into the sorted region

Difference is using a **heap** to maintain the unsorted elements of the array:

- Heap allows more information to be retained through each scan of the array to find the next element than just the identification of the single smallest item.

1. Build a heap from the array to be sorted
2. Repeatedly remove the root element from the heap and insert it into the array
   - Heap is updated after each removal to maintain the _heap property_
   - Once all items removed from heap, the result is a sorted array

Performed in place by splitting the array into two parts:

- Sorted array
  - Grows on each iteration
- Heap
  - Shrinks on each iteration

### Binary Heap

Sequence of keys:

- _h<sub>l</sub>, h<sub>l+1</sub>,...h<sub>r</sub>_

Maintaining the **heap property**:

- Min heap:
  - _h<sub>i</sub> <= h<sub>2i</sub>_
  - _h<sub>i</sub> <= h<sub>2i + 1</sub>_
    - _h<sub>1</sub> = min(h<sub>1</sub>...h<sub>1</sub>)_
- Max heap:
  - _h<sub>i</sub> >= h<sub>2i</sub>_
  - _h<sub>i</sub> >= h<sub>2i + 1</sub>_
    - _h<sub>1</sub> = max(h<sub>1</sub>...h<sub>1</sub>)_

For all _i = l...r/2_

**Assume min heap for following notes**

- Algorithms are the same for max heap except for comparison operations

Maintaining the heap property is by _sifting_ elements to be added along the path of smaller/larger comparands,which at the same time move up the path.

```
procedure sift(l, r: index);
    label 13;
    {i,j = indices of items to be exchanged during each sift step}
    var i, j: index; x: item;
begin i:=l; j := 2 * i; x := a[i];
    while j <= r do
    begin if j < r then
        if a[j].key > a[j + 1].key then j := j + 1;
        if x.key <= a[j].key then goto 13;
        a[i] := a[j]; i := j; j += 2*i {sift}
    end
13: a[i] := x
end
```

#### Building a Min Heap

Min heap can be built _in-situ_, using an array:

- Indexes _floor(n/2) + 1, floor(n/2) + 2, ..., n_ are **leaves** for the tree
  - No two indices _i,j_ are such that _j = 2i_
  - Each is a one-element heap, forming the bottom row of the heap binary tree
- Therefore, take each element of index _<= floor(n/2) + 1_ and use `sift` to arrange it within the heap

```
l := (n div 2) + 1;
while l > 1 do
    begin l := l - 1; sift(l, n)
    end
```

### Full Heapsort Algorithm

```
procedure heapsort;
    var l, r: index; x: item;
    procedure sift(l, r: index);
        label 13;
        {i,j = indices of items to be exchanged during each sift step}
        var i, j: index; x: item;
    begin i:=l; j := 2 * i; x := a[i];
        while j <= r do
        begin if j < r then
            if a[j].key > a[j + 1].key then j := j + 1;
            if x.key <= a[j].key then goto 13;
            a[i] := a[j]; i := j; j += 2*i {sift}
        end
    13: a[i] := x
    end
begin l := (n div 2) + 1;
    {build heap from array a}
    while l > 1 do
        begin l := l - 1; sift(l, n)
        end
    {repeatedly remove items from heap into sorted array}
    while r > 1 do
        begin x:= a[1]; a[1]: a[r]; a[r] := x;
            r := r-1; sift
        end
end {heapsort}
```

### Analysis

_On(log n)_

- `sift` = _O(log n)_
  - Height of binary tree = _log n_
  - Worst case = move item from root down the height of tree to a leaf
- Building initial min heap = _On/2(log n) ~ n log n_
  - Call `sift` for _n/2_ items
    - See [Building a Min Heap](#building-a-min-heap)
- Sorting step = *O(nlogn)*
    - *n* elements in array
    - Worst case have to move from root to leaf = *log n*    
- Building and sorting are executed sequentially -> *sum* the complexities -> remain order *n log n*
