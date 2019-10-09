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

void queue_check_memory_integrity(void);
void track_queue(Queue *q);

Queue *queue_new(size_t n)
{
    assert(n > 0);

    int *buf = mem_malloc((n + 1) * sizeof(int));
#ifdef DEBUG
    assert(buf);
    note_ref(buf);
#endif
    Queue *q = mem_malloc(sizeof(Queue));
    assert(q);
    q->inp = 0;
    q->outp = 0;
    q->size = n + 1;
    q->buf = buf;

#ifdef DEBUG
    track_queue(q);
    note_ref(q);
    queue_check_memory_integrity();
#endif
    return q;
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

/* track all created queues for memory allocation/deallocation */
typedef struct known_queue_node
{
    Queue *qp;
    struct known_queue_node *next;
} KnownQueueNode;

static KnownQueueNode *head = NULL;

void queue_check_memory_integrity(void)
{
#ifdef DEBUG
    /* mark all blocks as unreferenced */
    clear_refs();

    /* note all known allocations */
    for (KnownQueueNode *kp = head; kp != NULL; kp = kp->next)
    {
        note_ref(kp);
        note_ref(kp->qp->buf);
        note_ref(kp->qp);
    }

    /* ensure allocations are properly accounted for */
    check_refs();
#endif
}

void track_queue(Queue *q)
{
    assert(q);
    KnownQueueNode *k = mem_malloc(sizeof(KnownQueueNode));
    assert(k);
    k->qp = q;
    k->next = head;
    head = k; /* advance KnownQueueNode linked list head */
#ifdef DEBUG
    note_ref(k);
#endif
}

void untrack_queue(Queue *q)
{
    KnownQueueNode *kp, *prev;
    prev = NULL;
    for (kp = head; kp != NULL; kp = kp->next)
    {
        if (kp->qp == q)
        {
            if (prev == NULL)
            {
                head = kp->next;
            }
            else
            {
                prev->next = kp->next;
            }
            break;
        }
        prev = kp;
    }

#ifdef DEBUG
    /* q not valid or untracked */
    assert(kp);
#endif

    mem_free(kp);
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

    /* remove KnownQueueNode corresponding to q */
    untrack_queue(q);

    /* Free memory for Queue pointed to by q */
    mem_free(q->buf);
    mem_free(q);
#ifdef DEBUG
    queue_check_memory_integrity();
#endif
    /* TODO: Should this assert q replaced with garbage bytes? */
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