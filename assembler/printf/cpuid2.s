.section .data
output:
	.asciz "The processor Vendor ID is '%s' \n"
.section .bss
	.lcomm buffer, 12
.section .text
.global _start
_start:
	mov $0, %eax
	cpuid
	mov $buffer, %edi
	mov %ebx, (%edi)
	mov %edx, 4(%edi)
	mov %ecx, 8(%edi)
	push $buffer
	push $output
	call printf
	add $8, %esp
	push $0
	call exit
