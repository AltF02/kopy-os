.text
.code64

.globl _my_adder
_my_adder:
        addq %rdi, %rsi
        movq %rsi, %rax
        ret

.global _x86_asm_set_cs
.p2align 4
_x86_asm_set_cs:
    pushq %rdi
    leaq 1f(%rip), %rax
    pushq %rax
    lretq
1:
    retq

.global _x86_asm_ltr
.p2align 4
_x86_asm_ltr:
    mov %edi, %edx
    ltr %dx
    retq
