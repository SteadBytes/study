# Kata05: Bloom Filters

Bloom filter based spell checker.

To test the error rate (false positives), run `pytest` with [Hypothesis test statistics](https://hypothesis.readthedocs.io/en/latest/details.html#test-statistics) and check the 'False positive' event output:

```
pytest --hypothesis-show-statistics
============================== test session starts ===============================
test_error.py .                                                            [100%]
============================= Hypothesis Statistics ==============================

test_error.py::test_spell_check_false_positives:

  - 1000 passing examples, 0 failing examples, 1 invalid examples
  - Typical runtimes: 0-1 ms
  - Fraction of time spent in data generation: ~ 58%
  - Stopped because settings.max_examples=1000
  - Events:
    *   0.10%, False positive

=============================== 1 passed in 1.55s ================================
```
