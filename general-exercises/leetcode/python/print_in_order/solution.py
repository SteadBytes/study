from threading import Lock


class Foo:
    def __init__(self):
        self.lock_first = Lock()
        self.lock_second = Lock()

        self.lock_first.acquire()
        self.lock_second.acquire()

    def first(self, printFirst: "Callable[[], None]") -> None:
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.lock_first.release()

    def second(self, printSecond: "Callable[[], None]") -> None:
        self.lock_first.acquire()
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.lock_second.release()

    def third(self, printThird: "Callable[[], None]") -> None:
        self.lock_second.acquire()
        # printThird() outputs "third". Do not change or remove this line.
        printThird()
