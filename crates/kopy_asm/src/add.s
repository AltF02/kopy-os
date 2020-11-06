.text
.globl _my_adder
_my_adder:
        addq %rdi, %rsi
        movq %rsi, %rax
        ret