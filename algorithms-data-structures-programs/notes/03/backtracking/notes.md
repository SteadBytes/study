# Backtracking Algorithms

General problem solving - find solution so specific problems by **trial and error**

- Not following a fixed rule of computation

Enumerates a set of partial candidates that _could_ be completed in various ways to give all possible solutions to a given problem. When a candidate fails, the algorithm _backtracks_ to the previous candidate.

General pattern (assumes **finite** potential candidates at each step) to find a _single solution_:

```
procedure try;
begin initialize selection of candidates;
    repeat select next;
        if acceptable then
        begin record it;
            if solution incomplete then
            begin try next step;
                if not successful then cancel recording
            end
        end
    until successful V no more candidates
end
```

With a fixed number of candidates _m_:

```
procedure try(i: integer);
    var k: integer;
begin k: = 0;
    repeat k := k + 1; select k-th candidate`;
        if acceptable then
        begin record it;
            if i < n then
            begin try(i + 1);
                if not successful then cancel recording
            end
        end
    until successful V (k = m)
end
```

General pattern to find _all solutions_:

```
procedure try(i: integer);
    var k: integer;
begin;
    for k := 1 to m do
    begin select k-th candidate;
        if acceptable then
        begin record it;
            if i < n then try(i + 1) else print solution;
                cancel recording
        end
    end
end
```

General pattern to find _the optimum selection_ out of a given set of objects subject to some constraints:

```
procedure try(i: integer);
begin
1: if inclusion is acceptable then
    begin include i-th object;
        if i < n then try(i + 1) else check optimality;
        eliminate i-th object
    end;
2: if exclusion is acceptable then
    if i < n then try(i + 1) else check optimality
end
```

- Consideration of each object has _2 possible outcomes_
  - _Inclusion_ in the current selection
  - _Exclusion_ from the current selection
- Objects numbered _1,2,...n_
- _2<sup>n</sup>_ possible sets

  - Acceptability criteria is important to reduce the number of investigated candidates

- To find the _optimal solution_ from a _backtracking_ search, each object would be a successful candidate solution from the search.

**Branch and bound algorithm** = backtracking search with a limitation factor reducing the growth of the potential search tree
