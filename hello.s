	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"hello.llvm"
	.def	__pseudo__please_dont_give_same_name;
	.scl	2;
	.type	32;
	.endef
	.globl	__pseudo__please_dont_give_same_name # -- Begin function __pseudo__please_dont_give_same_name
	.p2align	4, 0x90
__pseudo__please_dont_give_same_name:   # @__pseudo__please_dont_give_same_name
# %bb.0:                                # %"$0"
	retq
                                        # -- End function
	.def	main;
	.scl	2;
	.type	32;
	.endef
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
main:                                   # @main
.seh_proc main
# %bb.0:                                # %"$1"
	pushq	%rbp
	.seh_pushreg %rbp
	pushq	%r14
	.seh_pushreg %r14
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	pushq	%rbx
	.seh_pushreg %rbx
	subq	$144, %rsp
	.seh_stackalloc 144
	leaq	128(%rsp), %rbp
	.seh_setframe %rbp, 128
	.seh_endprologue
	callq	__main
	movl	$24, %ecx
	movl	$2, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%eax, %r14d
	movl	%eax, 4(%rbp)
	callq	newbar
	movl	%eax, %esi
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%esi, 8(%rbp)
	callq	newscore
	movl	%eax, %ebx
	callq	newbar
	movl	%eax, %edi
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$14, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %edi
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$1, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %edi
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %edi
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$4, %r8d
	callq	newnote_rest
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$21, %ecx
	movl	$1, %edx
	movl	$4, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$17, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %edi
	movl	$21, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$23, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$20, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	%edi, %edx
	callq	score_push
	movl	%ebx, 12(%rbp)
	movabsq	$197568495665, %rax             # imm = 0x2E00000031
	movq	%rax, -76(%rbp)
	movabsq	$416611827831, %rax             # imm = 0x6100000077
	movq	%rax, -68(%rbp)
	movq	$118, -60(%rbp)
	movq	$0, -52(%rbp)
	movq	$0, -44(%rbp)
	movabsq	$197568495666, %rcx             # imm = 0x2E00000032
	movq	%rcx, -36(%rbp)
	movq	%rax, -28(%rbp)
	movq	$118, -20(%rbp)
	movq	$0, -12(%rbp)
	movq	$0, -4(%rbp)
	movl	%esi, %ecx
	movl	%r14d, %edx
	callq	bar_push
	movl	%ebx, %ecx
	movl	$150, %edx
	callq	score_setbpm
	movl	%esi, %ecx
	movl	$210, %edx
	callq	bar_setbpm
	movl	%ebx, %ecx
	movl	%esi, %edx
	callq	score_push
	movl	$2, 40(%rsp)
	movl	$16, 32(%rsp)
	leaq	-76(%rbp), %rdx
	leaq	-36(%rbp), %r8
	movl	%ebx, %ecx
	movl	$44100, %r9d                    # imm = 0xAC44
	callq	score_sing
	xorl	%eax, %eax
	addq	$144, %rsp
	popq	%rbx
	popq	%rdi
	popq	%rsi
	popq	%r14
	popq	%rbp
	retq
	.seh_endproc
                                        # -- End function
