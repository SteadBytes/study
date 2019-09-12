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