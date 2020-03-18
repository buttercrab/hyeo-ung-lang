CARGO = ~/.cargo/bin/cargo

install:
	$(CARGO) build --release
	mkdir -p bin
	cp target/release/hyeong bin/hyeong
	mkdir -p ~/.hyeong
	cd ~/.hyeong && $(CARGO) new hyeong-build --vcs none
	cp src/number.rs ~/.hyeong/hyeong-build/src/
	cp src/big_number.rs ~/.hyeong/hyeong-build/src/
	cargo build --manifest-path="$(HOME)"/.hyeong/hyeong-build/Cargo.toml --release
	@printf "\n\e[01;36m==> done!\e[0m \n"

uninstall:
	rm -rf ~/.hyeong
