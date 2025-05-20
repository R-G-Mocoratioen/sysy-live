const int x = 2;
int a[x] = {x, x};
const int b[x][x] = {2, 1, 0};
int c[x + 1][x][x] = {2, 1, 0, 3, {1}, {2, 3, {2}}};
int d[x * x][x * x][x * x];
int e[105] = {1, 1};

int f(int a[][2][2]) {
    return a[0][0][0] + a[0][1][1];
}

int g(int a[][2]) {
    return a[0][0] + a[0][1];
}

int h(int a[]) {
    return a[0] + a[1];
}

int main() {
    int i = 2;
    while (i < 20) {
        e[i] = e[i - 1] + e[i - 2];
        i = i + 1;
    }
    putint(e[i - 1]);
    putint(f(c));
    putint(g(c[2]));
    putint(h(c[2][1]));
    int p = 0;
    while (p < 4) {
        int q = 0;
        while (q < 4) {
            int r = 0;
            while (r < 4) {
                d[p][q][r] = c[p % 3][q % 2][r % 2];
                r = r + 1;
            }
            q = q + 1;
        }
        p = p + 1;
    }
    return 0;
}