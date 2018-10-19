multiexample: multiexample_d decompressor multiexample.c vec_u8.h arg.h custom_alloc.h
	gcc -O3 -o multiexample -g multiexample.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
multiexample_d: decompressor multiexample.c vec_u8.h arg.h custom_alloc.h
	gcc -o multiexample_d -g multiexample.c -I. target/debug/libbrotli.[sd]?*
decompressor: decompressor_d decompressor.c target/release/libbrotli.so catbrotli
	gcc -O3 -o decompressor -g decompressor.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
decompressor_d: decompressor.c target/debug/libbrotli.so brotli_tool catbrotli_d
	cargo build && gcc -o decompressor_d -g decompressor.c -I. -Ltarget/debug -lbrotli -Wl,-rpath=target/debug

brotli_tool: brotli_tool_d decompressor.c target/release/libbrotli.so 
	gcc -O3 -o brotli_tool -g brotli.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
brotli_tool_d: decompressor.c target/debug/libbrotli.so 
	cargo build && gcc -o brotli_tool_d -g brotli.c -I. -Ltarget/debug -lbrotli -Wl,-rpath=target/debug


catbrotli: catbrotli_d decompressor.c catbrotli.c target/release/libbrotli.so 
	gcc -O3 -o catbrotli -g catbrotli.c -I. -Ltarget/release -lbrotli -Wl,-rpath=target/release
catbrotli_d: catbrotli.c decompressor.c target/debug/libbrotli.so 
	cargo build && gcc -o catbrotli_d -g catbrotli.c -I. -Ltarget/debug -lbrotli -Wl,-rpath=target/debug




target/release/libbrotli.so:
	cargo build --release
target/debug/libbrotli.so:
	cargo build
clean:
	cargo clean && rm -f multiexample multiexample_d catbrotli catbrotli_d decompressor decompressor_d brotli_tool brotli_tool_d
