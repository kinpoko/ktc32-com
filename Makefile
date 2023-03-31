.PHONY: all
all: test1 test2 test3

test1:
	cargo run 'a = 1; b = 2; return a + b;' > test1.asm
	ktc32-asm test1.asm -o test1.mem

test2:
	cargo run 'a = 3; if (a == 1) return 1; return 2;' > test2.asm
	ktc32-asm test2.asm -o test2.mem

test3:
	cargo run 'a = 1; if (a == 1) a = a + 1; else a = a * 3; return a;' > test3.asm
	ktc32-asm test3.asm -o test3.mem

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.mem
