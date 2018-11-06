multiexample: multiexample_d decompressor multiexample.c vec_u8.h arg.h custom_alloc.h fuzz fuzz_d
	gcc -O3 -o multiexample -g multiexample.c -I. target/release/libbrotli_ffi.[ds][lyo]*
multiexample_d: decompressor multiexample.c vec_u8.h arg.h custom_alloc.h
	gcc -o multiexample_d -g multiexample.c -I. target/debug/libbrotli_ffi.[ds][lyo]*
decompressor: decompressor_d decompressor.c target/release/libbrotli.so catbrotli
	gcc -O3 -o decompressor -g decompressor.c -I. target/release/libbrotli_ffi.[ds][lyo]*
decompressor_d: decompressor.c target/debug/libbrotli.so brotli_tool catbrotli_d
	cargo build && gcc -o decompressor_d -g decompressor.c -I. target/debug/libbrotli_ffi.[ds][lyo]*
fuzz: fuzz_d fuzz.c target/release/libbrotli.so catbrotli
	gcc -O3 -o fuzz -g fuzz.c -I. target/release/libbrotli_ffi.[ds][lyo]*
fuzz_d: fuzz.c target/debug/libbrotli.so brotli_tool catbrotli_d
	cargo build  && gcc -o fuzz_d -g fuzz.c -I. target/debug/libbrotli_ffi.[ds][lyo]*

brotli_tool: brotli_tool_d decompressor.c target/release/libbrotli.so 
	gcc -O3 -o brotli_tool -g brotli.c -I. target/release/libbrotli_ffi.[ds][lyo]*
brotli_tool_d: decompressor.c target/debug/libbrotli.so 
	cargo build  && gcc -o brotli_tool_d -g brotli.c -I. target/debug/libbrotli_ffi.[ds][lyo]*


catbrotli: catbrotli_d decompressor.c catbrotli.c target/release/libbrotli.so 
	gcc -O3 -o catbrotli -g catbrotli.c -I. target/release/libbrotli_ffi.[ds][lyo]*
catbrotli_d: catbrotli.c decompressor.c target/debug/libbrotli.so 
	cargo build  && gcc -o catbrotli_d -g catbrotli.c -I. target/debug/libbrotli_ffi.[ds][lyo]*




target/release/libbrotli.so:
	cargo build --release 
target/debug/libbrotli.so:
	cargo build 
clean:
	cargo clean && rm -f multiexample multiexample_d catbrotli catbrotli_d decompressor decompressor_d brotli_tool brotli_tool_d
