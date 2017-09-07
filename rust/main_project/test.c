/*
  C applications making calls to c static lib made by rust

BUILD:
gcc test.c ../staticlib/target/debug/libstaticlib.a -o test -lpthread -ldl
*/

#include <stdio.h>
#include <pthread.h>

// redeclared rust structure Point
typedef struct {
    int x;
    int y;
} Point;

// struct without definition, will be passed to library functions by pointer
extern struct complicated_rust_struct;

//extern functions defined in staticlib in rust
extern int increment(int);
extern int add_point_values(Point);
extern void use_struct_fields_function(struct complicated_rust_struct*);
extern struct complicated_rust_struct* create_struct_ptr_function();
extern void free_struct_ptr_function(struct complicated_rust_struct*);


int main() {
    printf("Calling rust function: %d\n", increment(0));
    Point p = {1, 1};
    printf("Calling rust function using redeclared structure: %d\n", add_point_values(p));

    printf("Calling rust library to create object ptr and calling function using it, freed in the end.\n");
    struct complicated_rust_struct* ptr_strc;
    ptr_strc = create_struct_ptr_function();
    use_struct_fields_function(ptr_strc);
    free_struct_ptr_function(ptr_strc);

    return 0;
}
