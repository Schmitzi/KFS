# Keyboard Input Guide - Interrupts and Hardware I/O

## Overview

Keyboard input is the most complex feature because it requires:
1. **Interrupts** - Hardware signals that stop CPU execution
2. **IDT** - Interrupt Descriptor Table (tells CPU where handlers are)
3. **PIC** - Programmable Interrupt Controller (manages interrupts)
4. **Scan codes** - Raw keyboard codes that need translation
5. **ASCII conversion** - Convert scan codes to printable characters

This guide explains everything from scratch.

---

## What Are Interrupts?

### The Problem

Your kernel runs in a loop:
```rust
loop {
    // Do stuff
}
```

How does the keyboard tell the kernel "hey, a key was pressed"?

**Bad solution:** Polling
```rust
loop {
    if keyboard_has_data() {
        read_keyboard();
    }
}
```
- Wastes CPU cycles
- Checks constantly even when nothing happens
- Inefficient

**Good solution:** Interrupts
```rust
loop {
    hlt(); // Sleep until interrupt
}

// Somewhere else...
fn keyboard_interrupt_handler() {
    read_keyboard(); // Only runs when key pressed!
}
```
- CPU sleeps until key pressed
- Hardware wakes CPU automatically
- Efficient!

### How Interrupts Work

1. **Key is pressed** → Keyboard hardware sends electrical signal
2. **PIC receives signal** → Routes it to CPU as IRQ1
3. **CPU stops** → Saves current state
4. **CPU looks up handler** → Checks IDT for IRQ1 handler
5. **Handler runs** → Your `keyboard_interrupt_handler()` executes
6. **CPU resumes** → Returns to what it was doing

---

## Interrupt Descriptor Table (IDT)

### What Is It?

The IDT is a table that tells the CPU:
- "When interrupt X happens, call function Y"

**Structure:**
```
IDT Entry 0  → Divide by zero handler
IDT Entry 1  → Debug handler
...
IDT Entry 32 → Timer interrupt (IRQ0)
IDT Entry 33 → Keyboard interrupt (IRQ1)  ← We care about this!
...
IDT Entry 255 → Last entry
```

### IDT Entry Structure

```rust
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,    // Lower 16 bits of handler address
    selector: u16,      // Code segment selector (0x08 = kernel code)
    zero: u8,           // Always 0
    type_attr: u8,      // 0x8E = present, DPL=0, 32-bit interrupt gate
    offset_high: u16,   // Upper 16 bits of handler address
}
```

**Breaking it down:**

**offset_low + offset_high:**
- Combined form the 32-bit address of your handler function
- Example: Handler at 0x00102030
  - offset_low = 0x2030
  - offset_high = 0x0010

**selector:**
- Code segment where handler lives
- 0x08 = kernel code segment (set by bootloader/GRUB)

**type_attr:**
- Bit 7: Present (1 = entry is valid)
- Bits 6-5: DPL (0 = kernel privilege)
- Bits 4-0: Type (0xE = 32-bit interrupt gate)
- Combined: 0x8E

### Setting an IDT Entry

```rust
fn set_handler(&mut self, handler: unsafe extern "C" fn()) {
    let handler_addr = handler as usize;
    self.offset_low = (handler_addr & 0xFFFF) as u16;
    self.offset_high = ((handler_addr >> 16) & 0xFFFF) as u16;
    self.selector = 0x08;
    self.zero = 0;
    self.type_attr = 0x8E;
}
```

**Example:**
```
Handler function at: 0x00102ABC

offset_low  = 0x2ABC (lower 16 bits)
offset_high = 0x0010 (upper 16 bits)
```

### Loading the IDT

```rust
#[repr(C, packed)]
struct IdtPointer {
    limit: u16,  // Size of IDT - 1
    base: u32,   // Address of IDT
}

// Load with lidt instruction
asm!(
    "lidt [{}]",
    in(reg) &idt_ptr,
);
```

**What this does:**
- Tells CPU "the IDT is at this address"
- CPU will use it for all future interrupts

---

## Programmable Interrupt Controller (PIC)

### What Is It?

The PIC (8259 chip) manages hardware interrupts:
- Receives signals from keyboard, mouse, timer, etc.
- Prioritizes them
- Sends them to CPU one at a time

