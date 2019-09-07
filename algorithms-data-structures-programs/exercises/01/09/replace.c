/**
 * Given a text T in the form of a file, and lists of a small number of words in
 * the form of two arrays A and B. Assume that words are short arrays of
 * characters of a small and fixed maximum length.
 * Write a program that transforms the text T into a text S by replacing each
 * occurrence of a word Ai by its corresponding word Bi .
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "hash_table.h"

#define MAXWORD 100

/**
 * Transform ifp to ofp by replacing all occurences of word A[i] with
 * corresponding word B[i]
 */
void replace(FILE *ifp, FILE *ofp, ht_t *replacements)
{
    char word[MAXWORD];
    char *w = word;
    char c;
    while ((c = fgetc(ifp)) != EOF)
    {
        if (!isalpha(c))
        {
            *w++ = '\0';
            char *rep = ht_get(replacements, word);
            fputs(rep == NULL ? word : rep, ofp);
            fputc(c, ofp);
            w = word;
        }
        else
        {
            *w++ = c;
        }
    }
}

ht_t *init_replacements(char *A[], char *B[], unsigned int len)
{
    /* initialize replacements table */
    ht_t *hash_table = ht_alloc();
    for (int i = 0; i < len; i++)
    {
        ht_set(hash_table, A[i], B[i]);
    }
    return hash_table;
}

int main(int argc, char *argv[])
{
    char *A[] = {"hobbit", "nasty", "in", "lots"};
    char *B[] = {"man", "ugly", "on", "loads"};
    ht_t *hash_table = init_replacements(A, B, 4);

    /* no args: stdin -> stdout */
    if (argc == 1)
    {
        replace(stdin, stdout, hash_table);
    }
    else
    {

        char *prog = argv[0];
        FILE *ifp;
        if ((ifp = fopen(*++argv, "r")) == NULL)
        {
            fprintf(stderr, "%s: can't open input file %s\n", prog, *argv);
            exit(1);
        }
        argc--;
        FILE *ofp;
        if (argc > 1)
        {
            if ((ofp = fopen(*++argv, "w")) == NULL)
            {
                fprintf(stderr, "%s: can't open output file %s\n", prog, *argv);
                exit(1);
            }
        }
        else
        {
            ofp = stdout;
        }

        replace(ifp, ofp, hash_table);
    }
}
