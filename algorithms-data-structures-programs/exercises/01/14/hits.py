#! /usr/bin/env python3.7

import json
import sys
from collections import Counter
from pathlib import Path
from typing import Callable, Dict, List, Tuple


def hit_totals(responses: List[dict]) -> Dict[int, int]:
    """
    Return dict of {hit_number: count}
    """
    choices = (h for r in responses for h in r["choice"])
    return Counter(choices)


def most_popular_hits(responses: List[dict]) -> List[Tuple[int, int]]:
    return sorted(list(hit_totals(responses).items()), key=lambda x: x[1], reverse=True)


def categorise_dicts(dicts: List[dict], **categories: Callable[[dict], bool]) -> dict:
    """
    Returns `dicts` categorised into a dict with categorie names as keys and list
    of `dict` matching that category. `categories` should provide a predicate function,
    called with an item from `dicts` to determine if it matches that category.

    Categories are matched greedily, and `ValueError` is raised if no match is
    found.

    Example:
        >>> data = [
        ...     {"age": 25, "sex": "male"},
        ...       {"age": 18, "sex": "female"},
        ...        {"age": 15, "sex": "male"},
        ...        {"age": 30, "sex": "female"},
        ...    ]

        >>> result = categorise_dicts(
        ...        data,
        ...        male_up_to_20=lambda r: r["sex"] == "male" and r["age"] <= 20,
        ...       male_over_20=lambda r: r["sex"] == "male" and r["age"] > 20,
        ...      female_up_to_20=lambda r: r["sex"] == "female" and r["age"] <= 20,
        ...       female_over_20=lambda r: r["sex"] == "female" and r["age"] > 20,
        ...    )
        >>> result == {
        ...     "male_up_to_20": [{"age": 15, "sex": "male"}],
        ...     "male_over_20": [{"age": 25, "sex": "male"}],
        ...     "female_up_to_20": [{"age": 18, "sex": "female"}],
        ...     "female_over_20": [{"age": 30, "sex": "female"}],
        ...  }
        True
    """
    r = {name: [] for name in categories}
    for d in dicts:
        try:
            name, pred = next((n, p) for n, p in categories.items() if p(d) is True)
            r[name].append(d)
        except StopIteration:
            raise ValueError(f"No matching category provided for {r}")
    return r


def categorise_responses(responses: List[dict]) -> dict:
    """
    Split responses into four categories; by sex, then age less than or equal to
    20 and older than 20 (see tests for example).
    """
    return categorise_dicts(
        responses,
        male_up_to_20=lambda r: r["s"] == "male" and r["age"] <= 20,
        male_over_20=lambda r: r["s"] == "male" and r["age"] > 20,
        female_up_to_20=lambda r: r["s"] == "female" and r["age"] <= 20,
        female_over_20=lambda r: r["s"] == "female" and r["age"] > 20,
    )


def to_table(data: List, headers: List, col_sep="|", header_sep="-") -> str:
    """
    Build an ascii style table string from `data` using `headers` as first row.
    Table contents are left justified, with equal column sizes according to the longest
    item in the table.

    Example:
        >>> to_table([[1, 2, 3], [4, 5, 6]], ["First", "Second", "Third"])
        'First    |Second   |Third    \\n-----------------------------\\n1        |2        |3        \\n4        |5        |6        '
    """
    data = data if isinstance(data, list) else list(data)
    width = max(max(len(str(x)) for x in data), max(len(x) for x in headers))
    table = []
    for i, d in enumerate([headers] + data):
        l = col_sep.join(str(x).ljust(width) for x in d)
        table.append(l)
        if i == 0:
            table.append(header_sep * len(l))
    return "\n".join(table)


def main(f: Path):
    with f.open() as f:
        responses = json.load(f)

    # 1
    # calculate totals across all categories for each hit
    # build descending sorted list of (hit, count)
    print("Most Popular Hits:")
    print(to_table(most_popular_hits(responses), ["Hit", "Count"]))
    print()

    # 2
    responses_by_category = categorise_responses(responses)
    totals_by_category = {k: hit_totals(v) for k, v in responses_by_category.items()}
    top_3_by_category = {
        k: dict(v.most_common(3)) for k, v in totals_by_category.items()
    }
    # build list of (name, first name) where respondent's first choice was one
    # of the 3 top hits in their respective category
    preffered_in_top_3 = {
        k: [
            (r["name"], r["firstname"])
            for r in v
            if r["choice"][0] in top_3_by_category[k]
        ]
        for k, v in responses_by_category.items()
    }
    for category, results in preffered_in_top_3.items():
        print(category.replace("_", " ").title() + ":")
        print(to_table(results, ["Name", "Firstname"]))
        print()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage ./hits.py [RESPONSES_JSON_FILE]")
        exit(1)
    main(Path(sys.argv[1]))
