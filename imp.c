#include <stdio.h>
#include <stdlib.h>

void putint(int x) { printf("%d", x); }

void putch(int x) { printf("%c", (char)x); }

int getint() {
    int x;
    scanf("%d", &x);
    return x;
}