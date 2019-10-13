# C Data Structure Memory Management Example

Memory management routines to provide better safety semantics than `malloc` and `free` alone along with an example of their usage for managing a [queue](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>) data structure.

Improved safety is provided by tracking memory allocations and bookkeeping their information - pointer to the start of the memory block, size of the block, whether it is referenced e.t.c. This is implemented in `src/mem_block.c` and `src/mem.c` A module implementing a data structure should encapsulate the management of it's own memory using these improved allocation functions. `src/queue.c` contains an example of such an implementation.

Pre/post conditions are used to assert that the semantic invariants of memory usage are upheld:

- Pointers are checked for validity before usage
  - Not `NULL`
  - Point to a **known** memory block
  - The size of the memory block is sufficient for usage
- Memory allocations and deallocations are tracked
- Allocated memory blocks are not lost/leaked throughout the program

**Note**: To avoid runtime overhead, memory tracking is enabled only in **debug** builds via use of `#ifdef DEBUG`.

Initially based on ideas presented in Chapter 3 of ["Writing Solid Code" by Steve Maguire](http://writingsolidcode.com/).

Implemented as part of [The Pragmatic Programmer](https://pragprog.com/book/tpp20/the-pragmatic-programmer-20th-anniversary-edition) Topic 26, Challenge 1:

> Although there are no guaranteed ways of ensuring that you always free resources, certain design techniques, when applied consistently, will help. In the text we discussed how establishing a semantic invariant for major data structures could direct memory deallocation decisions. Consider how Topic 23, Design by Contract, on page 104, could help refine this idea.

See my accompanying [blog post](https://steadbytes.com/blog/the-pragmatic-programmer-20th/topic-26-exercises/)

This approach is certainly not perfect, this is an improvement over uncontrolled usage of `malloc` and `free` and an interesting thought experiment into resource management.

## Building

Production build:

```
make
```

Debug build:

```
make DEBUG=1
```

## Code Structure

- `src/mem_block.c`: Memory tracking/logging
- `src/mem.c`: Memory allocation/deallocation functions (wrappers around `malloc` and `free`) which utilise `mem_block.c` to track allocations/deallocations
- `src/queue.c`: basic implementation of a [Queue](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>) utilising `mem.c` to encapsulate memory management for the `Queue` data structure
- `src/main.c`: simple entry point demonstrating the use of `queue.c`
