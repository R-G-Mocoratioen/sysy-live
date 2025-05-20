const int b = 2, d = b + 1;
int a = 1, c = d - b;

int g(int x, int y, int z) {
    const int w = 10 + 11;
    return w - x + (y * z) + (d / b) + (a % c);
}

void f() {
    const int k = 10 + 11, y = k;
    int x = y, z = x * x;
    putint(g(x, y, z));
}


int main() {
    f();
    int x = g(2, 3, 4);
    while (x < 10) {
        x = x + 1;
    }
    putint(g(x, g(x, x, x), x) - x);
    return 0;
}