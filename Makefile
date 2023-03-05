.PHONY: test
test: 
	cargo run 42 > test1.asm
	ktc32-asm test1.asm -o test1.mem
	cargo run '5+20-4' > test2.asm
	ktc32-asm test2.asm -o test2.mem
	cargo run ' 12 + 34 - 5' > test3.asm
	ktc32-asm test3.asm -o test3.mem

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.mem
