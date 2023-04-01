TESTS:=$(wildcard tests/*.ktc)
ASM:=$(patsubst %.ktc, %.asm, $(notdir $(TESTS)))
BIN:=$(patsubst %.ktc, %.bin, $(notdir $(TESTS)))

.PHONY: test
test: asm bin

.PHONY: asm
asm: $(ASM) 

%.asm: tests/%.ktc
	cargo run $< > $@

.PHONY: bin
bin: $(BIN) 

%.bin: %.asm
	ktc32-asm $< -o $@

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.bin
