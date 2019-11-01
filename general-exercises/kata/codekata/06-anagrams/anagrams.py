from typing import Iterable, Dict, Set
from collections import defaultdict

AnagramIndex = Dict[str, Set[str]]


def build_index(words: Iterable[str]) -> AnagramIndex:
    index = defaultdict(set)  # avoid duplicates w/set
    for word in words:
        _word = "".join(sorted(word.lower()))
        index[_word].add(word)

    return index


def longest_anagram(index: AnagramIndex) -> Set[str]:
    return index[max(index, key=len)]


def most_anagrams(index: AnagramIndex) -> str:
    return max(index.values(), key=len)


if __name__ == "__main__":
    with open("data/wordlist.txt", encoding="iso-8859-1") as f:
        index = build_index((l.strip() for l in f))

    for ws in index.values():
        print(*ws)

    print()

    print(f"Longest anagram: {longest_anagram(index)}")
    most = most_anagrams(index)
    print(f"Most anagrams: {most} ({len(most)})")

    # TODO: Use this list for testing?
    # words = [
    #     "kinship",
    #     "pinkish",
    #     "enlist",
    #     "inlets",
    #     "listen",
    #     "silent",
    #     "boaster",
    #     "boaters",
    #     "borates",
    #     "fresher",
    #     "refresh",
    #     "sinks",
    #     "skins",
    #     "knits",
    #     "stink",
    #     "rots",
    #     "sort",
    # ]
