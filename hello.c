 int main() { 
  int a = 1, b = 2; 
  { 
    a = 2; 
    int a = 3; 
    b = b + a;
  } 
  return a; 
  return b;
}