#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>

#define BUFSIZE 4096

size_t read_file(int fd, char **data);
size_t read_file_improved(int fd, char **data);

int main(int argc, char *argv[])
{
    size_t len;
    char *data = NULL;
    /* default to improved implementation*/
    if (argc == 1)
    {
        len = read_file_improved(STDIN_FILENO, &data);
    }
    /* optionally specify original implementation for comparison purposes */
    else if (argc == 2 && !strcmp(argv[1], "--slow"))
    {
        len = read_file(STDIN_FILENO, &data);
    }
    else
    {
        fprintf(stderr, "usage: read_file [--slow]\nReads from standard input.");
        return 1;
    }
    // do some work with data
    printf("%ld\n", len);

    return 0;
}

size_t read_file(int fd, char **data)
{
    char buf[BUFSIZE];
    size_t len = 0;
    ssize_t read_len;
    char *tmp;

    while ((read_len = read(fd, buf, BUFSIZE)) > 0)
    {
        tmp = malloc(len + read_len);
        memcpy(tmp, *data, len);
        memcpy(tmp + len, buf, read_len);
        free(*data);
        *data = tmp;
        len += read_len;
    }
    return len;
}

typedef struct node_t
{
    char *data;
    int len;
    struct node_t *next;
} node_t;

size_t read_file_improved(int fd, char **data)
{
    char buf[BUFSIZE];
    ssize_t read_len;
    size_t len = 0;

    /* build linked list containing chunks of data read from fd */
    node_t *head = NULL;
    node_t *prev = NULL;
    while ((read_len = read(fd, buf, BUFSIZE)) > 0)
    {
        node_t *node = malloc(sizeof(node_t));
        node->data = malloc(read_len);
        memcpy(node->data, buf, read_len);
        node->len = read_len;
        node->next = NULL;

        /* first chunk */
        if (head == NULL)
        {
            head = node;
        }
        /* second chunk onwards */
        if (prev != NULL)
        {
            prev->next = node;
        }
        prev = node;
        len += read_len;
    }

    /* copy each chunk into data once only */
    *data = malloc(len);
    char *p = *data;
    node_t *cur = head;
    while (cur != NULL)
    {
        memcpy(p, cur->data, cur->len);
        p += cur->len;
        cur = cur->next;
    }

    return len;
}