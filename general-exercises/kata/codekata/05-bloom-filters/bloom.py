from hashlib import md5
from itertools import zip_longest

N_HASHES = 8


def bitmap(m: int) -> list:
    return [0 for _ in range(m)]


def grouper(iterable, n, fillvalue=None):
    "Collect data into fixed-length chunks or blocks"
    # grouper('ABCDEFG', 3, 'x') --> ABC DEF Gxx"
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=fillvalue)


def hashes(x: str, n: int):
    h = md5(x.encode("utf-8")).hexdigest()
    return ("".join(_h) for _h in grouper(h, len(h) // n))


def hash_positions(x: str, m: int):
    for h in hashes(x, N_HASHES):
        yield int(h, 16) % m


def insert(bm: list, *xs: str):
    for x in xs:
        for p in hash_positions(x, len(bm)):
            bm[p] |= 1


def test(bm, x):
    vals = [bm[p] for p in hash_positions(x, len(bm))]
    return all(vals)


def bloom_filter(m: int, iterable=None):
    bm = bitmap(m)
    if iterable:
        insert(bm, *iterable)
    return bm


if __name__ == "__main__":
    from pathlib import Path

    words = Path("/usr/share/dict/words")
    with words.open() as f:
        bm = bloom_filter(1000000, f)

    print(test(bm, "hello"))
    print(test(bm, ";alksdjf"))
    print(test(bm, "llk"))
