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
# you might need to add the following libraries to the LDLIBS
# -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc

run: $(target)/$(name)
	./$<

$(target)/$(name): $(sources)
	$(CXX) -o $@ $^ $(CFLAGS) $(LDFLAGS) $(LDLIBS)
