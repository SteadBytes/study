#! /usr/bin/env python3
"""
Towers of Hanoi recursive solution.

Algorithm:
To move m disks from source to target peg:
1. Move m - 1 disks from source to spare peg
2. Move disk m from source to targe
3. Move m - 1 disks from the spare onto the target
4. Base case = move 0 disks (do nothing)
"""
import sys


def towers_to_str(towers):
    return "\n".join(f"{k}: {' '.join(str(x) for x in v)}" for k, v in towers.items())


def display(towers):
    print(towers_to_str(towers) + "\n")


def move(m, source, target, spare, towers):
    # sanity check: uncomment to prove no disk placed on top of a smaller one
    # for k, v in towers.items():
    #     assert sorted(v, reverse=True) == v
    if m == 0:
        return
    move(m - 1, source, spare, target, towers)
    towers[target].append(towers[source].pop())
    display(towers)
    move(m - 1, spare, target, source, towers)


def hanoi(n):
    towers = {"A": list(range(n, 0, -1)), "B": [], "C": []}
    display(towers)
    move(n, "A", "C", "B", towers)
    return towers


if __name__ == "__main__":
    try:
        n = int(sys.argv[1])
    except (IndexError, ValueError):
        print("Usage: hanoi.py n_disks", file=sys.stderr)

    hanoi(n)
