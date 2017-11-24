#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#include "test.h"
#include "sortedcontainer.h"

typedef int (*test_t)(FILE* );

#define ASSERT(c,s) do { \
    if(!(c)) { \
        fprintf(printFile, "ASSERT FAILED[%s:%i] %s: %s\n", __FILE__, __LINE__, #c, s); \
        return 1; \
    } \
} while(0)

int test1(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root == NULL, "root is not NULL");

    sortedcontainer_delete(sc);

    return 0;
}

int test2(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root == NULL, "root is not NULL");

    data* dat = data_new(10, "aap");
    sortedcontainer_insert(sc, dat);

    ASSERT(sc->root != NULL, "root is NULL");
    ASSERT(sc->root->data != NULL, "root->data is NULL");
    ASSERT(!data_compare(dat, sc->root->data), "data is not equivalent");
    ASSERT(dat == sc->root->data, "data is not the same instant");
    ASSERT(sc->root->left == NULL, "root->data->left is not NULL");
    ASSERT(sc->root->right == NULL, "root->data->right is not NULL");

    data* dataDifferent = data_new(20, "noot");
    data* dataCopy = data_new(10, "aap");

    ASSERT(sortedcontainer_contains(sc, dataCopy), "data is not in the container (sortedcontainer_contains)");
    ASSERT(!sortedcontainer_contains(sc, dataDifferent), "data should not be in the container");

    sortedcontainer_erase(sc, dataDifferent);

    ASSERT(sc->root->data != NULL, "data wrongly deleted");
    ASSERT(!data_compare(dat, sc->root->data), "data wrongly deleted");
    ASSERT(dat == sc->root->data, "data wrongly deleted");
    ASSERT(sortedcontainer_contains(sc, dataCopy), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(!sortedcontainer_contains(sc, dataDifferent), "data should not be in the container");

    sortedcontainer_erase(sc, dataCopy);

    ASSERT(sc->root == NULL, "data not deleted");

    sortedcontainer_delete(sc);

    data_delete(dataDifferent);
    data_delete(dataCopy);

    return 0;
}

int test3(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    data* aap = data_new(10, "aap");
    data* noot = data_new(20, "noot");
    data* mies = data_new(15, "mies");

    sortedcontainer_insert(sc, aap);
    sortedcontainer_insert(sc, noot);
    sortedcontainer_insert(sc, mies);

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root != NULL, "root is NULL");
    ASSERT(sc->root->data != NULL, "root->data is NULL");

    ASSERT(!data_compare(aap, sc->root->data), "data is not equivalent");
    ASSERT(aap == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(noot, sc->root->right->data), "data is not equivalent");
    ASSERT(noot == sc->root->right->data, "data is not the same instant");

    ASSERT(!data_compare(mies, sc->root->right->left->data), "data is not equivalent");
    ASSERT(mies == sc->root->right->left->data, "data is not the same instant");

    sortedcontainer_erase(sc, noot);

    ASSERT(!data_compare(aap, sc->root->data), "data is not equivalent");
    ASSERT(aap == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(mies, sc->root->right->data), "data is not deleted");
    ASSERT(mies == sc->root->right->data, "data is not deleted");

    ASSERT(sc->root->right->left == NULL, "left child of mies' node is not NULL");
    ASSERT(sc->root->right->right == NULL, "right child of mies' node is not NULL");

    ASSERT(sortedcontainer_contains(sc, aap), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, mies), "data is not in the container anymore (sortedcontainer_contains)");

    sortedcontainer_delete(sc);

    return 0;
}

