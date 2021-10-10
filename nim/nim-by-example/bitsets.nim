# Sets of *ordinal types* that have `high(T) < 2^16`
# - Each element of the set consumes 1 bit
#  - Best practice to keep much smaller than the `2^16` limit (64KiB)
# - See `sets` module for hashsets to use with non-ordinal types

# Membership test
assert('d' in {'a'..'z'})
# Non-membership test
assert(40 notin {2..20})
# Union
assert({'a'..'m'} + {'n'..'z'} == {'a'..'z'})
# Relative complement
assert({'a'..'z'} - {'b'..'d'} == {'a', 'e'..'z'})
# Add element
assert({'b'..'z'} + {'a'} == {'a'..'z'})
# Remove element
assert({'a'..'z'} - {'a'} == {'b'..'z'})
# Intersection
assert({'a'..'m'} * {'c'..'z'} == {'c'..'m'})
# Subset
assert({'a'..'c'} <= {'a'..'z'})
# Strict subset
assert({'b'..'c'} < {'a'..'z'})
