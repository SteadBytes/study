#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>

#define BUFSIZE 4096

size_t read_file(int fd, char **data);

int main()
{
    size_t len;
    char *data = NULL;

    len = read_file(STDIN_FILENO, &data);

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