// Test the deletion of a node with two children twice (if node is root, or not)
int test4(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    data* a = data_new(10, "a");
    data* b = data_new(15, "b");
    data* c = data_new(20, "c");
    data* d = data_new(25, "d");
    data* e = data_new(30, "e");

    sortedcontainer_insert(sc, b);  
    sortedcontainer_insert(sc, a);  
    sortedcontainer_insert(sc, d);  
    sortedcontainer_insert(sc, c);  
    sortedcontainer_insert(sc, e);  

    /**
     *  1. Test tree's initial state:
     *
     *               b 
     *              / \
     *             a   d 
     *                / \
     *               c   e     
     */

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root != NULL, "root is NULL");
    ASSERT(sc->root->data != NULL, "root->data is NULL");

    ASSERT(!data_compare(b, sc->root->data), "data is not equivalent");
    ASSERT(b == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(d, sc->root->right->data), "data is not equivalent");
    ASSERT(d == sc->root->right->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->right->left->data), "data is not equivalent");
    ASSERT(c == sc->root->right->left->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->right->right->data), "data is not equivalent");
    ASSERT(e == sc->root->right->right->data, "data is not the same instant");


    /**
     *  2. Test tree after deleting d:
     *
     *               b 
     *              / \
     *             a   e 
     *                / 
     *               c 
     */
    sortedcontainer_erase(sc, d);

    ASSERT(!data_compare(b, sc->root->data), "data is not equivalent");
    ASSERT(b == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->right->data), "data is not equivalent");
    ASSERT(e == sc->root->right->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->right->left->data), "data is not equivalent");
    ASSERT(c == sc->root->right->left->data, "data is not the same instant");

    ASSERT(sc->root->right->right == NULL, "right child of e' node is not NULL");

    ASSERT(sortedcontainer_contains(sc, a), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, b), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, c), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, e), "data is not in the container anymore (sortedcontainer_contains)");

    /**
     *  3. Test tree after deleting b (the root):
     *
     *               c 
     *              / \
     *             a   e 
     */
    sortedcontainer_erase(sc, b);

    ASSERT(!data_compare(c, sc->root->data), "data is not equivalent");
    ASSERT(c == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->right->data), "data is not equivalent");
    ASSERT(e == sc->root->right->data, "data is not the same instant");

    ASSERT(sc->root->right->left  == NULL, "left child of e' node is not NULL");
    ASSERT(sc->root->right->right == NULL, "right child of e' node is not NULL");

    ASSERT(sortedcontainer_contains(sc, a), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, c), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, e), "data is not in the container anymore (sortedcontainer_contains)");

    sortedcontainer_delete(sc);

    return 0;
}

// Test the deletion of a node with two children, who each have two children as well
int test5(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    data* a = data_new(10, "a");
    data* b = data_new(15, "b");
    data* c = data_new(20, "c");
    data* d = data_new(25, "d");
    data* e = data_new(30, "e");
    data* f = data_new(35, "f");
    data* g = data_new(40, "g");

    sortedcontainer_insert(sc, d);  
    sortedcontainer_insert(sc, b);  
    sortedcontainer_insert(sc, f);  
    sortedcontainer_insert(sc, a);  
    sortedcontainer_insert(sc, c);  
    sortedcontainer_insert(sc, e);  
    sortedcontainer_insert(sc, g);  

    /**
     *  1. Test tree's initial state:
     *
     *               d 
     *              / \
     *            b     f 
     *           / \   / \
     *          a   c e   g
     */

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root != NULL, "root is NULL");
    ASSERT(sc->root->data != NULL, "root->data is NULL");

    ASSERT(!data_compare(d, sc->root->data), "data is not equivalent");
    ASSERT(d == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(b, sc->root->left->data), "data is not equivalent");
    ASSERT(b == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(f, sc->root->right->data), "data is not equivalent");
    ASSERT(f == sc->root->right->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->left->right->data), "data is not equivalent");
    ASSERT(c == sc->root->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->right->left->data), "data is not equivalent");
    ASSERT(e == sc->root->right->left->data, "data is not the same instant");

    ASSERT(!data_compare(g, sc->root->right->right->data), "data is not equivalent");
    ASSERT(g == sc->root->right->right->data, "data is not the same instant");

    /**
     *  2. Test tree after deleting d:
     *
     *               e 
     *              / \
     *            b     f 
     *           / \     \
     *          a   c     g
     */

    sortedcontainer_erase(sc, d);

    ASSERT(!data_compare(e, sc->root->data), "data is not equivalent");
    ASSERT(e == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(b, sc->root->left->data), "data is not equivalent");
    ASSERT(b == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(f, sc->root->right->data), "data is not equivalent");
    ASSERT(f == sc->root->right->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->left->right->data), "data is not equivalent");
    ASSERT(c == sc->root->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(g, sc->root->right->right->data), "data is not equivalent");
    ASSERT(g == sc->root->right->right->data, "data is not the same instant");

    ASSERT(sc->root->right->left == NULL, "left child of f' node is not NULL");

    ASSERT(sortedcontainer_contains(sc, a), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, b), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, c), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, e), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, f), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, g), "data is not in the container anymore (sortedcontainer_contains)");

    sortedcontainer_delete(sc);

    return 0;
}



