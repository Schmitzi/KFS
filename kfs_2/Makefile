# Makefile for $(KFS) kernel
KFS := KFS_2

# Compiler and tools
ASM := nasm
RUSTC := rustup run nightly cargo
LD := ld
GRUB_MKRESCUE := grub-mkrescue
QEMU := qemu-system-i386

# Flags
ASM_FLAGS := -f elf32
RUST_FLAGS := build -Z build-std=core --target kfs/i386-unknown-none.json
LD_FLAGS := -m elf_i386 -n -T linker.ld

# Directories
KFS_DIR := kfs
OBJ_DIR := obj
ISO_DIR := iso
BOOT_DIR := $(ISO_DIR)/boot
GRUB_DIR := $(BOOT_DIR)/grub

# Files
ASM_SRC := $(KFS_DIR)/boot.asm
ASM_OBJ := $(OBJ_DIR)/boot.o
KB_SRC := $(KFS_DIR)/kb.asm
KB_OBJ := $(OBJ_DIR)/kb.o
EX_SRC := $(KFS_DIR)/exc.asm
EX_OBJ := $(OBJ_DIR)/exc.o
GDT_SRC := $(KFS_DIR)/gdt.asm
GDT_OBJ := $(OBJ_DIR)/gdt.o
LIB_RS := $(KFS_DIR)/src/lib.rs
VGA_RS := $(KFS_DIR)/src/vga.rs
LINKER := $(KFS_DIR)/linker.ld
CARGO := $(KFS_DIR)/Cargo.toml
RUST_LIB := $(KFS_DIR)/target/i386-unknown-none/debug/libkfs.a
KERNEL := kernel.bin
ISO := kfs.iso


# Default target
all: setup $(KERNEL)

setup:
	@mkdir -p obj

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
$(RUST_LIB): $(LIB_RS) $(VGA_RS) $(CARGO)
	@echo "Building Rust kernel..."
	@$(RUSTC) $(RUST_FLAGS)

# Link everything into final kernel binary
$(KERNEL): $(ASM_OBJ) $(RUST_LIB) $(LINKER)
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
	@rm -f $(ASM_OBJ) $(KERNEL) $(KB_OBJ) $(EX_OBJ) $(GDT_OBJ) *.out $(OBJ_DIR)	@rm -rf $(ISO_DIR) $(ISO)
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
