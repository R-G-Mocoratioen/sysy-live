#include <bits/stdc++.h>
using namespace std;
int main() {
	system("\"C:\\tools\\ghc-9.8.2\\mingw\\bin\\llc.exe\" -o hello.s hello.llvm");
	system("gcc -c hello.s -o hello.o");
	system("g++ -c imp.cpp -o imp.o");
	system("g++ imp.o hello.o -o hello");
	system("hello");
}