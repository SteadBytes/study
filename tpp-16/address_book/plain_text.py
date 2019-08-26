import json
from dataclasses import asdict
from functools import partial
from pathlib import Path
from typing import Iterable

from .models import Person


def from_dict(d: dict) -> Person:
    return Person(**d)


def to_dict(p: Person) -> dict:
    return asdict(p)


def read_address_book(db: Path):
    with db.open() as f:
        return [from_dict(d) for d in json.load(f)]


def write_address_book(db: Path, people: Iterable[Person]):
    with db.open("w") as f:
        json.dump([to_dict(p) for p in people], f)
