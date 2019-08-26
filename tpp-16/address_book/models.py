from dataclasses import dataclass, field
from typing import Iterable
from uuid import uuid4

from faker import Faker

fake = Faker("en_GB")


@dataclass
class Person:
    first_name: str
    last_name: str
    phone_number: str
    house_number: str
    street: str
    town: str
    postcode: str
    directions: str
    id: str = field(default_factory=lambda: str(uuid4()))


def generate_people(n: int) -> Iterable[Person]:
    for _ in range(n):
        yield Person(
            fake.first_name(),
            fake.last_name(),
            fake.phone_number(),
            fake.building_number(),
            fake.street_name(),
            fake.city(),
            fake.postcode(),
            fake.text(),  # random latin is about as useful as most directions
        )
