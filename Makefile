target=x86_64-unknown-linux-gnu

build: 
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target ${target} --release ; \
	strip target/${target}/release/* ; \
	upx --best --lzma target/${target}/release/*
prod:
	make build; \
	rm -rf build; \
	mkdir build; \
	mv target/${target}/release/!(*.*) build/
