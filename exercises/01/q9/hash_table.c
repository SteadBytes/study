#include "hash_table.h"

#define TABLE_SIZE 1000

/**
 * Form hash value for string s where 0 <= hash(s) < TABLE_SIZE
 */
unsigned int hash(const char *s)
{
    /* add each char value in s to scrambled combination of previous values */
    unsigned int hash = 0;
    for (; *s != '\0'; s++)
    {
        hash = *s + 31 * hash;
    }
    return hash % TABLE_SIZE;
}

/**
 * Allocate a hash table entry for a key, val pair
 */
entry_t *ht_alloc_pair(const char *key, const char *val)
{
    /* allocate entry */
    entry_t *entry = malloc(sizeof(entry_t));
    entry->key = malloc(strlen(key) + 1);
    entry->val = malloc(strlen(val) + 1);

    /* assign key, val to new entry */
    strcpy(entry->key, key);
    strcpy(entry->val, val);

    entry->next = NULL;

    return entry;
}

/**
 * Allocate a hash table
 */
ht_t *ht_alloc(void)
{
    /* allocate hash table */
    ht_t *hash_table = malloc(sizeof(ht_t));

    /* allocate entries */
    hash_table->entries = malloc(sizeof(entry_t) * TABLE_SIZE);

    /* initialize all entries to NULL */
    for (int i = 0; i < TABLE_SIZE; i++)
    {
        hash_table->entries[i] = NULL;
    }
    return hash_table;
}

/**
 * Set key, val pair in hash table, overwriting existing value if key exists.
 */
void ht_set(ht_t *hash_table, const char *key, const char *val)
{
    unsigned int pos = hash(key);
    entry_t *entry = hash_table->entries[pos];

    /* empty slot -> insert new entry directly */
    if (entry == NULL)
    {
        hash_table->entries[pos] = ht_alloc_pair(key, val);
        return;
    }

    entry_t *prev;

    /* existing entry in slot -> walk through until end or matching key found */
    while (entry != NULL)
    {
        if (strcmp(entry->key, key) == 0)
        {
            /* match found -> replace existing value */
            free(entry->val);
            entry->val = malloc(strlen(val) + 1);
            strcpy(entry->val, val);
            return;
        }

        prev = entry;
        entry = prev->next;
    }
    /* no match found -> insert new entry at end */
    prev->next = ht_alloc_pair(key, val);
}

/**
 * Get val from hash table corresponding to key
 */
char *ht_get(ht_t *hash_table, const char *key)
{
    unsigned int pos = hash(key);
    entry_t *entry = hash_table->entries[pos];

    /* empty slot -> no entry */
    if (entry == NULL)
    {
        return NULL;
    }

    /* walk through entries in slot until end or matching key found */
    while (entry != NULL)
    {
        if (strcmp(entry->key, key) == 0)
        {
            /* matching key found */
            return entry->val;
        }

        entry = entry->next;
    }
    /* key not found */
    return NULL;
}

/**
 * Delete entry from hash table corresponding to key
 */
void ht_del(ht_t *hash_table, const char *key)
{
    unsigned int pos = hash(key);
    entry_t *entry = hash_table->entries[pos];

    /* key not found */
    if (entry == NULL)
    {
        return;
    }

    entry_t *prev;
    int first = 1;
    /* walk through entries until matching key found */
    while (entry != NULL)
    {
        if (strcmp(entry->key, key) == 0)
        {
            /* first item with no next entry */
            if (entry->next == NULL && first)
            {
                hash_table->entries[pos] = NULL;
            }

            /* first item with next entry */
            if (entry->next != NULL && first)
            {
                hash_table->entries[pos] = entry->next;
            }

            /* last item */
            if (entry->next == NULL && !first)
            {
                prev->next = NULL;
            }

            /* middle item */
            if (entry->next != NULL && !first)
            {
                prev->next = entry->next;
            }

            /* free deleted entry */
            free(entry->key);
            free(entry->val);
            free(entry);

            return;
        }
        prev = entry;
        entry = prev->next;

        first = 0;
    }
}