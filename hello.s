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
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	subq	$96, %rsp
	.seh_stackalloc 96
	leaq	96(%rsp), %rbp
	.seh_setframe %rbp, 96
	.seh_endprologue
	subq	$32, %rsp
	callq	__main
	callq	newscore
	addq	$32, %rsp
	movl	%eax, %esi
	subq	$32, %rsp
	callq	newbar
	addq	$32, %rsp
	movl	%eax, %edi
	subq	$32, %rsp
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
	movl	%esi, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	addq	$32, %rsp
	movl	%eax, %edi
	subq	$32, %rsp
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
	movl	$12, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$11, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%esi, %ecx
	movl	%edi, %edx
	callq	score_push
	callq	newbar
	addq	$32, %rsp
	movl	%eax, %edi
	subq	$32, %rsp
	movl	$9, %ecx
	movl	$1, %edx
	movl	$2, %r8d
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
	movl	$11, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$9, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$8, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$9, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$4, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	$7, %ecx
	movl	$1, %edx
	movl	$2, %r8d
	callq	newnote
	movl	%edi, %ecx
	movl	%eax, %edx
	callq	bar_push
	movl	%esi, %ecx
	movl	%edi, %edx
	callq	score_push
	addq	$32, %rsp
	movl	%esi, -8(%rbp)
	subq	$32, %rsp
	callq	newscore
	addq	$32, %rsp
	movl	%eax, %edi
	subq	$32, %rsp
	movl	%eax, %ecx
	movl	%esi, %edx
	callq	score_copy
	addq	$32, %rsp
	movl	%edi, -12(%rbp)
	movabsq	$197568495665, %rax             # imm = 0x2E00000031
	movq	%rax, -92(%rbp)
	movabsq	$416611827831, %rax             # imm = 0x6100000077
	movq	%rax, -84(%rbp)
	movq	$118, -76(%rbp)
	movq	$0, -68(%rbp)
	movq	$0, -60(%rbp)
	movabsq	$197568495666, %rcx             # imm = 0x2E00000032
	movq	%rcx, -52(%rbp)
	movq	%rax, -44(%rbp)
	movq	$118, -36(%rbp)
	movq	$0, -28(%rbp)
	movq	$0, -20(%rbp)
	movl	$2, -4(%rbp)
	cmpl	$6, -4(%rbp)
	jg	.LBB1_3
	.p2align	4, 0x90
.LBB1_2:                                # %"$55"
                                        # =>This Inner Loop Header: Depth=1
	subq	$32, %rsp
	callq	newscore
	addq	$32, %rsp
	movl	%eax, %esi
	movl	-12(%rbp), %edx
	subq	$32, %rsp
	movl	%eax, %ecx
	callq	score_copy
	addq	$32, %rsp
	movl	$16, %eax
	callq	___chkstk_ms
	subq	%rax, %rsp
	movq	%rsp, %rax
	movl	%esi, (%rax)
	movl	-4(%rbp), %edi
	subq	$32, %rsp
	movl	%esi, %ecx
	movl	%edi, %edx
	callq	score_inc_pitch
	addq	$32, %rsp
	imull	$500, %edi, %eax                # imm = 0x1F4
	movl	$12000, %edx                    # imm = 0x2EE0
	subl	%eax, %edx
	subq	$32, %rsp
	movl	%esi, %ecx
	callq	score_set_duration
	addq	$32, %rsp
	movl	-8(%rbp), %ecx
	subq	$32, %rsp
	movl	%esi, %edx
	callq	score_append
	addq	$32, %rsp
	addl	$2, %edi
	movl	%edi, -4(%rbp)
	subq	$32, %rsp
	movl	%edi, %ecx
	callq	putint
	addq	$32, %rsp
	cmpl	$6, -4(%rbp)
	jle	.LBB1_2
.LBB1_3:                                # %"$56"
	movl	-8(%rbp), %ecx
	subq	$48, %rsp
	movl	$2, 40(%rsp)
	movl	$16, 32(%rsp)
	leaq	-92(%rbp), %rdx
	leaq	-52(%rbp), %r8
	movl	$44100, %r9d                    # imm = 0xAC44
	callq	score_sing
	addq	$48, %rsp
	xorl	%eax, %eax
	movq	%rbp, %rsp
	popq	%rdi
	popq	%rsi
	popq	%rbp
	retq
	.seh_endproc
                                        # -- End function
