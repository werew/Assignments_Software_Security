#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sortedcontainer.h"


/**
 * BUGS FIXED:
 *  - Specification mismatch:
 *      the specifications indicate that this function would return
 *      always -1 if d1 < d2, 1 if d2 > d1 and 0 otherwise. On the
 *      other hand this was not always the case given that strcmp
 *      could possibly return other values.
 */
int data_compare(data* d1, data* d2) {
    assert(d1);
    assert(d2);
   
    // Age has the priority 
    if(d1->age < d2->age) return -1;
    if(d1->age > d2->age) return 1;

    // ...and then we compare the name
    int cmp = strcmp(d1->name, d2->name);
    if (cmp < 0) return -1;
    if (cmp > 0) return  1;
    return 0;
}

// Do not change
void data_print(data* d, FILE* f) {
    fprintf(f, "%i %s", d->age, d->name);
}


/**
 * BUGS FIXED:
 * - NULL pointer dereferencing:
 *      no check on the return value of malloc was performed
 */      
data* data_new(int age, char const* name) {
    data* d = (data*)malloc(sizeof(data));  
    if (d == NULL) return NULL;
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
    if (n == NULL) return; // Same behaviour as free
    data_delete(n->data);
    free(n);
}


/**
 * BUGS FIXED:
 *  - NULL pointer dereferencing:
 *      no check was performed against the return value of malloc
 */
sortedcontainer* sortedcontainer_new() {
    sortedcontainer* d = (sortedcontainer*)malloc(sizeof(sortedcontainer));
    if (d == NULL) return NULL;
    d->root = NULL; 
    return d;
}


/**
 * OBSERVATIONS:
 *  - Missing return value:
 *      this function doesn't provide any feedback about whether the insertion
 *      has been successful or not, which in a real application could pose
 *      serious usability concerns
 */
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
        int cmp = data_compare(data, p->data);

        if (cmp < 0){
            // Insert on the left
            if (p->left == NULL){
                p->left = n;
                return;
            }
            p = p->left;

        } else if (cmp > 0){
            // Insert on the right
            if (p->right == NULL){
                p->right = n;
                return;
            }
            p = p->right;

        } else {                
            // Node is already inside the tree
            node_delete(n);
            return;     
        } 
    }
}


// Note: n is the root (ie. sc->root == n) iff p == NULL (ie. n has no parent)
static void _node_removal(sortedcontainer* sc, node* p, node* n){

    if (n->left  == NULL){
        /* Node has right child or is a leaf */
        if (p == NULL)          sc->root = n->right;
        else if (p->right == n) p->right = n->right;
        else                    p->left  = n->right;
        node_delete(n);

    } else if (n->right == NULL){
        /* Node has left child */
        if (p == NULL)          sc->root = n->left;
        else if (p->right == n) p->right = n->left;
        else                    p->left  = n->left;
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
        int cmp = data_compare(data, n->data);
    
        if (cmp < 0) {
            p = n;
            n = n->left; 

        } else if (cmp > 0) {
            p = n;
            n = n->right;

        } else {
            // Delete node
            _node_removal(sc,p,n);
            return 1;
        }
        
    }
    
    // Node not found
    return 0;
}

int sortedcontainer_contains(sortedcontainer* sc, data* data) {
    node* n = sc->root;

    while (n != NULL){
        int cmp = data_compare(data, n->data);

        if (cmp < 0){
            n = n->left;

        } else if (cmp > 0){
            n = n->right;

        } else {
            // Node found !
            return 1;
        } 
    }

    // Node not found 
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
