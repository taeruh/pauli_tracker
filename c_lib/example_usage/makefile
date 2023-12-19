# unnecessary complex for the current use case, but it will pay off later
SHELL := /bin/sh
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

name := example_usage
src := src
# include_dir := include/$(name)
# include_private_dir := include/private
target_dir := target
debug_dir := $(target_dir)/debug
release_dir := $(target_dir)/release
debug_build_dir := $(debug_dir)/build
release_build_dir := $(release_dir)/build

sources := $(shell find $(src) -name '*.cpp')
# headers := $(shell find $(include_dir) -name '*.hpp')
# private_headers := $(shell find $(include_private_dir) -name '*.hpp')

debug_objs := $(sources:$(src)/%.cpp=$(debug_build_dir)/%.o)
debug_deps := $(debug_objs:.o=.d)


pauli_tracker := pauli_tracker_clib
pauli_tracker_directory := ../output

pauli_tracker_static := -Wl,-Bstatic -l$(pauli_tracker) -Wl,-Bdynamic
pauli_tracker_dynamic := -l$(pauli_tracker) -Wl,-rpath=$(pauli_tracker_directory)


std := -std=c++20
err := -Wall -Wextra -pedantic
op := -O3
lld := -fuse-ld=lld
lto := -flto
deps_generation := -MMD -MP


CXX := clang++

# HEADER_PATH := -I $(include_dir) -I $(pauli_tracker_directory)
HEADER_PATH := -I $(pauli_tracker_directory)
LD_LIBRARY_PATH := -L $(pauli_tracker_directory)

CPPFLAGS := $(deps_generation)

CFLAGS := $(err) $(std) $(HEADER_PATH)

LDLIBS := $(pauli_tracker_static)
# dynamic is usually a bad idea when it is not installed on the system
# LDLIBS := $(pauli_tracker_dynamic)
LDFLAGS := $(LD_LIBRARY_PATH)


define default_message =
There are a few standard rules
 - build: shorthand for build_debug
 - run: shorthand for run_debug
 - build_debug: build the debug build
 - run_debug: build and run the debug build
 - build_release: build the release build
 - run_release: build and run the release build
 - clean_$(target_dir): remove the $(target_dir) directory
 - clean: currently the same as clean_$(target_dir)
 - clean_full: clean and remove the output directory
 - default: this message
The resulting executables are in $(debug_dir) and $(release_dir), respectively.
endef
export default_message
default:
	@echo "$$default_message"


debug := $(debug_dir)/$(name)
release := $(release_dir)/$(name)


run: run_debug
build: build_debug
run_debug: build_debug
	./$(debug)
run_release: build_release
	./$(release)
build_debug: $(debug)
build_release: $(release)


$(debug): $(debug_objs) # | $(debug_dir)/ # objs ensure existence
	$(CXX) -o $@ $^ $(LDLIBS) $(LDFLAGS)

-include $(debug_deps)

#$(debug_build_dir)/%.o: ... -> after .SECONDEXPANSION 


$(release): $(sources) $(headers) | $(release_dir)/
	$(CXX) -o $@ $(sources) $(CFLAGS) $(op) $(LDFLAGS) $(LDLIBS)
	# $(CXX) -o $@ $(sources) $(CFLAGS) $(op) $(LDFLAGS) $(lto) $(lld) $(LDLIBS)


%/:
	@mkdir -p $@


clean: clean_$(target_dir)
clean_full: clean_$(target_dir)
	@rm -rf output
clean_$(target_dir):
	@rm -rf $(target_dir)


.PHONY := default clean clean_for_real clean_$(target_dir) run build run_debug \
					build_debug run_release build_release


.SECONDEXPANSION:

$(debug_build_dir)/%.o: | $$(@D)/
	$(CXX) -c -o $@ $(src)/$*.cpp $(CPPFLAGS) $(CFLAGS)
