# Makefile for $(KFS) kernel
KFS := KFS_1

# Compiler and tools
ASM := nasm
RUSTC := rustup run nightly cargo
LD := ld
GRUB_MKRESCUE := grub-mkrescue
QEMU := qemu-system-i386

# Flags
ASM_FLAGS := -f elf32
RUST_FLAGS := build -Z build-std=core --target i386-unknown-none.json
LD_FLAGS := -m elf_i386 -n -T linker.ld

# Files
ASM_SRC := boot.asm
ASM_OBJ := boot.o
KB_SRC := kb.asm
KB_OBJ := kb.o
EX_SRC := exc.asm
EX_OBJ := exc.o
GDT_SRC := gdt.asm
GDT_OBJ := gdt.o
RUST_LIB := target/i386-unknown-none/debug/libkfs.a
KERNEL := kernel.bin
ISO := kfs.iso

# Directories
ISO_DIR := iso
BOOT_DIR := $(ISO_DIR)/boot
GRUB_DIR := $(BOOT_DIR)/grub

# Default target
all: $(KERNEL)

# Compile assembly bootloader
$(ASM_OBJ): $(ASM_SRC)
	@echo "Assembling $(ASM_SRC)..."
	@$(ASM) $(ASM_FLAGS) $(ASM_SRC) -o $(ASM_OBJ)
	@echo "Assembling $(KB_SRC)..."
	@$(ASM) $(ASM_FLAGS) $(KB_SRC) -o $(KB_OBJ)
	@echo "Assembling $(EX_SRC)..."
	@$(ASM) $(ASM_FLAGS) $(EX_SRC) -o $(EX_OBJ)
	@echo "Assembling $(GDT_SRC)..."
	@$(ASM) $(ASM_FLAGS) $(GDT_SRC) -o $(GDT_OBJ)

# Build Rust kernel library
$(RUST_LIB): src/lib.rs src/vga.rs Cargo.toml
	@echo "Building Rust kernel..."
	@$(RUSTC) $(RUST_FLAGS)

# Link everything into final kernel binary
$(KERNEL): $(ASM_OBJ) $(RUST_LIB) linker.ld
	@echo "Linking kernel..."
	@$(LD) $(LD_FLAGS) -o $(KERNEL) $(ASM_OBJ) $(KB_OBJ) $(EX_OBJ) $(GDT_OBJ) $(RUST_LIB)
	@echo "✓ Kernel built successfully: $(KERNEL)"

# Run kernel in QEMU
run: $(KERNEL)
	@echo "Running kernel in QEMU..."
	@$(QEMU) -kernel $(KERNEL)

# Create bootable ISO with GRUB
iso: $(KERNEL)
	@echo "Creating bootable ISO..."
	@mkdir -p $(GRUB_DIR)
	@cp $(KERNEL) $(BOOT_DIR)/
	@echo 'menuentry "$(KFS)" {' > $(GRUB_DIR)/grub.cfg
	@echo '    multiboot /boot/$(KERNEL)' >> $(GRUB_DIR)/grub.cfg
	@echo '    boot' >> $(GRUB_DIR)/grub.cfg
	@echo '}' >> $(GRUB_DIR)/grub.cfg
	@$(GRUB_MKRESCUE) -o $(ISO) $(ISO_DIR) 2>/dev/null
	@echo "✓ ISO created: $(ISO)"

# Run ISO in QEMU
run-iso: iso
	@echo "Running ISO in QEMU..."
	@$(QEMU) -cdrom $(ISO)

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -f $(ASM_OBJ) $(KERNEL) $(KB_OBJ) $(EX_OBJ) $(GDT_OBJ) *.out	@rm -rf $(ISO_DIR) $(ISO)
	@cargo clean 2>/dev/null || true
	@echo "✓ Clean complete"

# Remove everything including Cargo cache
fclean: clean
	@echo "Deep cleaning..."
	@rm -rf target/
	@echo "✓ Deep clean complete"

# Rebuild everything
re: fclean all

# Help target
help:
	@echo "$(KFS) Kernel Makefile"
	@echo "Usage:"
	@echo "  make         - Build kernel"
	@echo "  make run     - Build and run kernel in QEMU"
	@echo "  make iso     - Create bootable ISO"
	@echo "  make run-iso - Create and run ISO in QEMU"
	@echo "  make clean   - Remove build artifacts"
	@echo "  make fclean  - Deep clean (remove target/)"
	@echo "  make re      - Rebuild everything"

.PHONY: all run iso run-iso clean fclean re help
