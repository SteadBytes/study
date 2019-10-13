#ifndef MEM_BLOCK_H
#define MEM_BLOCK_H

#define garbage_bytes 0xCC
typedef unsigned char byte;
typedef signed char flag;

#ifdef DEBUG
#include <string.h>
#include <stdlib.h>
#include <assert.h>

typedef struct MemBlockInfo
{
    struct MemBlockInfo *next;
    byte *p_block;            /* start of block */
    size_t size;        /* length of block */
    flag is_referenced; /* TODO: Make a *count* of refs instead of binary flag */
} MemBlockInfo;

flag create_mem_block_info(byte *p_block, size_t size);
void free_mem_block_info(byte *p_block);
size_t block_size(byte *p_block);

void clear_refs(void);
void note_ref(void *p);
void check_refs(void);
flag valid_pointer(void *p, size_t size);

#endif
#endif