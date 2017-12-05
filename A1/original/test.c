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

int test4(FILE* printFile) {
    (void)printFile;

    // Add a test in the style of test3 to test the deletion of a node with
    // two children
    ASSERT(0, "implement this test");

    return 0;
}

int test5(FILE* printFile) {
    (void)printFile;

    // Add a test in the style of test3 to test the deletion of a node with
    // two children, who each have two children as well
    ASSERT(0, "implement this test");

    return 0;
}

// If you want to add test6 and onwards, create the test6 function above and
// add it to this list
test_t tests[] = {test1, test2, test3, test4, test5};

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
