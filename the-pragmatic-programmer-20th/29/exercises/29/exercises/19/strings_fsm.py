#! /bin/usr/env python3
"""
Simple Finite State Machine for extracting quoted strings within a stream of
text.

Translated into Python and adapted slightly from event/strings_fsm.rb on page
141.
"""
import sys
from enum import Enum, auto

import pytest


class State(Enum):
    DEFAULT = auto()
    IGNORE = auto()
    LOOK_FOR_STRING = auto()
    START_NEW_STRING = auto()
    IN_STRING = auto()
    COPY_NEXT_CHAR = auto()
    ADD_CURRENT_TO_STRING = auto()
    FINISH_CURRENT_STRING = auto()


TRANSITIONS = {
    State.LOOK_FOR_STRING: {
        '"': [State.IN_STRING, State.START_NEW_STRING],
        State.DEFAULT: [State.LOOK_FOR_STRING, State.IGNORE],
    },
    State.IN_STRING: {
        '"': [State.LOOK_FOR_STRING, State.FINISH_CURRENT_STRING],
        "\\": [State.COPY_NEXT_CHAR, State.ADD_CURRENT_TO_STRING],
        State.DEFAULT: [State.IN_STRING, State.ADD_CURRENT_TO_STRING],
    },
    State.COPY_NEXT_CHAR: {
        State.DEFAULT: [State.IN_STRING, State.ADD_CURRENT_TO_STRING]
    },
}


def find_strings(char_stream):
    state = State.LOOK_FOR_STRING
    result = []

    for ch in char_stream:
        state, action = TRANSITIONS[state].get(ch, TRANSITIONS[state][State.DEFAULT])
        if action == State.IGNORE:
            continue
        elif action == State.START_NEW_STRING:
            result = []
        elif action == State.ADD_CURRENT_TO_STRING:
            result.append(ch)
        elif action == State.FINISH_CURRENT_STRING:
            yield "".join(result)


def getc(f=sys.stdin):
    """
    Returns a generator of *characters* in file-like *f*. Note, if `f` is line
    buffered (such as the default `sys.stdin`) this will not begin yielding results
    until the full line is read.
    """
    return (ch for l in f for ch in l)


if __name__ == "__main__":
    for s in find_strings(getc()):
        print(s)
