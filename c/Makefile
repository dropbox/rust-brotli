
decompressor: decompressor_d decompressor.c target/release/libbrotli.so 
	gcc -O3 -o decompressor -g decompressor.c -I. target/release/libbrotli.so 
decompressor_d: decompressor.c target/debug/libbrotli.so 
	cargo build && gcc -o decompressor_d -g decompressor.c -I. target/debug/libbrotli.so 
target/release/libbrotli.so:
	cargo build --release
target/debug/libbrotli.so:
	cargo build
clean:
	cargo clean && rm -f decompressor decompressor_d
