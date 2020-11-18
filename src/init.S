.section .text.init
.global _start
_start:
    msr cpsr_c, #0x13 
    mov r0, #0
    mov r1, #0
    mov r2, #0
    mov r3, #0
    mov r4, #0
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    mov r9, #0
    mov r10, #0
    mov r11, #0
    mov r12, #0
    mov r13, #0
    mov r14, #0

ldr sp, =_stack

b entry
