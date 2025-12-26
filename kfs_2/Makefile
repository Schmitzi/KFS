# Makefile for KFS_2 kernel
KFS := KFS_2

# Compiler and tools
ASM := nasm
RUSTC := rustup run nightly cargo
LD := ld
GRUB_MKRESCUE := grub-mkrescue
QEMU := qemu-system-i386

# Flags
ASM_FLAGS := -f elf32
RUST_FLAGS := build -Z build-std=core --target i386-unknown-none.json
LD_FLAGS := -m elf_i386 -n -T kfs/linker.ld

# Directories
KFS_DIR := kfs
SRC_DIR := $(KFS_DIR)/src
OBJ_DIR := obj
ISO_DIR := iso
BOOT_DIR := $(ISO_DIR)/boot
GRUB_DIR := $(BOOT_DIR)/grub

# Assembly files
ASM_SRCS := boot.asm kb.asm exc.asm gdt.asm
ASM_OBJS := $(addprefix $(OBJ_DIR)/, $(ASM_SRCS:.asm=.o))

# Rust files
RUST_SRCS := $(wildcard $(SRC_DIR)/*.rs)
RUST_LIB := $(KFS_DIR)/target/i386-unknown-none/debug/libkfs.a

# Other files
LINKER := $(KFS_DIR)/linker.ld
CARGO := $(KFS_DIR)/Cargo.toml
KERNEL := kernel.bin
ISO := kfs.iso

# Default target
all: $(KERNEL)

# Create obj directory
$(OBJ_DIR):
	@mkdir -p $(OBJ_DIR)

# Compile each assembly file
$(OBJ_DIR)/boot.o: $(KFS_DIR)/boot.asm | $(OBJ_DIR)
	@echo "Assembling boot.asm..."
	@$(ASM) $(ASM_FLAGS) $< -o $@

$(OBJ_DIR)/kb.o: $(KFS_DIR)/kb.asm | $(OBJ_DIR)
	@echo "Assembling kb.asm..."
	@$(ASM) $(ASM_FLAGS) $< -o $@

$(OBJ_DIR)/exc.o: $(KFS_DIR)/exc.asm | $(OBJ_DIR)
	@echo "Assembling exc.asm..."
	@$(ASM) $(ASM_FLAGS) $< -o $@

$(OBJ_DIR)/gdt.o: $(KFS_DIR)/gdt.asm | $(OBJ_DIR)
	@echo "Assembling gdt.asm..."
	@$(ASM) $(ASM_FLAGS) $< -o $@

# Build Rust kernel library
$(RUST_LIB): $(RUST_SRCS) $(CARGO)
	@echo "Building Rust kernel..."
	@cd $(KFS_DIR) && $(RUSTC) $(RUST_FLAGS)

# Link everything into final kernel binary
$(KERNEL): $(ASM_OBJS) $(RUST_LIB) $(LINKER)
	@echo "Linking kernel..."
	@$(LD) $(LD_FLAGS) -o $(KERNEL) $(ASM_OBJS) $(RUST_LIB)
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
	@rm -f $(KERNEL) *.out
	@rm -rf $(OBJ_DIR) $(ISO_DIR) $(ISO)
	@cd $(KFS_DIR) && cargo clean 2>/dev/null || true
	@echo "✓ Clean complete"

# Remove everything including Cargo cache
fclean: clean
	@echo "Deep cleaning..."
	@rm -rf $(KFS_DIR)/target/
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
