CARGO = ~/.cargo/bin/cargo

install:
	$(CARGO) build --release
	mkdir -p bin
	cp target/release/hyeong bin/hyeong
	./bin/hyeong install

uninstall:
	./bin/hyeong uninstall
	rm -rf bin
