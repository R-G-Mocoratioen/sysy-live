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
	pushq	%r15
	.seh_pushreg %r15
	pushq	%r14
	.seh_pushreg %r14
	pushq	%r13
	.seh_pushreg %r13
	pushq	%r12
	.seh_pushreg %r12
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	pushq	%rbx
	.seh_pushreg %rbx
	subq	$232, %rsp
	.seh_stackalloc 232
	leaq	128(%rsp), %rbp
	.seh_setframe %rbp, 128
	.seh_endprologue
	callq	__main
	callq	newscore
	movl	%eax, %r14d
	callq	newbar
	movl	%eax, %esi
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
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
	movl	$19, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r14d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$21, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$21, %ecx
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
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r14d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$17, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$16, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r14d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$14, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$14, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$12, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r14d, %ecx
	movl	%esi, %edx
	callq	score_push
	movl	%r14d, 80(%rbp)
	callq	newscore
	movl	%eax, %r15d
	callq	newbar
	movl	%eax, %esi
	xorl	%ecx, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r15d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$5, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r15d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$-5, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$-1, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r15d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$7, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	xorl	%ecx, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r15d, %ecx
	movl	%esi, %edx
	callq	score_push
	movl	%r15d, 84(%rbp)
	callq	newscore
	movl	%eax, %r12d
	callq	newbar
	movl	%eax, %esi
	movl	$24, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$24, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$28, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$31, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$24, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$28, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$31, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r12d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$33, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$29, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$36, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$33, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$31, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$28, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$24, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$28, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r12d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$29, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$26, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$29, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$23, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$23, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$28, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r12d, %ecx
	movl	%esi, %edx
	callq	score_push
	callq	newbar
	movl	%eax, %esi
	movl	$26, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$19, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$23, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$26, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$24, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$2, %ecx
	movl	$1, %edx
	movl	$1, %r8d
	callq	newnote_rest
	movl	%esi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%r12d, %ecx
	movl	%esi, %edx
	callq	score_push
	movl	%r12d, 88(%rbp)
	movabsq	$197568495665, %rax             # imm = 0x2E00000031
	movq	%rax, -80(%rbp)
	movabsq	$416611827831, %rax             # imm = 0x6100000077
	movq	%rax, -72(%rbp)
	movq	$118, -64(%rbp)
	movq	$0, -56(%rbp)
	movq	$0, -48(%rbp)
	movabsq	$197568495666, %rcx             # imm = 0x2E00000032
	movq	%rcx, -40(%rbp)
	movq	%rax, -32(%rbp)
	movq	$118, -24(%rbp)
	movq	$0, -16(%rbp)
	movq	$0, -8(%rbp)
	movabsq	$197568495667, %rcx             # imm = 0x2E00000033
	movq	%rcx, (%rbp)
	movq	%rax, 8(%rbp)
	movq	$118, 16(%rbp)
	movq	$0, 24(%rbp)
	movq	$0, 32(%rbp)
	movabsq	$197568495668, %rcx             # imm = 0x2E00000034
	movq	%rcx, 40(%rbp)
	movq	%rax, 48(%rbp)
	movq	$118, 56(%rbp)
	movq	$0, 64(%rbp)
	movq	$0, 72(%rbp)
	movl	%r14d, %ecx
	movl	$-12, %edx
	callq	score_inc_pitch
	movl	%r15d, %ecx
	movl	$-12, %edx
	callq	score_inc_pitch
	movl	%r12d, %ecx
	movl	$-12, %edx
	callq	score_inc_pitch
	movl	$2, 40(%rsp)
	movl	$16, 32(%rsp)
	leaq	-80(%rbp), %rsi
	leaq	-40(%rbp), %r13
	movl	%r14d, %ecx
	movq	%rsi, %rdx
	movq	%r13, %r8
	movl	$44100, %r9d                    # imm = 0xAC44
	callq	score_sing
	movl	$2, 40(%rsp)
	movl	$16, 32(%rsp)
	movq	%rbp, %r14
	movl	%r15d, %ecx
	movq	%rsi, %rdx
	movq	%r14, %r8
	movl	$44100, %r9d                    # imm = 0xAC44
	callq	score_sing
	movl	$2, 40(%rsp)
	movl	$16, 32(%rsp)
	leaq	40(%rbp), %r15
	movl	%r12d, %ecx
	movq	%rsi, %rdx
	movq	%r15, %r8
	movl	$44100, %r9d                    # imm = 0xAC44
	callq	score_sing
	callq	newtrack
	movl	%eax, %esi
	movl	%eax, %ecx
	movq	%r13, %rdx
	callq	track_load
	movl	%esi, 92(%rbp)
	callq	newtrack
	movl	%eax, %edi
	movl	%eax, %ecx
	movq	%r14, %rdx
	callq	track_load
	movl	%edi, 96(%rbp)
	callq	newtrack
	movl	%eax, %ebx
	movl	%eax, %ecx
	movq	%r15, %rdx
	callq	track_load
	movl	%ebx, 100(%rbp)
	movl	%esi, %ecx
	movl	%edi, %edx
	callq	track_stack
	movl	%esi, %ecx
	movl	%ebx, %edx
	callq	track_stack
	xorl	%eax, %eax
	addq	$232, %rsp
	popq	%rbx
	popq	%rdi
	popq	%rsi
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
	.seh_endproc
                                        # -- End function
