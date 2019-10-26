#! /usr/bin/env python3
from enum import Enum
from typing import Generator

from svg import EMPTY_PATH, abs_move, rel_horiz, rel_ver


class Turn(Enum):
    BASE = 1
    UP = 2
    DOWN = 3
    LEFT = 4
    RIGHT = 5


def hilbert(order: int, turn: Turn = Turn.UP) -> Generator[Turn, None, None]:
    if order == 0:
        yield Turn.BASE
    elif order > 0:
        if turn == Turn.UP:
            yield from hilbert(order - 1, Turn.RIGHT)
            yield Turn.UP
            yield from hilbert(order - 1, Turn.UP)
            yield Turn.RIGHT
            yield from hilbert(order - 1, Turn.UP)
            yield Turn.DOWN
            yield from hilbert(order - 1, Turn.LEFT)
        elif turn == Turn.LEFT:
            yield from hilbert(order - 1, Turn.DOWN)
            yield Turn.LEFT
            yield from hilbert(order - 1, Turn.LEFT)
            yield Turn.DOWN
            yield from hilbert(order - 1, Turn.LEFT)
            yield Turn.RIGHT
            yield from hilbert(order - 1, Turn.UP)
        elif turn == Turn.RIGHT:
            yield from hilbert(order - 1, Turn.UP)
            yield Turn.RIGHT
            yield from hilbert(order - 1, Turn.RIGHT)
            yield Turn.UP
            yield from hilbert(order - 1, Turn.RIGHT)
            yield Turn.LEFT
            yield from hilbert(order - 1, Turn.DOWN)
        elif turn == Turn.DOWN:
            yield from hilbert(order - 1, Turn.LEFT)
            yield Turn.DOWN
            yield from hilbert(order - 1, Turn.DOWN)
            yield Turn.LEFT
            yield from hilbert(order - 1, Turn.DOWN)
            yield Turn.UP
            yield from hilbert(order - 1, Turn.RIGHT)


def hilbert_height(order):
    if order == 0:
        return 0
    else:
        return 1 + 2 * hilbert_height(order - 1)


def svg_hilbert(order, unit_length, size=500):
    edge_length = unit_length // hilbert_height(order)

    def up_line():
        return rel_ver(-edge_length, EMPTY_PATH)

    def down_line():
        return rel_ver(edge_length, EMPTY_PATH)

    def left_line():
        return rel_horiz(-edge_length, EMPTY_PATH)

    def right_line():
        return rel_horiz(edge_length, EMPTY_PATH)

    turn_map = {
        Turn.BASE: lambda: EMPTY_PATH,
        Turn.UP: up_line,
        Turn.DOWN: down_line,
        Turn.LEFT: left_line,
        Turn.RIGHT: right_line,
    }

    path = "".join(turn_map[turn]() for turn in hilbert(order))

    d = abs_move(5, unit_length + 5, path)
    viewbox_size = unit_length + 10
    viewbox = f"0 0 {viewbox_size} {viewbox_size}"
    return f"""
    <svg
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        width="{size}"
        height="{size}"
        viewBox="{viewbox}"
    >
        <g stroke="black" fill="white" stroke-width="4">
            <path d="{d}" stroke-linejoin="round"/>
            <circle cx="0" cy="0" r="6" style="fill:red">
                <animateMotion
                    dur="100s"
                    path="{d}"
                    rotate="auto"
                    repeatCount="indefinite"
                />
            </circle>
        </g>
    </svg>"""


if __name__ == "__main__":
    import sys

    try:
        order = int(sys.argv[1])
    except (IndexError, ValueError):
        print(
            "hilbert_curve: Output SVG of a Hilbert curve\nUsage: hilbert_curve order",
            file=sys.stderr,
        )
        exit(1)

    print(svg_hilbert(order, 1000, size=1000))
