from collections import defaultdict
from functools import partial
from itertools import count, islice
from math import floor
from random import randint, shuffle
from time import time
from typing import Iterable, List

from shell_sort import shell_sort


def few_unique_keys():
    """
    Random values in range 0...50, multipled by 10^8
    """
    while True:
        yield randint(0, 50) * pow(10, 8)


def many_unique_keys():
    """
    Values in range 0...10^9
    """
    while True:
        yield randint(0, pow(10, 9))


def small_keys():
    """
    Values in range 0...50
    """
    while True:
        yield randint(0, 50)


def grouped(iterable, n):
    "s -> (s0,s1,s2,...sn-1), (sn,sn+1,sn+2,...s2n-1), (s2n,s2n+1,s2n+2,...s3n-1), ..."
    return zip(*[iter(iterable)] * n)


def mostly_sorted(l: List):
    _l = []
    for g in grouped(sorted(l), 10):
        shuffle(list(g))
        _l += g
    return _l


def unsorted(l: List):
    _l = l[:]
    shuffle(_l)
    return _l


def reverse(l: List):
    return list(reversed(l))


KEY_GENERATORS = [few_unique_keys, many_unique_keys, small_keys]
INITIAL_ORDERINGS = [unsorted, sorted, reverse, mostly_sorted]


def dd():
    return defaultdict(dd)


def bench_sort(sort_fn, ns):
    r = dd()
    for key_gen in KEY_GENERATORS:
        for n in ns:
            l = list(islice(key_gen(), n))
            for f in INITIAL_ORDERINGS:
                print(
                    f"key_gen={key_gen.__name__}, initial_ordering={f.__name__}, n={n}"
                )
                _l = f(l)
                times = []
                for i in range(10):
                    _l_copy = _l[:]
                    start = time()
                    sort_fn(_l_copy)
                    end = time()
                    duration = end - start
                    times.append(duration)
                    print(f"\tLoop {i}: {duration}")
            r[key_gen.__name__][f.__name__][n] = sum(times) / len(times)
    return r


def perf_shell_sort():
    r = {}
    for gap_fn in [original_gap]:
        r[gap_fn.__name__] = bench_sort(
            lambda l: shell_sort(l, gap_fn), [100, 10000, 1000000]
        )
    return r


def perf_builtin_sort():
    return bench_sort(sorted, [100, 10000, 1000000])
