.text
.globl _my_adder
.globl _my_test
_my_adder:
        addq %rdi, %rsi
        movq %rsi, %rax
        ret

;_edxa:
        ;mov edx,len
        ;mov ecx,msg
        ;mov ebx,1
        ;mov eax,4
        ;int 0x80

        ;mov eax,1
        ;int 0x80
