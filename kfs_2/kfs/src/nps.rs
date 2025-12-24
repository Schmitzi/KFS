use crate::gdt;
use crate::vga;

const MAX_COMMAND_LEN: usize = 64;

pub struct NPShell {
    buffer: [u8; MAX_COMMAND_LEN],
    pos: usize,
}

impl NPShell {
    pub const fn new() -> NPShell {
        NPShell {
            buffer: [0; MAX_COMMAND_LEN],
            pos: 0,
        }
    }

    pub fn show_prompt(&self) {
        crate::vga::writer().set_color(vga::Color::LightGreen, vga::Color::Black);
        crate::print!("> ");
        crate::vga::writer().set_color(vga::Color::White, vga::Color::Black);
    }

    pub fn handle_char(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                // Execute command
                self.execute();
                self.clear();
                println!();
                self.show_prompt();
            }
            0x08 => {
                // Backspace
                if self.pos > 0 {
                    self.pos -= 1;
                    crate::print!("\x08 \x08"); // Erase character
                }
            }
            0x20..=0x7E => {
                // Printable character
                if self.pos < MAX_COMMAND_LEN - 1 {
                    self.buffer[self.pos] = ch;
                    self.pos += 1;
                    crate::print!("{}", ch as char);
                }
            }
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.buffer = [0; MAX_COMMAND_LEN];
        self.pos = 0;
    }

    fn execute(&self) {
        if self.pos == 0 {
            return;
        }

        // Get command as string
        let cmd = core::str::from_utf8(&self.buffer[..self.pos])
            .unwrap_or("");

        println!(); // Newline after command

        match cmd.trim() {
            "help" => self.cmd_help(),
            "stack" => self.cmd_stack(),
            "gdt" => self.cmd_gdt(),
            "clear" => self.cmd_clear(),
            "about" => self.cmd_about(),
            "reboot" => self.cmd_reboot(),
            "halt" => self.cmd_halt(),
            "" => {},
            _ => println!("Unknown command: '{}'. Type 'help' for commands.", cmd),
        }
    }

    fn cmd_help(&self) {
        println!("Available commands:");
        println!("  help   - Show this help message");
        println!("  stack  - Print kernel stack information");
        println!("  gdt    - Print GDT information");
        println!("  clear  - Clear the screen");
        println!("  about  - About this kernel");
        println!("  halt   - Halt the CPU");
        println!("  reboot - Reboot the system");
    }

    fn cmd_stack(&self) {
        gdt::print_stack();
    }

    fn cmd_gdt(&self) {
        gdt::print_gdt();
    }

    fn cmd_clear(&self) {
        crate::vga::writer().clear_screen();
        println!("NPS - Not a POSIX Shell - Type 'help' for commands");
    }

    fn cmd_about(&self) {
        println!("KFS_2 - Kernel From Scratch");
        println!("A bare-metal i386 kernel written in Rust");
        println!("Features:");
        println!("  - Custom GDT implementation");
        println!("  - Interrupt handling (IDT + PIC)");
        println!("  - Keyboard input");
        println!("  - VGA text mode with colors");
        println!("  - This shell!");
    }

    fn cmd_halt(&self) {
        println!("Halting CPU...");
        unsafe {
            core::arch::asm!("cli; hlt", options(noreturn));
        }
    }

    fn cmd_reboot(&self) {
        println!("Rebooting...");
        unsafe {
            // Pulse the CPU reset line via keyboard controller
            let mut port: u8;
            loop {
                core::arch::asm!("in al, dx", out("al") port, in("dx") 0x64u16);
                if (port & 0x02) == 0 {
                    break;
                }
            }
            core::arch::asm!("out dx, al", in("dx") 0x64u16, in("al") 0xFEu8);
            
            // If that didn't work, triple fault
            core::arch::asm!("cli; hlt", options(noreturn));
        }
    }
}

// Global shell instance
static mut NPSHELL: NPShell = NPShell::new();

// Initialize shell
pub fn init() {
    println!("NPS - Not a POSIX Shell - Type 'help' for commands");
    unsafe {
        NPSHELL.show_prompt();
    }
}

// Handle keyboard input for shell
pub fn handle_input(ch: u8) {
    unsafe {
        NPSHELL.handle_char(ch);
    }
}