// Test the deletion of a non-root node with two children, who each have two children as well
int test6(FILE* printFile) {
    (void)printFile;

    sortedcontainer* sc = sortedcontainer_new();

    data* a = data_new(10, "a");
    data* b = data_new(15, "b");
    data* c = data_new(20, "c");
    data* d = data_new(25, "d");
    data* e = data_new(30, "e");
    data* f = data_new(35, "f");
    data* g = data_new(40, "g");
    data* h = data_new(45, "h");

    sortedcontainer_insert(sc, h);  
    sortedcontainer_insert(sc, d);  
    sortedcontainer_insert(sc, b);  
    sortedcontainer_insert(sc, f);  
    sortedcontainer_insert(sc, a);  
    sortedcontainer_insert(sc, c);  
    sortedcontainer_insert(sc, e);  
    sortedcontainer_insert(sc, g);  

    /**
     *  1. Test tree's initial state:
     *
     *                 h
     *                /
     *               d 
     *              / \
     *            b     f 
     *           / \   / \
     *          a   c e   g
     */

    ASSERT(sc != NULL, "failed to create sorted container");
    ASSERT(sc->root != NULL, "root is NULL");
    ASSERT(sc->root->data != NULL, "root->data is NULL");

    ASSERT(!data_compare(h, sc->root->data), "data is not equivalent");
    ASSERT(h == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(d, sc->root->left->data), "data is not equivalent");
    ASSERT(d == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(b, sc->root->left->left->data), "data is not equivalent");
    ASSERT(b == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(f, sc->root->left->right->data), "data is not equivalent");
    ASSERT(f == sc->root->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->left->left->right->data), "data is not equivalent");
    ASSERT(c == sc->root->left->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->left->right->left->data), "data is not equivalent");
    ASSERT(e == sc->root->left->right->left->data, "data is not the same instant");

    ASSERT(!data_compare(g, sc->root->left->right->right->data), "data is not equivalent");
    ASSERT(g == sc->root->left->right->right->data, "data is not the same instant");

    /**
     *  2. Test tree after deleting d:
     *
     *                 h
     *                /
     *               e 
     *              / \
     *             b   f 
     *            / \   \
     *           a   c   g
     */

    sortedcontainer_erase(sc, d);

    ASSERT(!data_compare(h, sc->root->data), "data is not equivalent");
    ASSERT(h == sc->root->data, "data is not the same instant");

    ASSERT(!data_compare(e, sc->root->left->data), "data is not equivalent");
    ASSERT(e == sc->root->left->data, "data is not the same instant");

    ASSERT(!data_compare(b, sc->root->left->left->data), "data is not equivalent");
    ASSERT(b == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(f, sc->root->left->right->data), "data is not equivalent");
    ASSERT(f == sc->root->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(a, sc->root->left->left->data), "data is not equivalent");
    ASSERT(a == sc->root->left->left->data, "data is not the same instant");

    ASSERT(!data_compare(c, sc->root->left->left->right->data), "data is not equivalent");
    ASSERT(c == sc->root->left->left->right->data, "data is not the same instant");

    ASSERT(!data_compare(g, sc->root->left->right->right->data), "data is not equivalent");
    ASSERT(g == sc->root->left->right->right->data, "data is not the same instant");

    ASSERT(sc->root->left->right->left == NULL, "left child of f' node is not NULL");

    ASSERT(sortedcontainer_contains(sc, a), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, b), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, c), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, e), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, f), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, g), "data is not in the container anymore (sortedcontainer_contains)");
    ASSERT(sortedcontainer_contains(sc, h), "data is not in the container anymore (sortedcontainer_contains)");

    sortedcontainer_delete(sc);

    return 0;
}





// If you want to add test7 and onwards, create the test7 function above and
// add it to this list
test_t tests[] = {test1, test2, test3, test4, test5, test6};

void test(FILE* printFile) {
    fprintf(printFile, "Testing...\n");
    int max = sizeof(tests)/sizeof(*tests);
    int lmax = 1 + log10(max);
    for(int i = 0; i < max; ++i) {
        int r = tests[i](printFile);
        fprintf(printFile, "[%*i/%i] ", lmax, i+1, max);
        if(r) {
            fprintf(printFile, "FAIL\n");
        } else {
            fprintf(printFile, "PASS\n");
        }
    }
}
