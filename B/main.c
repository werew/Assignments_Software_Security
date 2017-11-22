#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sortedcontainer.h"
#include "test.h"

// DO NOT change this value. It does not fix your problems
#define INPUT_INCREMENT 10

/**
 * @brief Prints the prompt to @c f
 * @param f The FILE to print the prompt to.
 */
void print_prompt(FILE* f) {
    fprintf(f, "\n> "); fflush(f);
}

/**
 * @brief Basic parser to read data from @c command
 * @param command The command string to read the data from
 * @return A new data object
 *
 * TO FIX:
 *   There are two serious problems in this function that are related
 */
data* read_data(char const* command) {
    int age;
    char name[NAME_LENGTH];
    sscanf(command, "%*s %i %s", &age, name);
    return data_new(age, name);
}

/**
 * @brief Handles @c command
 * @param printFile FILE to print messages to
 * @param sc The sortedcontainer to query or modify
 * @param command The command to handle
 * @return 1 iff the problem should quit, otherwise 0
 *
 * TO FIX:
 *   There are three problems in this function, two of which are related
 */
int handle_command(FILE* printFile, sortedcontainer* sc, char* command) {
    switch(*command) {
    case 'i':
        sortedcontainer_insert(sc, read_data(command));
        break;
    case 'e':
        sortedcontainer_erase(sc, read_data(command));
        break;
    case 'c':
        if(sortedcontainer_contains(sc, read_data(command))) {
            fprintf(printFile, "y\n");
        } else {
            fprintf(printFile, "n\n");
        }
        break;
    case 'p':
        sortedcontainer_print(sc, printFile);
        break;
    case 'x':
        return 1;
        break;
    case 't':
        test(printFile);
        break;
    default: {
        fprintf(printFile, "No such command: ");
        fprintf(printFile, "%s\n", command); // TODO this is a fast fix, but what if command is multiple words?
        fprintf(printFile, "\n");
        break;
    }
    }
    return 0;
}

/**
 * @brief Reads a command from the FILE @c in
 * @param in FILE to read a command from
 * @return The read command
 *
 * TO FIX:
 *   There are two separate problems in this function. Fix these problems
 *   by only changing TWO lines in total.
 */
char* read_command(FILE* in) {
    int inputMaxLength = 0;
    char* input = NULL;
    char* inputAt = NULL;

    int incr = INPUT_INCREMENT;

    inputMaxLength = incr;
    input = (char*)malloc(sizeof(char) * incr);
    inputAt = input;
    do {
        inputAt[incr - 1] = 'e';
        if(fgets(inputAt, incr, in) == NULL) return NULL;
        if(inputAt[incr - 1] != '\0' || inputAt[incr - 2] == '\n') {
            break;
        }
        inputMaxLength += INPUT_INCREMENT;
        input = realloc(input, sizeof(char) * inputMaxLength);
        inputAt += incr - 1;
        incr = INPUT_INCREMENT + 1;
    } while(1);
    input[strlen(input)-1] = 0;
    return input;
}

/**
 * @brief The main SINT loop
 * @param argc Argument count
 * @param argv Arguments
 * @return 0
 *
 * TO FIX:
 *   One issue needs to be fixed here.
 */
int main(int argc, char* argv[]) {
    (void)argc;
    (void)argv;

    sortedcontainer* sc = sortedcontainer_new();

    while(1) {
        print_prompt(stdout);

        char* command = read_command(stdin);
        if(command == NULL) {
            break;
        }

        if(handle_command(stdout, sc, command)) {
            break;
        }
    }

    sortedcontainer_delete(sc);

    fprintf(stdout, "\nBye.\n");

    return 0;
}

