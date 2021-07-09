# ParaTest
A multithreaded testing crate for cpp code, written in rust

- [setup](#Setup)
- [input](#Input)
- [usage](#Usage)

## Setup

Add the following to your shell config file.
```sh
export PARATEST_PATH="<path-to-directory>"
```

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
### Name of Interface file

The tool takes the name of a ```hpp``` file as its second argument.
Make sure the file is in the same directory as the tests specification file.

### Implementation file

The tool takes a ```cpp``` file as its third argument argument. The file holds your implementation.

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


Assuming you are in the ```src/``` directory.

The path to the sample test specification file : ```../cpp_files/tests.cpp```

The **name** of the interface file : ```impl.hpp```

The path to the implementation file : ```../cpp_files/impl.cpp```

To run the tester:

```sh
$ cargo run ../cpp_files/tests.cpp impl.hpp ../cpp_files/impl.cpp
```

The output should be similar to:

```rust
Running tests...

test1 PASSED : was completed successfully!

test2 FAILED : a.out: ../cpp_files/tests.cpp:18: void test2(): Assertion `7 == multiply(2, 3)' failed.

test3 PASSED : was completed successfully!

Summary:

Total no of tests run : 3, total PASSED : 2, total FAILED : 1

```
