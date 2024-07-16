// cpp_lib/cpp_lib.cpp

#include "cpp_lib.h"
#include <iostream>

int cpp_function(const char* input) {
	std::cout << "Received in C++: " << input << std::endl;
	return 42; // Arbitrary return value
}
