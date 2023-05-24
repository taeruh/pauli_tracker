## unset all the builtin stuff
# I like to see which rule is applied, and it is probably faster
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables


# compiler
C = gcc


## setting some flags

# optimization
OP = -O3

# warnings
ERR = -Wall -Wextra -pedantic

# language standard
STD = -std=c17

# auto dependecy generation
DEPSGEN = -MMD -MP

# linking
# LINK = -fuse-ld=lld
LINK =

# link time optimization
# LTO = -flto=3  # parallize using 3 jobs (default is 1)
LTO = -flto
# LTO =


## source files, binary name, ...

# source directory
SRC := src

# include directory for the headers
INCLUDE := include/c
# INCLUDE_PRIVATE := include/private


# pauli tracker library
TRACKER_PATH = ../../../target/debug
TRACKER_DYN = -lc_pauli_tracker -L $(TRACKER_PATH) -Wl,-rpath=$(TRACKER_PATH)
TRACKER_STATIC = -Wl,-Bstatic -lc_pauli_tracker -L $(TRACKER_PATH) -Wl,-Bdynamic

# LIBS = $(RUST_DYN) $(RUST_STATIC)
LIBS = $(TRACKER_DYN)
# LIBS = $(RUST_STATIC)
# LIBS = 


## get and set all building files

# source files
SOURCES := $(shell find $(SRC) -name '*.c')

# header files
HEADERS := $(shell find $(INCLUDE) -name '*.h')
HEADERS_PRIVATE := $(shell find $(INCLUDE_PRIVATE) -name '*.h')

# target directory for all build stuff
TARGET = target

# directory for temporary building files
BUILD = $(TARGET)/build

# the final output
OUTPUT := $(TARGET)/main

# temporary files for the building process 
OBJECTS = $(SOURCES:$(SRC)/%.c=$(BUILD)/%.o)
DEPENDENCIES = $(OBJECTS:.o=.d)


## final flags

# preprocessor flags
CPPFLAGS = $(DEPSGEN)

# compiling flags
CFLAGS = -I $(INCLUDE) -I $(INCLUDE_PRIVATE) $(OP) $(ERR) $(STD)

# linking flags
LDFLAGS = $(LINK) $(LIBS)


## building rules

all: $(OUTPUT)


# link all binary objects to the resulting executable
$(OUTPUT): $(OBJECTS)
	$(C) -o $@ $^ $(LDFLAGS)

# include/source the dependencies files (empty on the first build)
-include $(DEPENDENCIES)

# compile the source code
$(BUILD)/%.o: $(SRC)/%.c
	$(C) -c -o $@ $< $(CPPFLAGS) $(CFLAGS)


# release build
$(OUTPUT)_release: $(SOURCES) $(HEADERS)
	$(C) -o $@ $(SOURCES) $(CFLAGS) $(LDFLAGS) $(LTO)



.PHONY: clean clean_temporary setup_build_dir

clean: clean_temporary
	rm -f $(OUTPUT) $(OUTPUT)_release


clean_temporary:
	rm -f $(OBJECTS) $(DEPENDENCIES)

setup_build_dir:
	./setup.bash update_build_directory
