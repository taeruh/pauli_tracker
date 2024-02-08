MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

name := minimal_app
sources := app/main.cpp
target := target

std := -std=c++20
err := -Wall -Wextra -pedantic

CXX := clang++

CFLAGS := $(err) $(std) -I ./
LDFLAGS := -L target/debug
LDLIBS := -Wl,-Bstatic -l minimal_lib -Wl,-Bdynamic

run: $(target)/$(name)
	./$<

$(target)/$(name): $(sources)
	$(CXX) -o $@ $^ $(CFLAGS) $(LDFLAGS) $(LDLIBS)
