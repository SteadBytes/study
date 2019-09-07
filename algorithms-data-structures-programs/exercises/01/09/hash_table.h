#include <stdlib.h>
#include <string.h>

typedef struct entry_t
{
    char *key;
    char *val;
    struct entry_t *next;
} entry_t;

typedef struct
{
    entry_t **entries;
} ht_t;

ht_t *ht_alloc(void);
void ht_set(ht_t *hash_table, const char *key, const char *val);
char *ht_get(ht_t *hash_table, const char *key);
void ht_del(ht_t *hash_table, const char *key);