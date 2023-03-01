.PHONY: all
all: build test

.PHONY: build
build:	
	cargo build --release

.PHONY: test
test:
	ktc32-cc '5+20-4' > tmp.asm
	ktc32-asm tmp.asm -o tmp.mem
	easerial file tmp.mem

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.mem
