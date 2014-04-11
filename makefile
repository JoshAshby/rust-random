# All the rust!
RUST_SRC=$(wildcard *.rs)
RUST_COMPILED_DIR="build/"
RUST_NAMES=$(RUST_SRC:.rs=)
RESULTING_NAMES=$(addprefix build/, $(RUST_NAMES))

all: $(RUST_COMPILED_DIR) build/hello build/classes build/enums_classes
	@echo $(RUST_SRC)
	@echo $(RESULTING_NAMES)

$(RUST_COMPILED_DIR):
	mkdir -p $(RUST_COMPILED_DIR)

$(RESULTING_NAMES): build/%: %.rs
	rustc --out-dir $(RUST_COMPILED_DIR) $<

clean:
	rm -rf $(RUST_COMPILED_DIR)
