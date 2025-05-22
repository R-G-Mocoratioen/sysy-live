  .text
  .globl __pseudo__please_dont_give_same_name
__pseudo__please_dont_give_same_name:
__pseudo__please_dont_give_same_name_PLSDONT_0:
  ret









  .text
  .globl main
main:
  addi sp, sp, -80
main_PLSDONT_0:
  addi t0, sp, 4
  sw t0, 8(sp)
  li x5, 100
  lw x6, 8(sp)
  sw t0, 0(t1)
  addi t0, sp, 12
  sw t0, 16(sp)
  li x5, 100
  lw x6, 16(sp)
  sw t0, 0(t1)
  addi t0, sp, 20
  sw t0, 24(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw x5, 28(sp)
  lw x6, 32(sp)
  sgt t0, t1, t0
  xori t0, t0, 1
  sw t0, 36(sp)
  addi t0, sp, 40
  sw t0, 44(sp)
  li x5, 1
  lw x6, 44(sp)
  sw t0, 0(t1)
  lw x5, 36(sp)
  beqz t0, main_PLSDONT_4
  j main_PLSDONT_2
main_PLSDONT_4:
  j main_PLSDONT_1
main_PLSDONT_1:
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 52(sp)
  lw x5, 48(sp)
  lw x6, 52(sp)
  slt t0, t1, t0
  xori t0, t0, 1
  sw t0, 56(sp)
  lw x5, 56(sp)
  li x6, 0
  sub t1, t0, t1
  snez t0, t1
  sw t0, 60(sp)
  lw x5, 60(sp)
  lw x6, 44(sp)
  sw t0, 0(t1)
  j main_PLSDONT_2
main_PLSDONT_2:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 64(sp)
  lw x5, 64(sp)
  lw x6, 24(sp)
  sw t0, 0(t1)
  lw x5, 24(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  lw x10, 68(sp)
  mv t0, ra
  sw t0, 0(sp)
  call putint
  lw t0, 0(sp)
  mv ra, t0
  li x10, 0
  addi sp, sp, 80
  ret
main_PLSDONT_3:
  li x10, 0
  addi sp, sp, 80
  ret

