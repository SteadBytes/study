#! /usr/bin/env python3

"""
Scans source files for camelCase strings and reports on their locations.
Yes I'm aware this can be achieved with a grep one-liner.
"""

import re
import sys
from pathlib import Path
from typing import Generator, Iterable, List, NamedTuple, Tuple

# matches valid camelCase strings
# see tests for examples of matching/non-matching strings
CAMEL_RE = re.compile(r"\b[a-z]+((\d)|([A-Z0-9][a-z0-9]+))+([A-Z])?")


def err_exit(msg, status=1):
    print(msg)
    exit(status)


class MatchGroup(NamedTuple):
    start: int
    end: int


class Match(NamedTuple):
    lineno: int
    groups: List[MatchGroup]
    line: str


def pretty_match(m: Match, filename: str = None):
    pretty_name = f"\033[35m{filename}\033[m:" if filename else ""
    l = []
    prev = 0
    for g in m.groups:
        l.append(f"{m.line[prev:g.start]}\033[31m{m.line[g.start:g.end]}\033[m")
        prev = g.end
    l.append(m.line[prev:])
    return "".join([f"{pretty_name}\033[32m{m.lineno}\033[m:"] + l)


def find_camel(lines: Iterable[str]) -> Generator[MatchGroup, None, None]:
    for i, line in enumerate(lines):
        groups = [MatchGroup(*m.span()) for m in CAMEL_RE.finditer(line)]
        if groups:
            yield Match(i, groups, line)


def main(files: List[Path]):
    show_filenames = len(files) > 1
    for file in files:
        with file.open() as f:
            for m in find_camel(f):
                print(pretty_match(m, filename=file if show_filenames else None))


if __name__ == "__main__":
    if len(sys.argv) == 1:
        err_exit("Usage: ./camel_finder.py FILES...")

    main([Path(f) for f in sys.argv[1:]])
