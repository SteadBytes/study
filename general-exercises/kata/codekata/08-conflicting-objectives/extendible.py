from itertools import chain, combinations, tee
from typing import Iterable, Sequence, Set, Tuple


def pairwise(iterable: Iterable):
    """
    s -> (s0,s1), (s1,s2), (s2, s3), ...
    """
    a, b = tee(iterable)
    next(b, None)
    return zip(a, b)


def n_splits(s: Sequence, n: int) -> Iterable[Tuple[str, ...]]:
    """
    Generate all possible in-order n-splits of `s`:

    >>> list(n_splits("abc", 2))
    [['a', 'bc'], ['ab', 'c']]

    >>> list(n_splits("abc", 3))
    [['a', 'b', 'c']]

    >>> list(n_splits("abcd", 3))
    [['a', 'b', 'cd'], ['a', 'bc', 'd'], ['ab', 'c', 'd']]
    """
    if n > len(s) or n <= 0:
        # short circuit when no possible splits
        return
    else:
        for splits in combinations(range(1, len(s)), n - 1):
            yield tuple(s[s1:s2] for s1, s2 in pairwise(chain([None], splits, [None])))


def compounds(
    dictionary: Set[str], length: int = 6, n: int = 2
) -> Iterable[Tuple[str, ...]]:
    for w in (w for w in dictionary if len(w) == length):
        yield from (s for s in n_splits(w, n) if set(s) <= dictionary)


if __name__ == "__main__":
    with open("data/wordlist.txt", encoding="iso-8859-1") as f:
        dictionary = {l.strip().lower() for l in f}
    for ws in compounds(dictionary):
        print(f"{' + '.join(ws)} => {''.join(ws)}")
