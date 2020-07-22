from .solution import Foo
from threading import Thread
from functools import partial
import sys
import pytest

from time import sleep


@pytest.mark.slow
def test_thread_interleaving(capsys):
    # Increase chances of thread interleaving
    sys.setswitchinterval(1e-12)

    # Run multiple times to increase chances of thread interleaving
    for _ in range(100):
        foo = Foo()
        out = []

        def sleep_append(s):
            # Increase chances of thread interleaving
            sleep(0.1)
            out.append(s)

        a = Thread(target=partial(foo.first, partial(sleep_append, "first")))
        b = Thread(target=partial(foo.second, partial(sleep_append, "second")))
        c = Thread(target=partial(foo.third, partial(sleep_append, "third")))

        a.start()
        b.start()
        c.start()

        a.join()
        b.join()
        c.join()

        assert "".join(out) == "firstsecondthird"
