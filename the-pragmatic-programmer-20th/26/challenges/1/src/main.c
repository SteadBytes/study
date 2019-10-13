#include <stdio.h>
#include "queue.h"

int main(void)
{
    Queue *q = queue_new(10);

    /* basic queue operations */
    printf("size: %ld\n", queue_size(q));
    printf("put int 10 onto queue\n");
    queue_put(q, 10);
    printf("size: %ld\n", queue_size(q));
    printf("removed from queue: %d\n", queue_get(q));
    printf("size: %ld\n", queue_size(q));
    printf("put int 5 onto queue\n");
    queue_put(q, 5);
    printf("size: %ld\n", queue_size(q));

    queue_free(q);

    /*
    * In DEBUG build performing queue operations after queue_free causes 
    * memory checks to fail. Each of the following statements will cause the
    * program to exit with assertion failures in DEBUG build.
    */
    queue_size(q);
    queue_put(q, 10);
    queue_get(q);
}