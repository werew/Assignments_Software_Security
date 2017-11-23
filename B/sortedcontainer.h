// DO NOT change this value. It does not fix your problems
#define NAME_LENGTH 20

typedef struct {
    int age;
    char name[NAME_LENGTH];
} data;

typedef struct node_s {
    data* data;
    struct node_s* left;
    struct node_s* right;
} node;

typedef struct {
    node* root;
} sortedcontainer;

/**
 * @brief Compares @c d1 and @c d2
 * @param d1 Data to be compared
 * @param d2 Data to be compared
 * @return -1 if d1 < d2, 1 if d2 > 1, 0 otherwise
 */
int data_compare(data* d1, data* d2);

/**
 * @brief Prints the data @c to FILE @c f
 * @param d The data to print
 * @param f The FILE to print to
 */
void data_print(data* d, FILE* f);

/**
 * @brief Creates a new data using the specified @c age and @c name
 * @param age The age
 * @param name The name
 * @return Pointer to a new data instance
 */
data* data_new(int age, char const* name);

/**
 * @brief Deletes the specified data and frees its memory
 * @param d The data to delete
 */
void data_delete(data* d);

/**
 * @brief Creates a new node using the specified data
 * @param d The data that the node will contain
 * @return A pointer to the new node or NULL in case of error
 */
node* node_new(data* d);

/**
 * @brief Deletes the specified node and frees its memory
 * @param d The data to delete
 */
void node_delete(node* n);

/**
 * @brief Creates a new Sorted Container
 * @return Pointer to a new sortedcontainer instance
 */
sortedcontainer* sortedcontainer_new();

/**
 * @brief Inserts @c data into the container, at the end
 * When there is already an element in the container that is equal to
 * @c data, nothing is added. See @c data_compare().
 * @param sc The sortedcontainer into which data is to be inserted
 * @param data The data to be inserted. Claims ownership of data
 */
void sortedcontainer_insert(sortedcontainer* sc, data* data);

/**
 * @brief Erases the element in the container that is equal to @c data
 * @param sc The sortedcontainer of which data is to be erased
 * @param data The data to be erased. Does NOT claim ownership of data
 * @return 0 iff no node was erased, 1 otherwise
 */
int sortedcontainer_erase(sortedcontainer* sc, data* data);

/**
 * @brief Checks whether or not the container contains element that is
 *        equal to @c data
 * @param sc The sortedcontainer to check
 * @param data The data to be checked. Does NOT claim ownership of data
 * @return 0 iff no equal element is in the container, 1 otherwise
 */
int sortedcontainer_contains(sortedcontainer* sc, data* data);

/**
 * @brief Prints the elements in the sortedcontainer
 * @param sc The sortedcontainer to print
 * @param printFile The FILE to print to
 */
void sortedcontainer_print(sortedcontainer* sc, FILE* printFile);

/**
 * @brief Deletes the sortedcontainer and frees all its memory
 * @param sc The sortedcontainer to delete
 */
void sortedcontainer_delete(sortedcontainer* sc);
