  .data
  .globl GLBVAL_0
GLBVAL_0:
  .word 0

  .text
  .globl __pseudo__please_dont_give_same_name
__pseudo__please_dont_give_same_name:
__pseudo__please_dont_give_same_name_PLSDONT_0:
  ret









  .text
  .globl init1d
init1d:
  addi sp, sp, -64
init1d_PLSDONT_0:
  addi t0, sp, 0
  sw t0, 4(sp)
  mv x5, x10
  lw x6, 4(sp)
  sw t0, 0(t1)
  addi t0, sp, 8
  sw t0, 12(sp)
  li x5, 0
  lw x6, 12(sp)
  sw t0, 0(t1)
  j init1d_PLSDONT_1
init1d_PLSDONT_1:
  lw x5, 12(sp)
  lw t0, 0(t0)
  sw t0, 16(sp)
  lw x5, 4(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  lw x5, 16(sp)
  lw x6, 20(sp)
  slt t0, t0, t1
  sw t0, 24(sp)
  lw x5, 24(sp)
  beqz t0, init1d_PLSDONT_5
  j init1d_PLSDONT_2
init1d_PLSDONT_5:
  j init1d_PLSDONT_3
init1d_PLSDONT_2:
  lw x5, 12(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  mv x5, x11
  lw x6, 28(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 32(sp)
  la x5, GLBVAL_0
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw x5, 36(sp)
  lw x6, 32(sp)
  sw t0, 0(t1)
  la x5, GLBVAL_0
  lw t0, 0(t0)
  sw t0, 40(sp)
  lw x5, 40(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 44(sp)
  lw x5, 44(sp)
  la x6, GLBVAL_0
  sw t0, 0(t1)
  lw x5, 12(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw x5, 48(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 52(sp)
  lw x5, 52(sp)
  lw x6, 12(sp)
  sw t0, 0(t1)
  j init1d_PLSDONT_1
init1d_PLSDONT_3:
  addi sp, sp, 64
  ret

  .text
  .globl init2d
init2d:
  addi sp, sp, -64
init2d_PLSDONT_0:
  addi t0, sp, 4
  sw t0, 8(sp)
  mv x5, x10
  lw x6, 8(sp)
  sw t0, 0(t1)
  addi t0, sp, 12
  sw t0, 16(sp)
  li x5, 0
  lw x6, 16(sp)
  sw t0, 0(t1)
  j init2d_PLSDONT_1
init2d_PLSDONT_1:
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw x5, 20(sp)
  lw x6, 24(sp)
  slt t0, t0, t1
  sw t0, 28(sp)
  lw x5, 28(sp)
  beqz t0, init2d_PLSDONT_5
  j init2d_PLSDONT_2
init2d_PLSDONT_5:
  j init2d_PLSDONT_3
init2d_PLSDONT_2:
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  mv x5, x11
  lw x6, 32(sp)
  li t2, 40
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 36(sp)
  lw x5, 36(sp)
  li x6, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 40(sp)
  li x10, 10
  lw x11, 40(sp)
  mv t0, ra
  sw t0, 0(sp)
  call init1d
  lw t0, 0(sp)
  mv ra, t0
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw x5, 44(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 48(sp)
  lw x5, 48(sp)
  lw x6, 16(sp)
  sw t0, 0(t1)
  j init2d_PLSDONT_1
init2d_PLSDONT_3:
  addi sp, sp, 64
  ret

  .text
  .globl init3d
init3d:
  addi sp, sp, -64
init3d_PLSDONT_0:
  addi t0, sp, 4
  sw t0, 8(sp)
  mv x5, x10
  lw x6, 8(sp)
  sw t0, 0(t1)
  addi t0, sp, 12
  sw t0, 16(sp)
  li x5, 0
  lw x6, 16(sp)
  sw t0, 0(t1)
  j init3d_PLSDONT_1
init3d_PLSDONT_1:
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  lw x5, 20(sp)
  lw x6, 24(sp)
  slt t0, t0, t1
  sw t0, 28(sp)
  lw x5, 28(sp)
  beqz t0, init3d_PLSDONT_5
  j init3d_PLSDONT_2
init3d_PLSDONT_5:
  j init3d_PLSDONT_3
init3d_PLSDONT_2:
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  mv x5, x11
  lw x6, 32(sp)
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 36(sp)
  lw x5, 36(sp)
  li x6, 0
  li t2, 40
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 40(sp)
  li x10, 10
  lw x11, 40(sp)
  mv t0, ra
  sw t0, 0(sp)
  call init2d
  lw t0, 0(sp)
  mv ra, t0
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw x5, 44(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 48(sp)
  lw x5, 48(sp)
  lw x6, 16(sp)
  sw t0, 0(t1)
  j init3d_PLSDONT_1
init3d_PLSDONT_3:
  addi sp, sp, 64
  ret

  .text
  .globl main
main:
  li t0, -4032
  add sp, sp, t0
main_PLSDONT_0:
  li t0, 4
  add t0, sp, t0
  li t1, 4004
  add t1, sp, t1
  sw t0, 0(t1)
  li x5, 4004
  add x5, sp, x5
  lw x5, 0(x5)
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  li t1, 4008
  add t1, sp, t1
  sw t0, 0(t1)
  li x10, 10
  li x11, 4008
  add x11, sp, x11
  lw x11, 0(x11)
  mv t0, ra
  sw t0, 0(sp)
  call init3d
  lw t0, 0(sp)
  mv ra, t0
  la x5, GLBVAL_0
  lw t0, 0(t0)
  li t1, 4012
  add t1, sp, t1
  sw t0, 0(t1)
  li x10, 4012
  add x10, sp, x10
  lw x10, 0(x10)
  mv t0, ra
  sw t0, 0(sp)
  call putint
  lw t0, 0(sp)
  mv ra, t0
  li x10, 0
  li t0, 4032
  add sp, sp, t0
  ret
main_PLSDONT_1:
  li x10, 0
  li t0, 4032
  add sp, sp, t0
  ret

