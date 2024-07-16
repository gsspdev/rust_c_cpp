// c_lib/c_lib.c

#include "c_lib.h"
#include "cpp_lib.h"
#include <stdio.h>

int c_function(const char* input) {
	printf("Receieved in C: %s\n", input);
	return cpp_function(input);
}
