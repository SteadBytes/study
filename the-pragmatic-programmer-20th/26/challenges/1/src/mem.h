#ifndef MEM_H
#define MEM_H

#include <memory.h>
#include <assert.h>
#include <stdlib.h>
#include "mem_block.h"

void *mem_malloc(size_t size);
void mem_free(void *block);

#endif