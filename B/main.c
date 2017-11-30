#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <ctype.h>
#include <limits.h>

#include "sortedcontainer.h"
#include "test.h"

// DO NOT change this value. It does not fix your problems
#define INPUT_INCREMENT 10

/**
 * @brief Prints the prompt to @c f
 * @param f The FILE to print the prompt to.
 */
void print_prompt(FILE* f) {
    fprintf(f, "\n> "); fflush(f); //TODO uncaught IO errors (maybe explain why we don't consired it harmful)
}

/**
 * @brief Basic parser to read data from @c command
 * @param command The command string to read the data from
 * @return A new data object or a NULL pointer in case of error
 *         in this case errno is set accordingly
 *
 * BUGS FIXED:
 *
 * - Stack-based buffer overflow: 
 *      if the length of the 3rd argument of the command was bigger than
 *      NAME_LENGTH then `scanf` would have written beyond the bounds
 *      of the array `name`. 
 *
 * - Potential uninitialized values:
 *      in case of failure of `scanf`, `name` and/or `age` are used by the
 *      program without a proper initialization. This can be easily accomplished
 *      by an attacker by causing a "matching failure" (for example with the
 *      input: "i not_a_number foo") and could be used, for example, to leak 
 *      potentially sensitive data from the stack.
 *
 * - Potential integer overflow (or underflow):
 *     a big (or too small negative) age could have lead to an integer overflow
 *     (or underflow respectively. This could have resulted in unintended behaviour.
 *
 * OBSERVATIONS:
 *
 * - Negative ages are allowed:
 *      ages are by their nature always positives, on the other hand the program
 *      accepts also negative ages. Given that in the type `data` the field age
 *      is defined as a signed integer (and we were told not to modify type 
 *      definitions) we suppose that this is indeed an intended behaviour.
 *
 * - Return value could be NULL:
 *      `data_new` could return a NULL pointer (in case of failure of malloc),
 *      therefore `read_data` could also return NULL but this behaviour was not
 *      documented in the specifications
 *
 */
data* read_data(char const* command) {
    /* Some declarations */
    long int age;           // a long int is necessary for strtol
    char name[NAME_LENGTH]; 
    char const *p = command;
    char *endptr;


    /* 1. Skip command name */
    for (; isspace((int) *p); p++); // 1) Skip initial spaces (if any)
    for (; isalpha((int) *p); p++); // 2) Skip command name




    /* 2. Read age: for safety reasons we favor strtol over atoi or scanf,
          since this function let us check the validity of the output  */

    errno = 0;                      // So we can distinguish success/failure after call
    age = strtol(p, &endptr, 0);

    // Check for invalid age
    if ((errno == ERANGE && (age == LONG_MAX || age == LONG_MIN)) ||
        (errno != 0 && age == 0)) return NULL;

    // Check if age was found
    if (endptr == p){
        errno = EINVAL;
        return NULL;
    }

    // Check for int overflow/underflow 
    // (we make sure that age is a valid int before the cast)
    if (age > INT_MAX || age < INT_MIN){
        errno = ERANGE;
        return NULL;
    }

    p = endptr;   // Move forward

    /* 3. Read name */
    for (; isspace((int) *p); p++);     // Skip white-spaces (if any)

    unsigned int i = 0;
    while (*p != '\0' && !isspace((int) *p)){ // We only allow non white-space 
                                              // chars inside a name: this is the same 
                                              // as the original %s, just a safer
        // Fail if the name is too long
        if (i >= NAME_LENGTH - 1) { 
            errno = EINVAL;         
            return NULL; 
        }

        name[i++] = *p++;            // Copy character 
    }

    name[i] = '\0';                  // Null-terminate string
    
    // Fail if name was not found
    if (i == 0){
        errno = EINVAL;
        return NULL;
    }

    // Fail if an invalid character was encountered
    if (*p != '\0' && isspace((int)*p) == 0){
        errno = EINVAL;
        return NULL;
    }

    for (; isspace((int) *p); p++);  // Skip trailing spaces (if any)
   
    // Fail if the command contains unnecessary extra arguments (TODO ask) 
    if (*p != '\0'){
        errno = EINVAL;
        return NULL;
    }

    
    /* 4. Create and return new data struct */
    return data_new((int) age, name);
}

