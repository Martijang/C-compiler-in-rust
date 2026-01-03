# C compiler written in rust

My toy porject to create C compiler in rust

--- 
    WARN: current lexer does not support all the keywords in C
          Current parser has some minor issuse.    
---

## todo:
1. Implement parser for compiler
2. Design my own IR: OK
3. Implement IR generation
4. asm generation for windows x86_64
---

## ISSUE:
---
The input C code: int interger

output: 
    Error: Expected identifier, found: I32
    error: process didn't exit successfully: `target\debug\rs.exe /xampp/htdocs/a.c` (exit code: 1) 


### What has been done:
- [OK] parsing cli args
- [OK] lexer 
- [] parser
- [] IR generation
- [] asm translation