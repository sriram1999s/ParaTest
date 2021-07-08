# ParaTest
A multithreaded testing framework for cpp code, written in rust

## Input

The tool takes a ```cpp``` file as its first argument. The file must specify all the test functions .

The test functions must be of ```void``` return type, must not take any parameters and must be prefixed with a multiline comment ```/* test */```

An example of a test specification file is shown below:

```cpp
#include<cassert>

/* test */
void test1() {
  assert(1 == 5);
  std::cout << "test 1\n";
}

int helper() {
  return 2;
}

/* test */
void test2() {
  assert(2 == helper());
  std::cout << "test 2\n";  
}
```

## Functionalities

 - yet to begin
