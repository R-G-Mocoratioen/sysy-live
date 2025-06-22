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
  .globl QuickSort
QuickSort:
  addi sp, sp, -352
QuickSort_PLSDONT_0:
  addi t0, sp, 4
  sw t0, 8(sp)
  mv x5, x10
  lw x6, 8(sp)
  sw t0, 0(t1)
  addi t0, sp, 12
  sw t0, 16(sp)
  mv x5, x11
  lw x6, 16(sp)
  sw t0, 0(t1)
  addi t0, sp, 20
  sw t0, 24(sp)
  mv x5, x12
  lw x6, 24(sp)
  sw t0, 0(t1)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw x5, 24(sp)
  lw t0, 0(t0)
  sw t0, 32(sp)
  lw x5, 28(sp)
  lw x6, 32(sp)
  slt t0, t0, t1
  sw t0, 36(sp)
  lw x5, 36(sp)
  beqz t0, QuickSort_PLSDONT_21
  j QuickSort_PLSDONT_1
QuickSort_PLSDONT_21:
  j QuickSort_PLSDONT_2
QuickSort_PLSDONT_1:
  addi t0, sp, 40
  sw t0, 44(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw x5, 48(sp)
  lw x6, 44(sp)
  sw t0, 0(t1)
  addi t0, sp, 52
  sw t0, 56(sp)
  lw x5, 24(sp)
  lw t0, 0(t0)
  sw t0, 60(sp)
  lw x5, 60(sp)
  lw x6, 56(sp)
  sw t0, 0(t1)
  addi t0, sp, 64
  sw t0, 68(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 72(sp)
  lw x5, 72(sp)
  lw x6, 48(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 76(sp)
  lw x5, 76(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw x5, 80(sp)
  lw x6, 68(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_3
QuickSort_PLSDONT_2:
  li x10, 0
  addi sp, sp, 352
  ret
QuickSort_PLSDONT_3:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 84(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 88(sp)
  lw x5, 84(sp)
  lw x6, 88(sp)
  slt t0, t0, t1
  sw t0, 92(sp)
  lw x5, 92(sp)
  beqz t0, QuickSort_PLSDONT_22
  j QuickSort_PLSDONT_4
QuickSort_PLSDONT_22:
  j QuickSort_PLSDONT_5
QuickSort_PLSDONT_4:
  j QuickSort_PLSDONT_6
QuickSort_PLSDONT_5:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 96(sp)
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 100(sp)
  lw x5, 96(sp)
  lw x6, 100(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 104(sp)
  lw x5, 68(sp)
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw x5, 108(sp)
  lw x6, 104(sp)
  sw t0, 0(t1)
  addi t0, sp, 112
  sw t0, 116(sp)
  lw x5, 100(sp)
  li x6, 1
  sub t0, t0, t1
  sw t0, 120(sp)
  lw x5, 120(sp)
  lw x6, 116(sp)
  sw t0, 0(t1)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 124(sp)
  lw x10, 96(sp)
  lw x11, 124(sp)
  lw x12, 120(sp)
  mv t0, ra
  sw t0, 0(sp)
  call QuickSort
  lw t0, 0(sp)
  mv ra, t0
  mv t0, a0
  sw t0, 128(sp)
  lw x5, 128(sp)
  lw x6, 116(sp)
  sw t0, 0(t1)
  lw x5, 100(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 132(sp)
  lw x5, 132(sp)
  lw x6, 116(sp)
  sw t0, 0(t1)
  lw x5, 24(sp)
  lw t0, 0(t0)
  sw t0, 136(sp)
  lw x10, 96(sp)
  lw x11, 132(sp)
  lw x12, 136(sp)
  mv t0, ra
  sw t0, 0(sp)
  call QuickSort
  lw t0, 0(sp)
  mv ra, t0
  mv t0, a0
  sw t0, 140(sp)
  lw x5, 140(sp)
  lw x6, 116(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_2
QuickSort_PLSDONT_6:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 144(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 148(sp)
  lw x5, 144(sp)
  lw x6, 148(sp)
  slt t0, t0, t1
  sw t0, 152(sp)
  addi t0, sp, 156
  sw t0, 160(sp)
  li x5, 0
  lw x6, 160(sp)
  sw t0, 0(t1)
  lw x5, 152(sp)
  beqz t0, QuickSort_PLSDONT_23
  j QuickSort_PLSDONT_9
QuickSort_PLSDONT_23:
  j QuickSort_PLSDONT_10
QuickSort_PLSDONT_7:
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 164(sp)
  lw x5, 164(sp)
  li x6, 1
  sub t0, t0, t1
  sw t0, 168(sp)
  lw x5, 168(sp)
  lw x6, 56(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_6
QuickSort_PLSDONT_8:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 172(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 176(sp)
  lw x5, 172(sp)
  lw x6, 176(sp)
  slt t0, t0, t1
  sw t0, 180(sp)
  lw x5, 180(sp)
  beqz t0, QuickSort_PLSDONT_24
  j QuickSort_PLSDONT_11
QuickSort_PLSDONT_24:
  j QuickSort_PLSDONT_12
QuickSort_PLSDONT_9:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 184(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 188(sp)
  lw x5, 184(sp)
  lw x6, 188(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 192(sp)
  lw x5, 192(sp)
  lw t0, 0(t0)
  sw t0, 196(sp)
  lw x5, 68(sp)
  lw t0, 0(t0)
  sw t0, 200(sp)
  lw x5, 200(sp)
  li x6, 1
  sub t0, t0, t1
  sw t0, 204(sp)
  lw x5, 196(sp)
  lw x6, 204(sp)
  sgt t0, t0, t1
  sw t0, 208(sp)
  lw x5, 208(sp)
  li x6, 0
  sub t1, t0, t1
  snez t0, t1
  sw t0, 212(sp)
  lw x5, 212(sp)
  lw x6, 160(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_10
QuickSort_PLSDONT_10:
  lw x5, 160(sp)
  lw t0, 0(t0)
  sw t0, 216(sp)
  lw x5, 216(sp)
  beqz t0, QuickSort_PLSDONT_25
  j QuickSort_PLSDONT_7
QuickSort_PLSDONT_25:
  j QuickSort_PLSDONT_8
QuickSort_PLSDONT_11:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 220(sp)
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 224(sp)
  lw x5, 220(sp)
  lw x6, 224(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 228(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 232(sp)
  lw x5, 220(sp)
  lw x6, 232(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 236(sp)
  lw x5, 236(sp)
  lw t0, 0(t0)
  sw t0, 240(sp)
  lw x5, 240(sp)
  lw x6, 228(sp)
  sw t0, 0(t1)
  lw x5, 224(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 244(sp)
  lw x5, 244(sp)
  lw x6, 44(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_12
QuickSort_PLSDONT_12:
  j QuickSort_PLSDONT_13
QuickSort_PLSDONT_13:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 248(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 252(sp)
  lw x5, 248(sp)
  lw x6, 252(sp)
  slt t0, t0, t1
  sw t0, 256(sp)
  addi t0, sp, 260
  sw t0, 264(sp)
  li x5, 0
  lw x6, 264(sp)
  sw t0, 0(t1)
  lw x5, 256(sp)
  beqz t0, QuickSort_PLSDONT_26
  j QuickSort_PLSDONT_16
QuickSort_PLSDONT_26:
  j QuickSort_PLSDONT_17
QuickSort_PLSDONT_14:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 268(sp)
  lw x5, 268(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 272(sp)
  lw x5, 272(sp)
  lw x6, 44(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_13
QuickSort_PLSDONT_15:
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 276(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 280(sp)
  lw x5, 276(sp)
  lw x6, 280(sp)
  slt t0, t0, t1
  sw t0, 284(sp)
  lw x5, 284(sp)
  beqz t0, QuickSort_PLSDONT_27
  j QuickSort_PLSDONT_18
QuickSort_PLSDONT_27:
  j QuickSort_PLSDONT_19
QuickSort_PLSDONT_16:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 288(sp)
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 292(sp)
  lw x5, 288(sp)
  lw x6, 292(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 296(sp)
  lw x5, 296(sp)
  lw t0, 0(t0)
  sw t0, 300(sp)
  lw x5, 68(sp)
  lw t0, 0(t0)
  sw t0, 304(sp)
  lw x5, 300(sp)
  lw x6, 304(sp)
  slt t0, t0, t1
  sw t0, 308(sp)
  lw x5, 308(sp)
  li x6, 0
  sub t1, t0, t1
  snez t0, t1
  sw t0, 312(sp)
  lw x5, 312(sp)
  lw x6, 264(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_17
QuickSort_PLSDONT_17:
  lw x5, 264(sp)
  lw t0, 0(t0)
  sw t0, 316(sp)
  lw x5, 316(sp)
  beqz t0, QuickSort_PLSDONT_28
  j QuickSort_PLSDONT_14
QuickSort_PLSDONT_28:
  j QuickSort_PLSDONT_15
QuickSort_PLSDONT_18:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 320(sp)
  lw x5, 56(sp)
  lw t0, 0(t0)
  sw t0, 324(sp)
  lw x5, 320(sp)
  lw x6, 324(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 328(sp)
  lw x5, 44(sp)
  lw t0, 0(t0)
  sw t0, 332(sp)
  lw x5, 320(sp)
  lw x6, 332(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 336(sp)
  lw x5, 336(sp)
  lw t0, 0(t0)
  sw t0, 340(sp)
  lw x5, 340(sp)
  lw x6, 328(sp)
  sw t0, 0(t1)
  lw x5, 324(sp)
  li x6, 1
  sub t0, t0, t1
  sw t0, 344(sp)
  lw x5, 344(sp)
  lw x6, 56(sp)
  sw t0, 0(t1)
  j QuickSort_PLSDONT_19
QuickSort_PLSDONT_19:
  j QuickSort_PLSDONT_3
QuickSort_PLSDONT_20:
  li x10, 0
  addi sp, sp, 352
  ret

  .text
  .globl main
main:
  addi sp, sp, -160
main_PLSDONT_0:
  li x5, 10
  la x6, GLBVAL_0
  sw t0, 0(t1)
  addi t0, sp, 4
  sw t0, 44(sp)
  lw x5, 44(sp)
  li x6, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 48(sp)
  li x5, 4
  lw x6, 48(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 52(sp)
  li x5, 3
  lw x6, 52(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 56(sp)
  li x5, 9
  lw x6, 56(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 60(sp)
  li x5, 2
  lw x6, 60(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 64(sp)
  li x5, 0
  lw x6, 64(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 5
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 68(sp)
  li x5, 1
  lw x6, 68(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 6
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 72(sp)
  li x5, 6
  lw x6, 72(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 7
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 76(sp)
  li x5, 5
  lw x6, 76(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 8
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 80(sp)
  li x5, 7
  lw x6, 80(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 9
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 84(sp)
  li x5, 8
  lw x6, 84(sp)
  sw t0, 0(t1)
  addi t0, sp, 88
  sw t0, 92(sp)
  li x5, 0
  lw x6, 92(sp)
  sw t0, 0(t1)
  addi t0, sp, 96
  sw t0, 100(sp)
  li x5, 9
  lw x6, 100(sp)
  sw t0, 0(t1)
  lw x5, 44(sp)
  li x6, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 104(sp)
  lw x10, 104(sp)
  li x11, 0
  li x12, 9
  mv t0, ra
  sw t0, 0(sp)
  call QuickSort
  lw t0, 0(sp)
  mv ra, t0
  mv t0, a0
  sw t0, 108(sp)
  lw x5, 108(sp)
  lw x6, 92(sp)
  sw t0, 0(t1)
  j main_PLSDONT_1
main_PLSDONT_1:
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 112(sp)
  la x5, GLBVAL_0
  lw t0, 0(t0)
  sw t0, 116(sp)
  lw x5, 112(sp)
  lw x6, 116(sp)
  slt t0, t0, t1
  sw t0, 120(sp)
  lw x5, 120(sp)
  beqz t0, main_PLSDONT_5
  j main_PLSDONT_2
main_PLSDONT_5:
  j main_PLSDONT_3
main_PLSDONT_2:
  addi t0, sp, 124
  sw t0, 128(sp)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 132(sp)
  lw x5, 44(sp)
  lw x6, 132(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 136(sp)
  lw x5, 136(sp)
  lw t0, 0(t0)
  sw t0, 140(sp)
  lw x5, 140(sp)
  lw x6, 128(sp)
  sw t0, 0(t1)
  lw x10, 140(sp)
  mv t0, ra
  sw t0, 0(sp)
  call putint
  lw t0, 0(sp)
  mv ra, t0
  li x5, 10
  lw x6, 128(sp)
  sw t0, 0(t1)
  li x10, 10
  mv t0, ra
  sw t0, 0(sp)
  call putch
  lw t0, 0(sp)
  mv ra, t0
  lw x5, 132(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 144(sp)
  lw x5, 144(sp)
  lw x6, 92(sp)
  sw t0, 0(t1)
  j main_PLSDONT_1
main_PLSDONT_3:
  li x10, 0
  addi sp, sp, 160
  ret
main_PLSDONT_4:
  li x10, 0
  addi sp, sp, 160
  ret

