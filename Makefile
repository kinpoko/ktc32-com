.PHONY: test
test: 
	cargo run 42 > test.asm
	ktc32-asm test.asm -o test.mem
	cargo run '5+20-4' > test.asm
	ktc32-asm test.asm -o test.mem
	cargo run ' 12 + 34 - 5' > test.asm
	ktc32-asm test.asm -o test.mem
	cargo run ' 1 * 2 + 3' > test.asm
	ktc32-asm test.asm -o test.mem
	cargo run ' 1 * (2 + 3)' > test.asm
	ktc32-asm test.asm -o test.mem
	cargo run ' 1 <= (2 + 3)' > test.asm
	ktc32-asm test.asm -o test.mem

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.mem
