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

#define MAXWORD 100

#define N_REPLACEMENTS 4
char *A[] = {"hobbit", "nasty", "in", "lots"};
char *B[] = {"man", "ugly", "on", "loads"};

/**
 * Transform ifp to ofp by replacing all occurences of word A[i] with
 * corresponding word B[i]
 */
void replace(FILE *ifp, FILE *ofp)
{
    char word[MAXWORD];
    char *w = word;
    char c;
    while ((c = fgetc(ifp)) != EOF)
    {
        if (!isalpha(c))
        {
            *w++ = '\0';
            // TODO: Do this with hash table from A->B
            int matched = 0;
            for (int i = 0; i < N_REPLACEMENTS; i++)
            {
                if (strcmp(word, A[i]) == 0)
                {
                    fputs(B[i], ofp);
                    matched = 1;
                    break;
                }
            }
            if (!matched)
            {
                fputs(word, ofp);
            }
            fputc(c, ofp);
            w = word;
        }
        else
        {
            *w++ = c;
        }
    }
}

int main(int argc, char *argv[])
{
    /* no args: stdin -> stdout */
    if (argc == 1)
    {
        replace(stdin, stdout);
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

        replace(ifp, ofp);
    }
}
