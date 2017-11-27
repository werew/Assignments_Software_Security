#include <string.h>


/******* Snippet 4 (page 40) **********/
#define MAX_BUF 256

void BadCode (char* input){   
    short len;
    char buf[MAX_BUF];
    len = strlen(input);
    if (len < MAX_BUF) strcpy(buf,input);
}

int main(int argc, char* argv[]){

    /******* Snippet 1 (page 24) **********/
    char buf[20];
    char prefix[] = "http://";

    // copies the string prefix to buf
    strcpy(buf, prefix); 

    // concatenates path to the string buf
    strncat(buf, path, sizeof(buf)); 



    /******* Snippet 2 (page 26) **********/
    char src[9];
    char dest[9];
    char* base_url = ”www.ru.nl”;

    // copies base_url to src
    strncpy(src, base_url, 9); 

    // copies src to dest
    strcpy(dest, src);

 
    /******* Snippet 3 (page 30) **********/
    char *buf;
    int i, len;

    // read sizeof(int) bytes, ie. an int,
    // and store these at &len, ie. the 
    // memory address of the variable len
    read(fd, &len, sizeof(int)); 

    buf = malloc(len);

    read(fd,buf,len); // read len bytes into buf


    /******* Snippet 4 (page 40) **********/
    #define MAX_BUF 256

    char* input;
    short len;
    char buf[MAX_BUF];
    len = strlen(input);
    if (len < MAX_BUF) strcpy(buf,input);


    return 0;
}

