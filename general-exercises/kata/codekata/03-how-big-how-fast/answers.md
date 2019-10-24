# Kata03: How Big? How Fast?

## How Big?

1.  - 1,000 = 10 bits
    - 1,000,000 = 20 bits
    - 1,000,000 = 30 bits
    - 1,000,000,000 = 40 bits
    - 8,000,000,000 = 43 bits
2.  2100000000 bits = 262.5 Mb
    - 70 chars for full name
    - 100 chars for address
    - 15 chars for phone number
3.  Nodes = 1,000,000, levels ~= 20, space = 4 Mb
    - Binary tree height ~= _log<sub>2</sub>n_
    - Integer on 32 bit architecture = 4 bytes

## How Fast?

1. ~ 40s
   - Assume ~ 300 words per page, ~ 5 characters per word + a space
   - 1200 _ 300 _ (5 + 1) = 2160000 bits
   - 56 kbit/s = 56000 bit/s
   - 2160000 / 56000 ~= 40s
2. ~7 ms
   - Nodes visited in binary search ~= _log<sub>2</sub>n_
   - Nodes visited in 10,000 node tree ~= 14
   - Time per node in 10,000 tree = 4.5 / 14 ~= 0.3 ms
   - Nodes visited in 100,000 node tree ~= 17
   - Time per node in 1000,000 node tree ~= 6 / 17 ~= 0.3
   - ~ constant time per node
   - Nodes visited in 10,000,000 node tree != 24
   - Total time for 10,000,000 node tree = 24 \* 0.3 ~= 7ms
3. No, generating all possibilities would take ~ 64 years
   - Assuming all passwords use maximum length
   - Total character combinations = 16! * 96 = 2*10<sup>15</sup>
   - Total time = 2*10<sup>15</sup> * 10<sup>03</sup>s ~= 64 years


