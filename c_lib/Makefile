# c_lib/Makefile

CC = gcc
CFLAGS = -fPIC -I../cpp_lib
LDFLAGS = -L../cpp_lib -lcpp_lib

all: libc_lib.a

libc_lib.a: c_lib.o
	$(AR) rcs $@ $^

c_lib.o: c_lib.c c_lib.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a
