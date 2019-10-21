from typing import Dict, Hashable, Union, List, Tuple, Iterable

State = Hashable
Event = Hashable


class StateMachine:
    def __init__(
        self,
        transitions: Dict[State, Dict[Union[Event, State], List[State]]],
        default_state: State,
        initial_state: State,
    ):
        self.transitions = transitions
        self.default_state = default_state
        self.initial_state = initial_state

    def process(self, events: Iterable[Event]) -> Tuple[State, State, Event]:
        state = self.initial_state
        for event in events:
            state, action = self.transitions[state].get(
                event, self.transitions[state][self.default_state]
            )
            yield state, action, event
