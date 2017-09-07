#include <stdlib.h>

/*
c staticlib, will be used by rust application

gcc -c -o testlibc.o testlibc.c -fPIC
ar rcs libtestlibc.a testlibc.o
*/

typedef struct {
    int x;
    int y;
} c_point;

typedef struct {
    int x;
} c_opaque_obj;

typedef enum {
    RED,
    GREEN,
    BLUE,
} c_enum;

int c_multiply_four(int num) {
    return num * 4;
}

int c_add_point_values(c_point p){
    return p.x + p.y;
}

const char * c_get_enum_string(c_enum e) {
    switch(e){
        case RED:
            return "RED";
        case GREEN:
            return "GREEN";
        case BLUE:
            return "BLUE";
    }
}

c_opaque_obj c_create_obj(int x){
    c_opaque_obj obj;
    obj.x = x;
    return obj;
}

c_opaque_obj* c_malloc_obj(int x){
    c_opaque_obj* ptr = malloc(sizeof(c_opaque_obj));
    ptr->x = x;
    return ptr;
}

void* c_malloc_void_obj(int x){
    c_opaque_obj* ptr = malloc(sizeof(c_opaque_obj));
    ptr->x = x;
    return (void*)ptr;
}

int c_get_obj_val(c_opaque_obj obj){
    return obj.x;
}

int c_get_obj_ptr_val(c_opaque_obj* obj){
    return obj->x;
}

int c_get_obj_ptr_void_val(void* obj){
    c_opaque_obj* ptr = obj;
    return ptr->x;
}
