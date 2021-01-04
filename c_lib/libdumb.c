#include <stdio.h>
#include <stdarg.h>
#include "libdumb.h" 

extern int globaltest = 47;
rustcb globalcb = NULL;
rustobjcb objcb = NULL;
void * objptrforcb = NULL;

void printsomething() {
    printf("print something\n");
}

void printthis(const char *s) {
    printf("printing : %s\n", s);
}

void changefirstfive(int *ints) {
    int *ptr = ints;
    int i;
    for (i = 0; i < 5; i++) {
        *ptr = 9999;
        ++ptr;
    }
}

int32_t register_cb(rustcb cb) {
    printf("registering callback\n");
    globalcb = cb;
    printf("finished registering\n");
}

void call_cb() {
    printf("calling cb\n");
    globalcb(999);
    printf("finished calling\n");
}

void register_objcb(void * objptr, rustobjcb cb) {
    objcb = cb;
    objptrforcb = objptr;
}

void call_objcb() {
    objcb(objptrforcb, 101);
}

int variadictest(int count, ...) {
    va_list varargs;
    int i, sum;
    va_start(varargs, count);
    sum = 0;
    for (i = 0; i < count; i++) {
        sum += va_arg(varargs, int);
    }
    va_end(varargs);
    return sum;
}