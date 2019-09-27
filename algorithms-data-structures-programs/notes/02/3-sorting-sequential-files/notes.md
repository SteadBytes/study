# 2.3 Sorting Sequential Files

Array sorting algorithms from 2.2 are not applicable if the amount of data in a sequence does not fit in memory

- Store as sequential files
- Directly access a single component at a time

## 2.3.1 Straight Merging

**Mergesort**

Combine two or more ordered sequences into a single ordered sequence by repeated selection among currently accessible components.

1. Split sequence `a` into two halves `b` and `c`
2. Merge `b` and `c` by combining single items into _ordered pairs_
3. Call merged sequence `a` and repeat steps 1 and 2 -> merging ordered paris into ordered quadruples
4. Repeat, merging quadruples into octets e.t.c, doubling lengths of merged subsequences until fully ordered

_Phase_ = operation treating the entire set of data once

_Pass/Stage_ = smallest subprocess repeated to perform the sort process

**Two files** are required; the output of each merge process is redistributed onto two files to then be used as the sources of the next pass.

- _Single phase/balanced_ merge

### Array Mergesort

Use _strictly sequential_ scans of the array

- Same as sequential file access

**Single array** can be used instead of two files

- Treat as _double ended_ sequence
- Double the size of the source array
- Pick items off the two ends of the array instead of merging from two separate files

Split into source and destination sub-arrays

- Denote using indices

*a: array[1...2*n] of item\*

_a<sub>i</sub>, ..., a<sub>j</sub>_ = source
_a<sub>k</sub>, ..., a<sub>l</sub>_ = destination

```

    source        dest
+-+-+----+-+-+-+-+----+-+-+
| | |    | | | | |    | | |
+-+-+----+-+-+-+-+----+-+-+
 i          j k          l
```

See [merge_sort.py](./code/merge_sort.py).

```
procedure mergesort;
    var i, j, k, l, t: index;
    h, m, p, q, r: integer; up: boolean;
    {note that a has indices 1...2*n}
begin up := true; p := 1;
    repeat h := 1; m := n;
        if up then
        begin i := 1; j := n; k := n + 1; l := 2 * n
        end else
        begin i := 1; j := n; k := n + 1; l := 2 * n
        end else
        begin k := 1 l := n; i := n + 1; j := 2* n
        end ;
        repeat {merge a run from i and j to k}
            {q = length of i-run, r = length of j-run}
            if m >= p then q := p else q := m; m := m - q;
            if m >= p then r := p else r := m; m := m - r;
            while (q != 0) ^ (r != 0) do
            begin {merge}
                if a[i].key < a[j].key then
                begin a[k] := a[i]; k := k + h; i := i + 1; q := q - 1
                end else
                begin a[k] := a[j]; k := k + h; j := j - 1; r := r - 1
                end
            end ;
            {copy tail of j-run}
            while r != 0 do
                begin a[k] := a[j]; k := k + h; j := j - 1; r := r - 1
                end ;
            {copy tail of i-run}
            while q != 0 do
                begin a[k] := a[i]; k := k + h; i := i + 1; q := q -1
                end ;
            h := -h; t := k; k := l; l := t
        until m = 0;
        up := ¬up; p := 2 * p
    until p >= n;
    if ¬up then
    for i := 1 to n do a[i] := a[i + n]
end {mergesort}
```

### Analysis

**O(n log n)**

- Divide/split is **O(n)**
  - Calculate middle index
- `merge` is **O(n)**
  - Single pass through sub arrays, moving 1 element at a time
- Each recursive call sorts two sublists of size _n/2_
  - Height of tree = _log n_
  - _log n_ recursive calls, each doing _n/2_ work = _n log n_
