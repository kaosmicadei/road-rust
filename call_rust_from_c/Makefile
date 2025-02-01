RUST_LIB = rust_from_c
LIB_DIR = ./target/debug/

.PHONY: all call_rust

call_rust: call_rust.c
	clang -o $@ $< -l$(RUST_LIB) -L$(LIB_DIR)
 
