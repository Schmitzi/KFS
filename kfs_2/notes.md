# kfs_2

[Introduction](#introduction)<br>
[What we want to achieve](#what-we-want-to-achieve)<br>
[The Fun Stuff](#the-fun-stuff)<br>

## Introduction

On todays episode on ~~~"Why the hell do I do this to myself"~~~ "I'm having so much fun", we are going to learn how to code a stack and integrate it with GDT

Now your'e probably asking yourself "But Michael, I have a stack", "What's GDT?" and maybe "It's 5am, can I go to sleep yet?".

Firstly we did include a stack in our bootloader but this is a different stack

Secondly, GDT stands for "Global Descriptor Table". It is a data structure used to define the different memory areas: the base address, the size and access privileges like execute and write.

And lastly, no.....of course we don't sleep, this is way too much fun

These memory areas are called ```segments```

In a GDT, you can find:

- Kernel code, used to store the executable binary code
- Kernel data
- Kernel stack, used to store the call stack during kernel execution
- User code, used to store the executable binary code for user programs
- User program data
- User stack, used  to store the call stack during execution in userland

Next, the stack. As stack is defined as:

*an abstract data type that serves as a collection of elements, with two principal operations: push, which adds an element to the collection, and pop, which removes the most recently added element that was not yet removed. The order in which elements come off a stack gives rise to its alternative name, LIFO (for last in, first out).*

## What we want to achieve

- ***```create```, ```fill``` and ```link```*** - Build a Global Descriptor Table into the Kernel
    The GDT must contain:
    - Kernel Code
    - Kernel Data
    - Kernel stack
    - User code
    - User data
    - User stack

    GDT must be declared to the BIOS and set to address ```0x00000800```
    Then we need to code a tool to print the kernel stack in a human-friendly way. We have already built printk() so thats great.

- ***Understand how memory works*** - Understand how the stack and RAM work, how to use it, how to fill it and how to link it with the BIOS

***REMEMBER TO KEEP IT UNDER 10MB***

## The Fun Stuff

Because we have keyboards working, lets build a Shell (prototype name: NPS - Not a POSIX Shell)

It its suggested to add the print-kernel-stack function, ```reboot```, ```halt``` and the like.

# Michael's Famous InfoDump






