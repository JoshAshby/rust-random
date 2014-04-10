#All the rust!
RUST_SRC=$(wildcard *.rs)
RUST_COMPILED_DIR="compiled/"

normal: $(RUST_SRC)
	mkdir -p $(RUST_COMPILED_DIR)
	rustc --out-dir $(RUST_COMPILED_DIR) $?

test: $(RUST_SRC)
	rustc --test --out-dir $(RUST_COMPILED_DIR) $?

docs: $(RUST_SRC)
	rustdoc $?

all: normal

clean:
	rm -rf $(RUST_COMPILED_DIR)
