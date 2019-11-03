from typing import List, Iterable, Tuple

StringPairs = Iterable[Tuple[str, str]]


def substring_pairs(s: str) -> StringPairs:
    for i in range(1, len(s)):
        yield s[0:i], s[i:]


def compounds(dictionary: List[str], length: int = 6) -> StringPairs:
    all_words = {w.lower() for w in dictionary}
    for w in (w for w in dictionary if len(w) == length):
        for s1, s2 in substring_pairs(w):
            if s1 in all_words and s2 in all_words:
                yield s1, s2


if __name__ == "__main__":
    with open("data/wordlist.txt", encoding="iso-8859-1") as f:
        dictionary = [l.strip() for l in f]
    for w1, w2 in compounds(dictionary):
        print(f"{w1} + {w2} => {w1 + w2}")
