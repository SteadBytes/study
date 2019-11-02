from collections import defaultdict
from typing import DefaultDict, Iterable, Set

AnagramIndex = DefaultDict[str, Set[str]]


def build_index(words: Iterable[str]) -> AnagramIndex:
    index = defaultdict(set)  # avoid duplicates
    for word in words:
        _word = "".join(sorted(word.lower()))
        index[_word].add(word)

    return index


def _anagram_items(index: AnagramIndex):
    return ((k, ws) for k, ws in index.items() if len(ws) > 1)


def all_anagrams(index: AnagramIndex):
    return (ws for _, ws in _anagram_items(index))


def longest_anagram(index: AnagramIndex) -> Set[str]:
    try:
        return max(_anagram_items(index), key=lambda x: len(x[0]))[1]
    except ValueError:
        return None


def most_anagrams(index: AnagramIndex) -> str:
    try:
        return max(_anagram_items(index), key=lambda x: len(x[1]))[1]
    except ValueError:
        return None


if __name__ == "__main__":
    with open("data/wordlist.txt", encoding="iso-8859-1") as f:
        index = build_index((l.strip() for l in f))

    for ws in all_anagrams(index):
        print(*ws)

    print()

    print(f"Longest anagram: {longest_anagram(index)}")
    most = most_anagrams(index)
    print(f"Most anagrams: {most} ({len(most)})")
