#include<cassert>
#include<iostream>

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

int main() {
  // test1();
  test2();
}
