1. `Mood`
2. `Blah` or `Woot`
3. `changeMood :: Mood -> Woot` will only produce the correct result when it's argument is `Blah`; given `Woot` it returns `Woot` instead of `Blah`.
4. See [`mood_swings.hs`](./mood_swings.hs)
