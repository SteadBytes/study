import pytest

from address_book import binary
from address_book.models import Person, generate_people


@pytest.fixture
def people_list():
    return list(generate_people(50))


class TestBinary:
    @pytest.mark.parametrize("p", generate_people(50))
    def test_to_bytes_inverts_from_bytes(self, p):
        p_bytes, size = binary.to_bytes(p)
        p_again = binary.from_bytes(p_bytes)
        assert p == p_again

    def test_write_address_book_inverts_read_address_book(self, tmp_path, people_list):
        db = tmp_path / "address-book.bin"
        # sanity check
        assert db.exists() is False

        binary.write_address_book(db, people_list)

        assert db.exists() is True
        assert db.stat().st_size > 0

        people_list_again = binary.read_address_book(db)

        assert people_list == people_list_again
