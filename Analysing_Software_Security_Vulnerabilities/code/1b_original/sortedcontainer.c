#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sortedcontainer.h"

int data_compare(data* d1, data* d2) {
    assert(d1);
    assert(d2);
    if(d1->age < d2->age) return -1;
    if(d1->age > d2->age) return 1;
    return strcmp(d1->name, d2->name);
}

// Do not change
void data_print(data* d, FILE* f) {
    fprintf(f, "%i %s", d->age, d->name);
}

data* data_new(int age, char const* name) {
    data* d = (data*)malloc(sizeof(data));
    d->age = age;
    strncpy(d->name, name, NAME_LENGTH);
    return d;
}

void data_delete(data* d) {
    free(d);
}

node* node_new(data* d) {
    // Implement this
    (void)d;
    return NULL;
}

void node_delete(node* n) {
    // Implement this
    (void)n;
}

sortedcontainer* sortedcontainer_new() {
    sortedcontainer* d = (sortedcontainer*)malloc(sizeof(sortedcontainer));
    d->root = NULL;
    return d;
}

void sortedcontainer_insert(sortedcontainer* sc, data* data) {
    node* n = node_new(data);
    // Implement this
    (void)sc;
    (void)n;
}

int sortedcontainer_erase(sortedcontainer* sc, data* data) {
    // Implement this
    (void)sc;
    (void)data;
    return 0;
}

int sortedcontainer_contains(sortedcontainer* sc, data* data) {
    // Implement this
    (void)sc;
    (void)data;
    return 0;
}

// Do not change
static void node_printtree(node* n, int level, FILE* printFile) {
    fprintf(printFile, "%*s ", level, "");
    if(n) {
        data_print(n->data, printFile);
        fprintf(printFile, "\n");
        node_printtree(n->left, level+1, printFile);
        node_printtree(n->right, level+1, printFile);
    } else {
        fprintf(printFile, "(nil)\n");
    }
}

// Do not change
void sortedcontainer_print(sortedcontainer* sc, FILE* printFile) {
    node_printtree(sc->root, 0, printFile);
}

static void node_deletetree(node* n) {
    if(n) {
        node* left = n->left;
        node* right = n->right;
        node_delete(n);
        node_deletetree(left);
        node_deletetree(right);
    }
}

void sortedcontainer_delete(sortedcontainer* sc) {
    node_deletetree(sc->root);
    free(sc);
}
