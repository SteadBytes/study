#include <stdio.h>

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

Queue *queue_new(size_t n)
{
    assert(n > 0);

    int *buf = mem_malloc((n + 1) * sizeof(int));
#ifdef DEBUG
    assert(buf);
    note_ref(buf);
#endif
    Queue q = {0, 0, (n + 1), buf};
    Queue *qptr = mem_malloc(sizeof(Queue));

    assert(qptr);
    *qptr = q;

#ifdef DEBUG
    note_ref(qptr);
#endif
    return qptr;
}

flag queue_valid(Queue *p)
{
#ifdef DEBUG
    return valid_pointer(p->buf, p->size) && valid_pointer(p, sizeof(Queue));
#else
    return (flag)1;
#endif
}

void queue_free(Queue *q)
{
    assert(queue_valid(q));

    /* mem_free implements post conditions to ensure correct deallocation */
    mem_free(q->buf);
    mem_free(q);
    /* TODO: Should this assert q replaced with garbage bytes? */
    /* FIXME: Fails in debug build as mem block references are not cleared */
}

int queue_size(Queue *q)
{
    assert(q);
    int size = (q->inp - q->outp + q->size) % q->size;
    assert(size >= 0 && size <= q->size);
    return size;
}

void queue_put(Queue *q, int n)
{
    int size = queue_size(q);
    assert(size < q->size);

    q->buf[q->inp] = n;
    q->inp = (q->inp + 1) % q->size;

    assert(size + 1 == queue_size(q));
}

int queue_get(Queue *q)
{
    assert(q);
    int ans = q->buf[q->outp];
    q->outp = (q->outp + 1) % q->size;
    return ans;
}

int main(void)
{
    Queue *q = queue_new(10);

    /* fill the queue */
    for (int i = 1; i < 11; i++)
    {
        printf("put int %d onto queue\n", i);
        queue_put(q, i);
    }

    printf("size, %d\n", queue_size(q));
    printf("removed from queue: %d\n", queue_get(q));
    printf("size, %d\n", queue_size(q));
    printf("put int 5 onto queue\n");
    queue_put(q, 5);
    printf("size, %d\n", queue_size(q));

    /* empty the queue */
    while (queue_size(q) > 0)
    {
        printf("removed from queue: %d\n", queue_get(q));
        printf("size, %d\n", queue_size(q));
    }
    queue_free(q);
}