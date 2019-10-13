#include "mem_block.h"
#include "mem.h"

void *mem_malloc(size_t size)
{
    void *p_block = malloc(size);
#ifdef DEBUG
    assert(p_block);

    if (!create_mem_block_info(p_block, size))
    {
        free(p_block);
        p_block = NULL;
    }
#endif
    return p_block;
}

void mem_free(void *p_block)
{
#ifdef DEBUG
    assert(valid_pointer(p_block, block_size(p_block)));
    memset(p_block, garbage_bytes, block_size(p_block));
#endif
#ifdef DEBUG
    free_mem_block_info(p_block);
#endif
    free(p_block);
}