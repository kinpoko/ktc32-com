TESTS:=$(wildcard tests/*.c)
ASM:=$(patsubst %.c, %.asm, $(notdir $(TESTS)))
BIN:=$(patsubst %.c, %.bin, $(notdir $(TESTS)))

.PHONY: test
test: asm bin

.PHONY: asm
asm: $(ASM) 

%.asm: tests/%.c
	cargo run $< > $@

.PHONY: bin
bin: $(BIN) 

%.bin: %.asm
	ktc32-asm $< -o $@

.PHONY: clean
clean:
	rm -rf *.asm
	rm -rf *.bin