/**
 * @brief Handles @c command
 * @param printFile FILE to print messages to
 * @param sc The sortedcontainer to query or modify
 * @param command The command to handle
 * @return 1 iff the problem should quit, otherwise 0
 *
 * BUGS FIXED:
 *
 *  - Format string bug:
 *      command was passed directly to fprintf and interpreted as a format
 *      string. Using this vulnerability an attacker could have leaked
 *      or manipulated the memory.
 *
 *  - Return value of read_data not checked for errors:
 *      in case of error (eg. Invalid command parameters) the value 
 *      returned by read_data is used without any further check.
 *
 *  - Return value of read_data not freed:
 *      when erasing or checking the presence of certain data, the 
 *      structure returned by read_data was not properly freed afterwards
 */
int handle_command(FILE* printFile, sortedcontainer* sc, const char* command) {
    data* d = NULL;
 
    switch(*command) {
    case 'i':
        if ((d = read_data(command)) == NULL) goto error_handler;
        sortedcontainer_insert(sc, d);  // sortedcontainer_insert does claim ownership of d
        break;
    case 'e':
        if ((d = read_data(command)) == NULL) goto error_handler;
        sortedcontainer_erase(sc, d);   // sortedcontainer_erase does NOT claim ownership of d
        data_delete(d);   
        break;
    case 'c':
        if ((d = read_data(command)) == NULL) goto error_handler;
        if(sortedcontainer_contains(sc, d)) {  // sortedcontainer_contains does NOT claim ownership of d
            fprintf(printFile, "y\n");
        } else {
            fprintf(printFile, "n\n");
        }
        data_delete(d);   
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
        fprintf(printFile, "%s\n", command); 
        fprintf(printFile, "\n");
        break;
    }
    }
    return 0;

error_handler: 
    /* XXX IMPORTANT: Please don't be mad for this `goto`.
       While most C purists think `goto` statements should never be 
       used, we think that a responsible use of `goto` can help
       code's readability (we don't like spaghetti code too). 
       One of C programmers most common practices is to use `goto` 
       to directly jump to an error handler when needed (such practice
       can be found in many famous projects as for example: 
       linux, apache, valgind, etc..)                           */
       
    if (errno == ERANGE || errno == EINVAL){
        fprintf(printFile,"Invalid input\n");
    } else {
        fprintf(printFile, "%s", strerror(errno)); 
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
 *   by only changing TWO lines in total. TODO there are more than 2, ask
 *
 * BUGS FIXED:
 *  - Invalid reads/writes on the heap:
 *      the pointer inputAt was not appropriately updated after reallocating
 *      the buffer.
 *
 *  - Invalid write on the stack (off-by-one bug):
 *      an input starting with a zero byte will cause
 *      a write one byte out of the input buffer 
 */
char* read_command(FILE* in) {
    int inputMaxLength = 0;
    char* input = NULL;
    char* inputAt = NULL;

    int incr = INPUT_INCREMENT;

    inputMaxLength = incr;
    input = (char*)malloc(sizeof(char) * incr);         // TODO if input is NULL
    inputAt = input;
    do {
        inputAt[incr - 1] = 'e';
        if(fgets(inputAt, incr, in) == NULL) return NULL;   // TODO memory leak if NULL (input lost)
        if(inputAt[incr - 1] != '\0' || inputAt[incr - 2] == '\n') {
            break;
        }
        inputMaxLength += INPUT_INCREMENT;
        input = realloc(input, sizeof(char) * inputMaxLength);  // TODO if return NULL, memory leak
        inputAt = input + inputMaxLength - INPUT_INCREMENT - 1;
        incr = INPUT_INCREMENT + 1;
    } while(1);
    input[strlen(input) == 0 ? 0 : strlen(input)-1] = 0; // TODO error if empty string
    return input;
}

/**
 * @brief The main SINT loop
 * @param argc Argument count
 * @param argv Arguments
 * @return 0
 *
 * BUGS FIXED:
 *  - Memory leak:
 *      the memory allocated by read_command was never freed    
 */
int main(int argc, char* argv[]) {
    (void)argc;
    (void)argv;

    sortedcontainer* sc = sortedcontainer_new(); // TODO check if NULL

    while(1) {
        print_prompt(stdout);

        char* command = read_command(stdin);
        if(command == NULL) {
            break;
        }

        if(handle_command(stdout, sc, command)) {
            free(command);
            break;
        }

        free(command);
    }

    sortedcontainer_delete(sc);

    fprintf(stdout, "\nBye.\n");

    return 0;
}

