# Build and run C integration tests for expat-ffi
#
# Usage:
#   cargo build --release -p expat-ffi   # build the Rust library first
#   make -C expat-ffi/tests              # build and run tests

LIBDIR = ../../target/release
CC = cc
CFLAGS = -Wall -Wextra -std=c99
LDFLAGS = -L$(LIBDIR) -lexpat

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    LDFLAGS += -Wl,-rpath,$(LIBDIR)
else
    LDFLAGS += -Wl,-rpath,$(LIBDIR)
endif

test: test_ffi
	./test_ffi

test_ffi: test_ffi.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

clean:
	rm -f test_ffi

.PHONY: test clean
