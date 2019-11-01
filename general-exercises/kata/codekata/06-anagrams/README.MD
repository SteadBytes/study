# Kata06: Anagrams

Objectives:

1. Output list of anagrams per line
2. Find longest words that are anagrams
3. Find set of anagrams containing the most words

Finding an anagram:

1. Lower case and sort two strings
2. Compare if equal
   TODO: Are there improvements to be made here?

Core Data Structures:

- Input = Sequence of words (strings)
- Process input into a dictionary of `{word: {anagrams}}`
  - Fast equality test to find if a word is an anagram using method described
    above -> is string already in dictionary?
  - Find longest anagram using `max(anagram_dict, key=len)`
  - Find set of anagrams containing most words using
    `max(anagram_dict.values(), key=len))`
