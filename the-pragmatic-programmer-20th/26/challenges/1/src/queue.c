#include "queue.h"

flag queue_valid(Queue *q);
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

size_t queue_size(Queue *q)
{
    size_t size = (q->inp - q->outp + q->size) % q->size;
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
}