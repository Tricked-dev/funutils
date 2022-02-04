target=x86_64-unknown-linux-gnu

build: 
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu &> /dev/null; \
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target ${target} --release
out:
	make -B  build; \
	rm -rf build/; \
	mkdir build/; \
	cp target/${target}/release/* build/ &> /dev/null; \
	rm -rf build/*.*; \
	rm -rf build/*.rlib; \
	strip build/* ; \
	upx --best --lzma build/*
	