**Two PICs:**
- **PIC1** (Master) - Handles IRQ0-7
- **PIC2** (Slave) - Handles IRQ8-15, connected to PIC1's IRQ2

### IRQ Mapping

**Before remapping:**
```
IRQ0 (Timer)    → Interrupt 8   ← CONFLICT! (CPU exception)
IRQ1 (Keyboard) → Interrupt 9   ← CONFLICT!
...
```

**After remapping:**
```
IRQ0 (Timer)    → Interrupt 32
IRQ1 (Keyboard) → Interrupt 33  ← No conflict!
IRQ2 (Cascade)  → Interrupt 34
...
```

### Why Remap?

**The problem:**
- CPU reserves interrupts 0-31 for exceptions (divide by zero, page fault, etc.)
- Old PIC mapping uses interrupts 8-15
- **Conflict!** Keyboard interrupt overwrites "double fault" exception

**The solution:**
- Remap PIC to use interrupts 32-47
- No more conflicts!

### PIC Remapping Code

```rust
pub fn remap() {
    unsafe {
        // Start initialization
        outb(PIC1_COMMAND, ICW1_INIT);
        outb(PIC2_COMMAND, ICW1_INIT);

        // Set vector offsets
        outb(PIC1_DATA, 32);  // IRQ0-7 → interrupts 32-39
        outb(PIC2_DATA, 40);  // IRQ8-15 → interrupts 40-47

        // Tell PICs about each other
        outb(PIC1_DATA, 4);   // PIC2 at IRQ2
        outb(PIC2_DATA, 2);   // Cascade identity

        // Set 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        outb(PIC2_DATA, ICW4_8086);

        // Enable keyboard (IRQ1), disable others
        outb(PIC1_DATA, 0xFD); // 0xFD = 11111101 (IRQ1 enabled)
        outb(PIC2_DATA, 0xFF); // All disabled
    }
}
```

### End of Interrupt (EOI)

After handling an interrupt, you MUST tell the PIC "I'm done":

```rust
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);  // Tell slave PIC
        }
        outb(PIC1_COMMAND, PIC_EOI);      // Tell master PIC
    }
}
```

**Why?**
- PIC won't send more interrupts until you acknowledge
- Keyboard won't work if you forget this!

---

## Keyboard Hardware

### PS/2 Keyboard

**Data Port:** 0x60
- Read scan codes from here

