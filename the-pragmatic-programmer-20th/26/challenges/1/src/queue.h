#ifndef QUEUE_H
#define QUEUE_H

#include <stdlib.h>
#include <assert.h>

#include "mem.h"
#include "mem_block.h"

typedef struct queue
{
    int inp;
    int outp;
    size_t size;
    int *buf;
} Queue;

Queue *queue_new(size_t n);
size_t queue_size(Queue *q);
void queue_put(Queue *q, int n);
int queue_get(Queue *q);
void queue_free(Queue *q);

#endif