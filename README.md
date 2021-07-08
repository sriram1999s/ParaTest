# ParaTest
A multithreaded testing framework for cpp code, written in rust

- [input](#Input)
- [usage](#Usage)

## Input

### Test Specification File

The tool takes a ```cpp``` file as its first argument. The file must specify all the test functions .

The test functions must be of ```void``` return type, must not take any parameters and must be prefixed with a multiline comment ```/* test */```. Functions which do not fit this template will be ignored and not considered as tests.

A test function must cause a segmentation fault to be marked as a failure. Use of assert statements is advised.

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

### Implementation file

The tool takes a ```cpp``` file as its second argument argument. The file holds your implementation.

An example of an implementation file is shown below:

```cpp
#include<iostream>

int sum(int a, int b) {
  return a + b;
}

int multiply(int a, int b) {
  return a * b;
}

int mod(int a, int b) {
  return a % b;
}

```

## Usage

```python
print("Yet to complete :D")
```