**Status Port:** 0x64
- Check if data is available (we don't use this, just read directly)

### Scan Codes

When you press a key, the keyboard sends a **scan code**:

**Key press:** Scan code < 128
```
Press 'A' → Scan code 0x1E (30 in decimal)
```

**Key release:** Scan code + 128
```
Release 'A' → Scan code 0x9E (158 in decimal)
```

### Scan Code to ASCII

```rust
static SCANCODE_TO_ASCII: [u8; 128] = [
    0,    // 0x00
    27,   // 0x01 - Escape
    b'1', // 0x02
    b'2', // 0x03
    // ...
    b' ', // 0x39 - Space
    // ...
];
```

**Example:**
```
Scan code 0x1E → SCANCODE_TO_ASCII[0x1E] = b'a'
```

---

## Keyboard Interrupt Handler

### The Handler Function

```rust
#[no_mangle]
pub extern "C" fn keyboard_interrupt_handler() {
    unsafe {
        // 1. Read scan code from keyboard
        let scancode = inb(KEYBOARD_DATA_PORT);

        // 2. Check if it's a key press (not release)
        if scancode < 128 {
            // 3. Convert to ASCII
            let ascii = SCANCODE_TO_ASCII[scancode as usize];
            
            // 4. Print character
            if ascii != 0 {
                print!("{}", ascii as char);
            }
        }

        // 5. IMPORTANT: Tell PIC we're done
        pic::send_eoi(1);
    }
}
```

**Step by step:**

1. **Read scan code** - Get byte from port 0x60
2. **Filter releases** - Ignore key releases (scancode >= 128)
3. **Convert to ASCII** - Look up in table
4. **Print** - Display character
5. **Send EOI** - Tell PIC we're done (CRITICAL!)

### Why `#[no_mangle]`?

```rust
#[no_mangle]
pub extern "C" fn keyboard_interrupt_handler()
```

- `#[no_mangle]` - Keep function name as-is (don't scramble it)
- `extern "C"` - Use C calling convention (required for interrupts)
- Without these, the linker can't find your handler!

---

## Putting It All Together

### Initialization Sequence

```rust
fn kernel_main() -> ! {
    // 1. Initialize IDT
    idt::init();           // Set up interrupt table
    
    // 2. Configure PIC
    pic::remap();          // Remap IRQs to 32-47
    
    // 3. Enable interrupts
    idt::enable_interrupts(); // Run 'sti' instruction
    
    // 4. Wait for interrupts
    loop {
        hlt();  // Sleep until interrupt
    }
}
```

### What Happens When You Press a Key

```
[1] You press 'A'
    ↓
[2] Keyboard sends scan code 0x1E to port 0x60
    ↓
[3] Keyboard sends electrical signal to PIC
    ↓
[4] PIC sends IRQ1 to CPU
    ↓
[5] CPU checks IDT entry 33 (IRQ1 after remapping)
    ↓
[6] CPU finds keyboard_interrupt_handler address
    ↓
[7] CPU saves state and jumps to handler
    ↓
[8] Handler:
    - Reads 0x1E from port 0x60
    - Looks up: SCANCODE_TO_ASCII[0x1E] = b'a'
    - Prints 'a' to screen
    - Sends EOI to PIC
    ↓
[9] CPU restores state and returns to loop
    ↓
[10] Loop continues (hlt until next key)
```

---

## Port I/O

### What Is Port I/O?

**Memory-mapped I/O:** (VGA buffer at 0xB8000)
```rust
*VGA_BUFFER.add(offset) = byte;  // Normal memory write
```

**Port I/O:** (Keyboard at port 0x60)
```rust
outb(0x60, value);  // Special CPU instruction
value = inb(0x60);  // Special CPU instruction
```

**Why different?**
- Hardware has limited address space
- Port I/O uses separate address space (0-65535)
- Requires special CPU instructions (`in`, `out`)

### Inline Assembly for Port I/O

**Output byte:**
```rust
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}
```

**Input byte:**
```rust
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}
```

**Assembly breakdown:**
```
out dx, al  ← Send byte in 'al' to port in 'dx'
in al, dx   ← Read byte from port in 'dx' into 'al'
```

---

## Common Issues

### Issue: Keyboard doesn't work
**Cause:** Forgot to send EOI
**Fix:** Always call `pic::send_eoi(1)` at end of handler

### Issue: Only one key works
**Cause:** Not sending EOI
**Fix:** Same as above

### Issue: Double characters
**Cause:** Not filtering key releases
**Fix:** Check `if scancode < 128`

### Issue: Kernel hangs
**Cause:** Interrupts not enabled
**Fix:** Call `idt::enable_interrupts()`

### Issue: Wrong characters
**Cause:** Scan code table is wrong
**Fix:** Verify SCANCODE_TO_ASCII table

---

## Testing

### Test 1: Basic Typing
```
Type: hello
Expected: "hello" appears on screen
```

### Test 2: Numbers
```
Type: 12345
Expected: "12345" appears on screen
```

### Test 3: Special Keys
```
Type: Enter key
Expected: Cursor moves to new line
```

### Test 4: Backspace
```
Type: abc, then backspace
Expected: Shows backspace character (or implement proper backspace)
```

---

## Improvements (Future)

### Current Limitations
- No Shift support (all lowercase)
- No Ctrl/Alt support
- Backspace doesn't actually delete
- No key repeat
- No Caps Lock indicator

### Future Features
1. **Shift/Caps Lock** - Track modifier keys
2. **Proper Backspace** - Delete previous character
3. **Key Buffer** - Store keys for programs to read
4. **Special Keys** - F1-F12, arrows, etc.
5. **Keyboard Layouts** - Support non-US keyboards

---

## Learning Resources

- **OSDev Interrupts**: https://wiki.osdev.org/Interrupts
- **OSDev IDT**: https://wiki.osdev.org/Interrupt_Descriptor_Table
- **OSDev PIC**: https://wiki.osdev.org/PIC
- **OSDev PS/2 Keyboard**: https://wiki.osdev.org/PS/2_Keyboard
- **Scan Code Reference**: https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1

---

Good luck! This is the hardest feature, but once it works you have an interactive kernel! 🚀