# Kata05: Bloom Filters

Bloom filter based spell checker.

Array of _m_ 0-bits

Generate _k_ hashes

- Hash value once using MD5
- Split into _k_ parts -> yield each as an individual hash value to set a bits in the array

Position of bits to set in array = _hash(x) % m_

Prime w/ words from `/usr/dict/words`

Take input words and check if in bloom filter

- If not -> incorrect spelling
