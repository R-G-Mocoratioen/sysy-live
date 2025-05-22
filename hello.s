  .data
  .globl GLBVAL_0
GLBVAL_0:
  .zero 800

  .text
  .globl __pseudo__please_dont_give_same_name
__pseudo__please_dont_give_same_name:
__pseudo__please_dont_give_same_name_PLSDONT_0:
  ret









  .text
  .globl merge_sort
merge_sort:
  addi sp, sp, -448
merge_sort_PLSDONT_0:
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
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 20(sp)
  lw x5, 20(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 24(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw x5, 24(sp)
  lw x6, 28(sp)
  slt t0, t1, t0
  sw t0, 32(sp)
  lw x5, 32(sp)
  beqz t0, merge_sort_PLSDONT_21
  j merge_sort_PLSDONT_1
merge_sort_PLSDONT_21:
  j merge_sort_PLSDONT_2
merge_sort_PLSDONT_1:
  addi sp, sp, 448
  ret
merge_sort_PLSDONT_2:
  addi t0, sp, 36
  sw t0, 40(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 44(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 48(sp)
  lw x5, 44(sp)
  lw x6, 48(sp)
  add t0, t0, t1
  sw t0, 52(sp)
  lw x5, 52(sp)
  li x6, 2
  div t0, t0, t1
  sw t0, 56(sp)
  lw x5, 56(sp)
  lw x6, 40(sp)
  sw t0, 0(t1)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 60(sp)
  lw x5, 40(sp)
  lw t0, 0(t0)
  sw t0, 64(sp)
  lw x10, 60(sp)
  lw x11, 64(sp)
  mv t0, ra
  sw t0, 0(sp)
  call merge_sort
  lw t0, 0(sp)
  mv ra, t0
  lw x5, 40(sp)
  lw t0, 0(t0)
  sw t0, 68(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 72(sp)
  lw x10, 68(sp)
  lw x11, 72(sp)
  mv t0, ra
  sw t0, 0(sp)
  call merge_sort
  lw t0, 0(sp)
  mv ra, t0
  addi t0, sp, 76
  sw t0, 80(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 84(sp)
  lw x5, 84(sp)
  lw x6, 80(sp)
  sw t0, 0(t1)
  addi t0, sp, 88
  sw t0, 92(sp)
  lw x5, 40(sp)
  lw t0, 0(t0)
  sw t0, 96(sp)
  lw x5, 96(sp)
  lw x6, 92(sp)
  sw t0, 0(t1)
  addi t0, sp, 100
  sw t0, 104(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw x5, 108(sp)
  lw x6, 104(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_4
merge_sort_PLSDONT_3:
  j merge_sort_PLSDONT_2
merge_sort_PLSDONT_4:
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 112(sp)
  lw x5, 40(sp)
  lw t0, 0(t0)
  sw t0, 116(sp)
  lw x5, 112(sp)
  lw x6, 116(sp)
  slt t0, t0, t1
  sw t0, 120(sp)
  addi t0, sp, 124
  sw t0, 128(sp)
  li x5, 0
  lw x6, 128(sp)
  sw t0, 0(t1)
  lw x5, 120(sp)
  beqz t0, merge_sort_PLSDONT_26
  j merge_sort_PLSDONT_7
merge_sort_PLSDONT_26:
  j merge_sort_PLSDONT_8
merge_sort_PLSDONT_5:
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 132(sp)
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 136(sp)
  lw x5, 132(sp)
  lw x6, 136(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 140(sp)
  lw x5, 140(sp)
  lw t0, 0(t0)
  sw t0, 144(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 148(sp)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 152(sp)
  lw x5, 148(sp)
  lw x6, 152(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 156(sp)
  lw x5, 156(sp)
  lw t0, 0(t0)
  sw t0, 160(sp)
  lw x5, 144(sp)
  lw x6, 160(sp)
  slt t0, t0, t1
  sw t0, 164(sp)
  lw x5, 164(sp)
  beqz t0, merge_sort_PLSDONT_28
  j merge_sort_PLSDONT_9
merge_sort_PLSDONT_28:
  j merge_sort_PLSDONT_10
merge_sort_PLSDONT_6:
  j merge_sort_PLSDONT_12
merge_sort_PLSDONT_7:
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 168(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 172(sp)
  lw x5, 168(sp)
  lw x6, 172(sp)
  slt t0, t0, t1
  sw t0, 176(sp)
  lw x5, 176(sp)
  li x6, 0
  sub t1, t0, t1
  snez t0, t1
  sw t0, 180(sp)
  lw x5, 180(sp)
  lw x6, 128(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_8
merge_sort_PLSDONT_8:
  lw x5, 128(sp)
  lw t0, 0(t0)
  sw t0, 184(sp)
  lw x5, 184(sp)
  beqz t0, merge_sort_PLSDONT_32
  j merge_sort_PLSDONT_5
merge_sort_PLSDONT_32:
  j merge_sort_PLSDONT_6
merge_sort_PLSDONT_9:
  la x5, GLBVAL_0
  li x6, 1
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 188(sp)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 192(sp)
  lw x5, 188(sp)
  lw x6, 192(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 196(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 200(sp)
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 204(sp)
  lw x5, 200(sp)
  lw x6, 204(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 208(sp)
  lw x5, 208(sp)
  lw t0, 0(t0)
  sw t0, 212(sp)
  lw x5, 212(sp)
  lw x6, 196(sp)
  sw t0, 0(t1)
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 216(sp)
  lw x5, 216(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 220(sp)
  lw x5, 220(sp)
  lw x6, 80(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_11
merge_sort_PLSDONT_10:
  la x5, GLBVAL_0
  li x6, 1
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 224(sp)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 228(sp)
  lw x5, 224(sp)
  lw x6, 228(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 232(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 236(sp)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 240(sp)
  lw x5, 236(sp)
  lw x6, 240(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 244(sp)
  lw x5, 244(sp)
  lw t0, 0(t0)
  sw t0, 248(sp)
  lw x5, 248(sp)
  lw x6, 232(sp)
  sw t0, 0(t1)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 252(sp)
  lw x5, 252(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 256(sp)
  lw x5, 256(sp)
  lw x6, 92(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_11
merge_sort_PLSDONT_11:
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 260(sp)
  lw x5, 260(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 264(sp)
  lw x5, 264(sp)
  lw x6, 104(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_4
merge_sort_PLSDONT_12:
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 268(sp)
  lw x5, 40(sp)
  lw t0, 0(t0)
  sw t0, 272(sp)
  lw x5, 268(sp)
  lw x6, 272(sp)
  slt t0, t0, t1
  sw t0, 276(sp)
  lw x5, 276(sp)
  beqz t0, merge_sort_PLSDONT_37
  j merge_sort_PLSDONT_13
merge_sort_PLSDONT_37:
  j merge_sort_PLSDONT_14
merge_sort_PLSDONT_13:
  la x5, GLBVAL_0
  li x6, 1
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 280(sp)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 284(sp)
  lw x5, 280(sp)
  lw x6, 284(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 288(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 292(sp)
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 296(sp)
  lw x5, 292(sp)
  lw x6, 296(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 300(sp)
  lw x5, 300(sp)
  lw t0, 0(t0)
  sw t0, 304(sp)
  lw x5, 304(sp)
  lw x6, 288(sp)
  sw t0, 0(t1)
  lw x5, 80(sp)
  lw t0, 0(t0)
  sw t0, 308(sp)
  lw x5, 308(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 312(sp)
  lw x5, 312(sp)
  lw x6, 80(sp)
  sw t0, 0(t1)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 316(sp)
  lw x5, 316(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 320(sp)
  lw x5, 320(sp)
  lw x6, 104(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_12
merge_sort_PLSDONT_14:
  j merge_sort_PLSDONT_15
merge_sort_PLSDONT_15:
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 324(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 328(sp)
  lw x5, 324(sp)
  lw x6, 328(sp)
  slt t0, t0, t1
  sw t0, 332(sp)
  lw x5, 332(sp)
  beqz t0, merge_sort_PLSDONT_41
  j merge_sort_PLSDONT_16
merge_sort_PLSDONT_41:
  j merge_sort_PLSDONT_17
merge_sort_PLSDONT_16:
  la x5, GLBVAL_0
  li x6, 1
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 336(sp)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 340(sp)
  lw x5, 336(sp)
  lw x6, 340(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 344(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 348(sp)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 352(sp)
  lw x5, 348(sp)
  lw x6, 352(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 356(sp)
  lw x5, 356(sp)
  lw t0, 0(t0)
  sw t0, 360(sp)
  lw x5, 360(sp)
  lw x6, 344(sp)
  sw t0, 0(t1)
  lw x5, 92(sp)
  lw t0, 0(t0)
  sw t0, 364(sp)
  lw x5, 364(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 368(sp)
  lw x5, 368(sp)
  lw x6, 92(sp)
  sw t0, 0(t1)
  lw x5, 104(sp)
  lw t0, 0(t0)
  sw t0, 372(sp)
  lw x5, 372(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 376(sp)
  lw x5, 376(sp)
  lw x6, 104(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_15
merge_sort_PLSDONT_17:
  j merge_sort_PLSDONT_18
merge_sort_PLSDONT_18:
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 380(sp)
  lw x5, 16(sp)
  lw t0, 0(t0)
  sw t0, 384(sp)
  lw x5, 380(sp)
  lw x6, 384(sp)
  slt t0, t0, t1
  sw t0, 388(sp)
  lw x5, 388(sp)
  beqz t0, merge_sort_PLSDONT_45
  j merge_sort_PLSDONT_19
merge_sort_PLSDONT_45:
  j merge_sort_PLSDONT_20
merge_sort_PLSDONT_19:
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 392(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 396(sp)
  lw x5, 392(sp)
  lw x6, 396(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 400(sp)
  la x5, GLBVAL_0
  li x6, 1
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 404(sp)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 408(sp)
  lw x5, 404(sp)
  lw x6, 408(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 412(sp)
  lw x5, 412(sp)
  lw t0, 0(t0)
  sw t0, 416(sp)
  lw x5, 416(sp)
  lw x6, 400(sp)
  sw t0, 0(t1)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 420(sp)
  lw x5, 420(sp)
  li x6, 1
  add t0, t0, t1
  sw t0, 424(sp)
  lw x5, 424(sp)
  lw x6, 8(sp)
  sw t0, 0(t1)
  j merge_sort_PLSDONT_18
merge_sort_PLSDONT_20:
  addi sp, sp, 448
  ret

  .text
  .globl main
main:
  addi sp, sp, -48
main_PLSDONT_0:
  addi t0, sp, 4
  sw t0, 8(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 12(sp)
  lw x5, 12(sp)
  li x6, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 16(sp)
  lw x10, 16(sp)
  mv t0, ra
  sw t0, 0(sp)
  call getarray
  lw t0, 0(sp)
  mv ra, t0
  mv t0, a0
  sw t0, 20(sp)
  lw x5, 20(sp)
  lw x6, 8(sp)
  sw t0, 0(t1)
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 24(sp)
  li x10, 0
  lw x11, 24(sp)
  mv t0, ra
  sw t0, 0(sp)
  call merge_sort
  lw t0, 0(sp)
  mv ra, t0
  lw x5, 8(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  la x5, GLBVAL_0
  li x6, 0
  li t2, 400
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 32(sp)
  lw x5, 32(sp)
  li x6, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 36(sp)
  lw x10, 28(sp)
  lw x11, 36(sp)
  mv t0, ra
  sw t0, 0(sp)
  call putarray
  lw t0, 0(sp)
  mv ra, t0
  li x10, 0
  addi sp, sp, 48
  ret
main_PLSDONT_1:
  li x10, 0
  addi sp, sp, 48
  ret

