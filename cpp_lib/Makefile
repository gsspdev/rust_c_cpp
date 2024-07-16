# cpp_lib/Makefile

CXX = g++
CXXFLAGS = -fPIC

all: libcpp_lib.a

libcpp_lib.a: cpp_lib.o
	$(AR) rcs $@ $^

cpp_lib.o: cpp_lib.cpp cpp_lib.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a
