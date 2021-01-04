#include <stdio.h>
#include "libdumb.h" 

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