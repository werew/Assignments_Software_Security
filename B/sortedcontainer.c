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

    node* n = (node*) malloc(sizeof(node));
    if (n == NULL) return NULL;

    n->data  = d;
    n->left  = NULL;
    n->right = NULL;

    return n;
}

void node_delete(node* n) {
    data_delete(n->data);
    free(n);
}

sortedcontainer* sortedcontainer_new() {
    sortedcontainer* d = (sortedcontainer*)malloc(sizeof(sortedcontainer)); //TODO check return
    d->root = NULL; //TODO potential NULL pointer dereferencing
    return d;
}

void sortedcontainer_insert(sortedcontainer* sc, data* data) {

    /* Create new node */
    node* n = node_new(data);
    if (n == NULL){         // Fail silently 
        data_delete(data);
        return; 
    }

    /* If the tree is empty, insert as root */
    if (sc->root == NULL){
        sc->root = n;
        return;
    } 

    /* Traverse the tree and insert as a leaf */
    node* p = sc->root;     // Parent node
    while (1){
        switch (data_compare(data, p->data)){
            case -1: if (p->left == NULL){
                         p->left = n;
                         return;
                     }
                     p = p->left;
                break;
            case  1: if (p->right == NULL){
                         p->right = n;
                         return;
                     }
                     p = p->right;
                break;
            default: // Node is already inside the tree
                     node_delete(n);
                     return;     
        } 
    }
}

static void _node_removal(node* p, node* n){

    if (n->left  == NULL){
        /* Node has right child or is a leaf */
        if (p->right == n) p->right = n->right;
        else               p->left  = n->right;
        node_delete(n);

    } else if (n->right == NULL){
        /* Node has left child */
        if (p->right == n) p->right = n->left;
        else               p->left  = n->left;
        node_delete(n);

    } else {
        /* Node has both children */
    
        // Find smaller child on right branch
        node* pc = n;               // Child's parent
        node* c  = n->right;        // Child
        for (; c->left != NULL; pc = c, c = c->left);

        // Move child data on top
        data_delete(n->data);
        n->data = c->data;
 
        // Remove child
        if (pc == n) n->right = c->right;
        else         pc->left = c->right;

        c->data = NULL;     // So we don't free the data
        node_delete(c); 
    }
}

int sortedcontainer_erase(sortedcontainer* sc, data* data) {

    /* Traverse the tree and remove */
    node* n = sc->root; // Target node
    node* p = NULL;     // Parent node

    while (n != NULL){
        switch (data_compare(data, n->data)){
            case -1: p = n;
                     n = n->left; 
                break;
            case  1: p = n;
                     n = n->right;
                break;
            default: // Delete node
                     _node_removal(p,n);                 
                     return 1;
                break;
        } 
    }
    
    // Node not found
    return 0;
}

int sortedcontainer_contains(sortedcontainer* sc, data* data) {
    node* n = sc->root;

    while (n != NULL){
        switch (data_compare(data, n->data)){
            case -1: n = n->left;
                break;
            case  1: n = n->right;
                break;
            default: return 1;
        } 
    }

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
