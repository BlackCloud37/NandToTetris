// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@8192
D=A
@len  // store len
M=D

(KEYLOOP)  // listen to key status
@KBD  // pressed or not
D=M
@WHITE  // if D == 1, goto WHITE
D;JEQ
@BLACK  // else goto BLACK
0;JMP
  
  (WHITE)
  @i  // i = 0
  M=0

  (WHITEL1)
  @i
  D=M
  @len
  D=D-M  // D = i - 8192
  @KEYLOOP
  D;JEQ  // if i == D end loop
  
  @SCREEN  // paint
  D=A
  @i
  A=D+M
  M=0

  @i
  M=M+1

  @WHITEL1
  0;JMP

  
  (BLACK)
  @i  // i = 0
  M=0

  (BLACKL1)
  @i
  D=M
  @len
  D=D-M  // D = i - 8192
  @KEYLOOP
  D;JEQ  // if i == D end loop
  
  @SCREEN  // paint
  D=A
  @i
  A=D+M
  M=-1

  @i
  M=M+1

  @BLACKL1
  0;JMP