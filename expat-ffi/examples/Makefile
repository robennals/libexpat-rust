# Build C examples against expat-rust's libexpat
#
# First build the Rust library:
#   cargo build --release -p expat-ffi
#
# Then build and run examples:
#   make
#   make run

LIBDIR = ../../target/release
CC = cc
CFLAGS = -Wall -Wextra -std=c99
LDFLAGS = -L$(LIBDIR) -lexpat

# macOS uses -rpath, Linux uses -Wl,-rpath
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    LDFLAGS += -Wl,-rpath,$(LIBDIR)
else
    LDFLAGS += -Wl,-rpath,$(LIBDIR)
endif

EXAMPLES = basic_parse

all: $(EXAMPLES)

basic_parse: basic_parse.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

run: all
	@echo "--- basic_parse ---"
	./basic_parse

clean:
	rm -f $(EXAMPLES)

.PHONY: all run clean
