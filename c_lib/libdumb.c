#include <stdio.h>
#include "libdumb.h" 

extern int globaltest = 47;
rustcb globalcb = NULL;

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