# cargo allows us to hook early in the build process but not after the binary's
# compilation, so we need to supply our own build scripting with something, in
# this case I've chosen a Makefile.

no_arg:
	@echo "you probably want 'make build' or 'make test'"
	@false

build:
	cargo build --quiet
	cargo build --quiet --release
	sed -e "s/{RADIX_CALC_README}/$$(grep '^version =' Cargo.toml)/" alfred/info.plist.in > alfred/info.plist
	zip -q -j alfred/radix-calc.alfredworkflow \
	    alfred/info.plist \
	    alfred/1B5B938D-D584-4467-BBB3-9C41F6C212F6.png \
	    alfred/EE844101-D6F4-4115-86FB-8440E16E266C.png \
	    alfred/icons/*.png \
	    target/release/radix-calc

test: build
	cargo test --quiet -- --quiet
	cargo test --quiet --release -- --quiet
	PATH=`pwd`/target/release:`pwd`/tests/bats:$$PATH ./tests/test-binary.bats

clean:
	cargo clean

.PHONY: no_arg build test clean
