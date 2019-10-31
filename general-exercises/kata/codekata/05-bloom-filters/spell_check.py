import hashlib
import math
from pathlib import Path
from struct import unpack
from typing import Callable, Generator

WORDS_PATH = Path("/usr/share/dict/words")

# https://docs.python.org/3.7/library/struct.html#format-characters
FMT_BYTE_SIZES = {"Q": 8, "I": 4, "H": 2}
# https://en.wikibooks.org/wiki/C_Programming/limits.h
FMT_MAX_VALS = {"Q": 1 << 63, "I": 1 << 31, "H": 1 << 15}


def _choose_hash(hash_bits: int) -> Callable:
    """
    Return a `hashlib` hash function of minimum digest size such that
    `hash_bits <= digest_size_bits`.
    """
    # TODO: non-cryptographic hashes for perf improvement?
    if hash_bits > 512:
        raise ValueError("hash_bits must be < 512")
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


def _make_hashes_f(k: int, m: int) -> Callable[[bytes], Generator[int, None, None]]:
    """
    Return a function which takes a `bytes` argument and yields `k` integers
    within `range(len(m))` produced by hashing the input `bytes`.
    """
    # choose format char such that hash values can reach len(m)
    if m >= FMT_MAX_VALS["Q"]:
        # that's a reaaaaaaaaally big bloom filter
        raise ValueError(f"m <= {FMT_MAX_VALS['Q']}")
    elif m >= FMT_MAX_VALS["I"]:
        fmt_char = "Q`"
    elif m >= FMT_MAX_VALS["H"]:
        fmt_char = "I"
    else:
        fmt_char = "H"

    hash_bits = 8 * k * FMT_BYTE_SIZES[fmt_char]
    _hash_f = _choose_hash(hash_bits)
    fmt = fmt_char * (_hash_f().digest_size // FMT_BYTE_SIZES[fmt_char])

    def _hashes_f(data: bytes):
        h = _hash_f(data)
        for _int in unpack(fmt, h.digest())[:k]:
            yield _int % m

    return _hashes_f


class BloomFilter:
    def __init__(self, n: int, iterable=None, error_rate=0.001):
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
        """
        Yield `self.k` positions in `self.bm` produced by hashing `val`.
        """
        yield from self._hashes_f(val.encode("utf-8"))

    def __contains__(self, val: str):
        return all(self.bm[p] for p in self._hash_positions(val))


with WORDS_PATH.open() as f:
    words = [l.strip() for l in f]
    bf = BloomFilter(len(words), iterable=words)


def check(word: str):
    return word in bf
