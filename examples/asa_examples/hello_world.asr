.data
msg: .string "Hello World" ; '\0' is added by default
.text
.global _start 
_start:
  mov reg1, msg ; mov the start address of the string in reg1
  ; NB: The compiler automatically replace "msg" by the starting address of the string
  sys 3         ; syscall to print, the address of the string is found in reg1
  
  mov reg1, 0   ; mov the ok code for the exit syscall. (Here reg1 is already == 0, but it's better to always mov 0 in reg1 before exit)
  sys 6         ; Exit the program
.end