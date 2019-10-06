#include "mem_block.h"
#include "mem.h"

void *mem_malloc(size_t size)
{
    void *p = malloc(size);
    assert(p);

    if (!create_mem_block_info(p, size))
    {
        free(p);
        p = NULL;
    }
    return p;
}

void mem_free(void *block)
{
    assert(valid_pointer(block, block_size(block)));
    memset(block, garbage_bytes, block_size(block));
    free_mem_block_info(block);
    free(block);
}