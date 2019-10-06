#include "mem_block.h"

/* Private */

/* Linked list to track memory block information */
static MemBlockInfo *head = NULL;

/* 
* Find memory block information corresponding to the assigned memory
* block which contains location pointed to by p. p does not have to point
* to the start of the memory block.
* Exits with assert if no found.
*/
static MemBlockInfo *get_mem_block_info(byte *p)
{
    MemBlockInfo *cur;

    /* scan linked list of known MemBlockInfo */
    for (cur = head; cur != NULL; cur = cur->next)
    {
        byte *start = cur->p;
        byte *end = cur->p + cur->size - 1;

        /* p can be anywhere within the memory block */
        if (p >= start && p <= end)
        {
            break;
        }
    }
    assert(cur != NULL);
    return cur;
}

/* Public API */

/* 
* Create a new log entry for the memory block at p_new_block
* Returns indicates allocation:
*
*   if (create_mem_block_info(p_new_block, size)) {
*       // success -> memory has log entry -> proceed
*   } else {
*
*       // failure -> memory doesn't have log entry -> release p_new_block
*   }
*/
flag create_mem_block_info(byte *p_new_block, size_t size)
{
    assert(p_new_block && size > 0);

    MemBlockInfo *p = (MemBlockInfo *)malloc(sizeof(MemBlockInfo));

    /* unable to allocate memory */
    if (p == NULL)
    {
        return (flag)0;
    }

    /* initialise MemBlockInfo*/
    p->p = p_new_block;
    p->size = size;
    p->next = head;

    head = p; /* advance MemBlockInfo linked list head */
    return (flag)1;
}

/*
* Free MemoryBlockInfo corresponding to memory block pointed to by p.
* Exits with assert if p does not point to a block of memory tracked by
* this module or if p is still referenced.
* Memory is set to garbage bytes before being freed.
*/
void free_mem_block_info(byte *p)
{
    assert(p);

    MemBlockInfo *cur, *prev;

    prev = NULL;

    /* scan linked list known MemBlockInfo */
    for (cur = head; cur != NULL; cur = cur->next)
    {
        if (cur->p == p)
        {
            if (prev == NULL)
            {
                head = cur->next;
            }
            else
            {
                prev->next = cur->next;
            }
            break;
        }
        prev = cur;
    }

    /* p not valid */
    assert(cur != NULL);

    assert(!cur->is_referenced);
    /* destroy contents before free */
    memset(cur, garbage_bytes, sizeof(MemBlockInfo));
    free(cur);
}


/*
* Return the size of memory block pointed to by p.
*/
size_t block_size(byte *p)
{
    assert(p);
    MemBlockInfo *p_block_info = get_mem_block_info(p);
    assert(p == p_block_info->p);
    return p_block_info->size;
}

/* 
* Mark all blocks in memory log as unreferenced
*/
void clear_refs(void)
{
    for (MemBlockInfo *cur = head; cur != NULL; cur = cur->next)
    {
        cur->is_referenced = (flag)0;
    }
}

/*
* Mark a memory block as being referenced. Used to track usage of pointers to
* avoid use-after-free (see free_mem_block_info).
*/
void note_ref(void *p)
{
    assert(p);
    MemBlockInfo *p_block_info = get_mem_block_info(p);
    p_block_info->is_referenced = (flag)1;
}

/*
* Check that all entries in the memory log have been marked with a call to
* note_ref. Asserts if unmarked block is found.
*/
void check_refs(void)
{
    for (MemBlockInfo *cur = head; cur != NULL; cur = cur->next)
    {
        /* basic block integrity */
        assert(cur->p && cur->size > 0);
        /* lost/leaky memory */
        assert(cur->is_referenced);
    }
}

/*
* Verify that p points to an allocated memory block with at least size allocated
* blocks from p to the end of the block.
* If either condition is not met assert will fail (never returns FALSE)
* Otherwise returns TRUE to allow usage within assert macro.
*/
flag valid_pointer(void *p, size_t size)
{
    assert(p && size > 0);
    byte *pb = (byte *)p;
    MemBlockInfo *p_block_info = get_mem_block_info(pb);

    assert(pb + size <= p_block_info->p + p_block_info->size);

    return (flag)1;
}