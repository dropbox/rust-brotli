
decompressor: decompressor_d decompressor.c target/release/libbrotli.so
	gcc -O3 -o decompressor -g decompressor.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
decompressor_d: decompressor.c target/debug/libbrotli.so brotli_tool
	cargo build && gcc -o decompressor_d -g decompressor.c -I. -Ltarget/debug -lbrotli -Wl,-rpath=target/debug

brotli_tool: brotli_tool_d decompressor.c target/release/libbrotli.so 
	gcc -O3 -o brotli_tool -g brotli.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
brotli_tool_d: decompressor.c target/debug/libbrotli.so 
	cargo build && gcc -o brotli_tool_d -g brotli.c -I. -Ltarget/debug -lbrotli -Wl,-rpath=target/debug


target/release/libbrotli.so:
	cargo build --release
target/debug/libbrotli.so:
	cargo build
clean:
	cargo clean && rm -f decompressor decompressor_d brotli_tool brotli_tool_d
