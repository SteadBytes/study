import hashlib
import math
from struct import unpack

FMT_SIZES = {"Q": 8, "I": 4, "H": 2}


def _choose_hash(hash_bits: int):
    # TODO: document
    # TODO: check these values
    # TODO: non-cryptographic hashes for perf improvement?
    if hash_bits > 384:
        return hashlib.sha512
    elif hash_bits > 256:
        return hashlib.sha384
    elif hash_bits > 160:
        return hashlib.sha256
    elif hash_bits > 128:
        return hashlib.sha1
    else:
        return hashlib.md5


def _make_hashes_f(k, m):
    # TODO: document
    # TODO: check these values
    if m >= (1 << 31):
        fmt_char = "Q`"
    elif m >= (1 << 15):
        fmt_char = "I"
    else:
        fmt_char = "H"

    hash_bits = 8 * k * FMT_SIZES[fmt_char]
    _hash_f = _choose_hash(hash_bits)
    fmt = fmt_char * (_hash_f().digest_size // FMT_SIZES[fmt_char])

    def _hashes_f(data: bytes):
        h = _hash_f(data)
        for _int in unpack(fmt, h.digest())[:k]:
            yield _int % m

    return _hashes_f


class BloomFilter:
    def __init__(self, n: int, iterable=None, error_rate=0.01):
        # TODO: rename n, m, k & document var names for comparison to literature
        # TODO: formulas
        self.m = math.ceil(-n * math.log(error_rate) / (math.log(2) ** 2))
        self.k = math.ceil(self.m / n * math.log(2))
        self._hashes_f = _make_hashes_f(self.k, self.m)
        # TODO: more efficient bitmap representation
        self.bm = [0 for _ in range(self.m)]

        if iterable:
            self.add(*iterable)

    def add(self, *vals: str):
        for v in vals:
            for p in self._hash_positions(v):
                self.bm[p] |= 1

    def _hash_positions(self, val: str):
        yield from self._hashes_f(val.encode("utf-8"))

    def __contains__(self, val: str):
        return all(self.bm[p] for p in self._hash_positions(val))


if __name__ == "__main__":
    with open("/usr/share/dict/words") as f:
        words = [l.strip() for l in f]
        bf = BloomFilter(len(words), iterable=words)

    print("hello" in bf)
    print("test" in bf)
    print("ljasdkf" in bf)
