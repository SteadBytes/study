#ifndef MEM_BLOCK_H
#define MEM_BLOCK_H

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#define garbage_bytes 0xCC

typedef unsigned char byte;
typedef signed char flag;

typedef struct MemBlockInfo
{
    struct MemBlockInfo *next;
    byte *p;     /* start of block */
    size_t size; /* length of block */
    flag is_referenced;
} MemBlockInfo;

flag create_mem_block_info(byte *p, size_t size);
void free_mem_block_info(byte *p);
size_t block_size(byte *p);

void clear_refs(void);
void note_ref(void *p);
void check_refs(void);
flag valid_pointer(void *p, size_t size);

#endif
