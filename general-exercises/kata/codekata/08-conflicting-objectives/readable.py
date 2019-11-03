from itertools import combinations
from typing import List


def compounds(dictionary: List[str], n=2, length=6):
    full_length_words = {w.lower() for w in dictionary if len(w) == length}
    concatenation_candidates = (w.lower() for w in dictionary if len(w) < length)
    n_combinations = (
        c
        for c in combinations(concatenation_candidates, n)
        if sum(len(w) for w in c) == length
    )
    return (c for c in n_combinations if "".join(c) in full_length_words)


if __name__ == "__main__":
    with open("data/wordlist.txt", encoding="iso-8859-1") as f:
        dictionary = [l.strip() for l in f]
    for w1, w2 in compounds(dictionary):
        print(f"{w1} + {w2} => {w1 + w2}")
