import math
from hashlib import md5
from itertools import zip_longest, islice


def grouper(iterable, n, fillvalue=None):
    "Collect data into fixed-length chunks or blocks"
    # grouper('ABCDEFG', 3, 'x') --> ABC DEF Gxx"
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=fillvalue)


def hashes(data: bytes, k: int):
    h = md5(data).hexdigest()
    n_hashes = math.ceil(len(h) // k)
    return ("".join(_h) for _h in islice(grouper(h, n_hashes), 0, n_hashes + 1))


def hash_positions(data: bytes, k: int, m: int):
    for h in hashes(data, k):
        yield int(h, 16) % m


def error_rate(k, n, m):
    return math.pow(1 - math.pow(math.e, (-k * n) / m), k)


def calculate_m_k(n, target_error_rate, start_m=100):
    # TODO: Stop this loop after a sensible amount of iterations
    m = start_m
    while True:
        k = math.ceil((m / n) * math.log(2))
        if error_rate(k, n, m) <= target_error_rate:
            return m, k
        else:
            m *= 2


class BloomFilter:
    def __init__(self, n: int, iterable=None, error_rate=0.01):
        # TODO: check values of m and k are reasonable?
        self.m, self.k = calculate_m_k(n, target_error_rate=error_rate)
        self.bm = [0 for _ in range(self.m)]

        if iterable:
            self.add(*iterable)

    def add(self, *vals: str):
        for v in vals:
            for p in self._hash_positions(v):
                self.bm[p] |= 1

    def _hash_positions(self, val: str):
        return hash_positions(val.encode("utf-8"), self.k, self.m)

    def __contains__(self, val: str):
        return all(self.bm[p] for p in self._hash_positions(val))


if __name__ == "__main__":
    with open("/usr/share/dict/words") as f:
        words = [l.strip() for l in f]
        bm = BloomFilter(len(words), iterable=words)

    # TODO: FIX THIS
    print("hello" in bm)
    print("elloh" in bm)
