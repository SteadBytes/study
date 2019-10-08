#include "mem_block.h"
#include "mem.h"

void *mem_malloc(size_t size)
{
    void *p = malloc(size);
#ifdef DEBUG
    assert(p);

    if (!create_mem_block_info(p, size))
    {
        free(p);
        p = NULL;
    }
#endif
    return p;
}

void mem_free(void *block)
{
#ifdef DEBUG
    assert(valid_pointer(block, block_size(block)));
    memset(block, garbage_bytes, block_size(block));
#endif
#ifdef DEBUG
    free_mem_block_info(block);
#endif
    free(block);
}