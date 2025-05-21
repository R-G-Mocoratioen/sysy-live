int a = 1;

int f(){
    a = a + 1;
    return a;
}

int main() {
  int u = f();
  int v = f();
  putint(u + v);
  return 0;
}
