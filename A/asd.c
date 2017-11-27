#include <string.h>



int main(int argc, char* argv[]){

    /******* Snippet 1 (page 24) **********/
    char *path = argv[1]; // Mine 

    char buf[20];
    char prefix[] = "http://";

    // copies the string prefix to buf
    strcpy(buf, prefix); 

    // concatenates path to the string buf
    strncat(buf, path, sizeof(buf)); 

    return 0;
}

