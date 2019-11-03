# Kata08: Conflicting Objectives

Find all **six letter** words which are composed of two concatenated smaller words in the previously used dictionary (`data/wordlist.txt`).

```
al + bums => albums
bar + ely => barely
be + foul => befoul
con + vex => convex
here + by => hereby
jig + saw => jigsaw
tail + or => tailor
we + aver => weaver
```

Not specified in text, but I'm _assuming_ that concatenations are of **unique words**

- i.e. not two of the same word

Implement 3 times with the following priorities:

1. Readability
2. Speed
3. Extendibility

## Readable Version

Filtering pipeline approach.

1. Find all length 6 words
2. Find all combinations
3. Filter combinations by concatenated length == 6
4. Filter combinations by concatenated string is a word in dictionary
5. Print out results

**Problem**: Step 2 will produce a **huge** number of results and this algorithm
will take an extremely long time:

- total combinations = `n! / r! / (n-r)!`
- `n = len(dictionary) = 338882`
- `r = 2`
- `338882!` is a VERY BIG NUMBER

Only create combinations for words that are of length < 6

- Bigger length words cannot possibly concatenate to make a words of length 6

1. Find all length 6 words
2. Find all words < length 6
3. Find combinations on < length 6 words
4. Filter combinations by concatenated length == 6
5. Filter combinations by concatenated string is a word in dictionary
6. Print out results

```bash
$ time python3 readable.py > /dev/null
python3 readable.py > /dev/null  102.35s user 0.00s system 99% cpu 1:42.37 total
```

## Fast Version

Calculating combinations, filtering combinations and repeated filtering of the input dictionary are the most expensive parts of the readable algorithm.

- TODO: Calculate complexities

To remove brute force checking combinations of words < length 6, each length 6 word could be split into all possible pairs of substrings. Each substring can be tested for presence in the input dictionary. Taking `albums` from the example data:

```
a + lbums -> first in dictionary
al + bums -> **both in dictionary**
alb + ums -> first in dictionary
albu + ms -> none in dictionary
album + s -> **both in dictionary**
```

Initially reading the input text into a `set` provides _O(1)_ tests for membership.

For each word there are _O(n-1)_ substring pairs

- TODO: Calculate complexities

```bash
time python3 fast.py > /dev/null
python3 fast.py > /dev/null  0.19s user 0.02s system 99% cpu 0.208 total
```
