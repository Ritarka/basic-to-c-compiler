# BASIC-to-c-compiler

This was a short project. I woke up the morning after Christmas with the burning desire to build a compiler. I had built an interpreter before for my Compilers and Interpreters class during undergrad, but it was not my best work. I wanted to improve my understanding of how compilers works and build something more robust. It was also a good opportunity for me to get more experience with Rust. I picked up the language recently (just for fun) and wanted to get some more experience with it.

I looked around the internet and came across an excellent [article](https://austinhenley.com/blog/teenytinycompiler1.html) by Austin Henley detailing his process of building a compiler. I followed his first couple steps (building a lexer and parser) and diverged afterwards. Following the early stages of his work was quite valuable: in my previous works, I had struggled with lexers and parsers the most and this time, I learned from his rather elegant engineering.

Afterwards, I diverged and started working on my project without much reference to his work. I built an Abstract Syntax Tree (AST), optimizer, and emitter.  
Technically I could have made the emitter during the Recursive Descent Parsing as Austin Henley did, but I wanted to add an optimization step as well, so during the Parsing, I built an AST using a bottom-up approach.  

Just as a sanity check, I started working on the emitter immediately after. I created some functions to print out the AST to help out with debugging. It took me a few attempts to find a suitable way to construct the AST and output the compiled code, but eventually I found a good solution.

Afterwards, I just built a simple optimizer. This just took constant values and combined them together. E.g `2 * 3` would be turned into `6`. See Example 1 below. I could have gone deeper with the optimizer, but decided it was a good place to end the project. I have built more complicated optimizers before for class, and so I didn't see the need repeat that.

Overall I:
- Learned how compilers are built
- Learned what an AST is how to build one
- Learned how to build a Lexer and Parser
- Gained more experience with Rust

## Program grammar  
```
program grammar

program ::= {statement}
statement ::= "PRINT" (expression | string) nl
    | "IF" comparison "THEN" nl {statement} "ENDIF" nl
    | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
    | "LABEL" ident nl
    | "GOTO" ident nl
    | "LET" ident "=" expression nl
    | "INPUT" ident nl
comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
expression ::= term {( "-" | "+" ) term}
term ::= unary {( "/" | "*" ) unary}
unary ::= ["+" | "-"] primary
primary ::= number | ident
nl ::= '\n'+
```

## Example 1
Input
```
LET foo = 1 * 2 * 3 * 4 * 5 * 6 * 7 + 1 + 2 + 3 - 2 * 3 
PRINT foo
```
  
Output  
```
#include <stdio.h>

int main(void) {
    float foo;

    foo = 5040;
    printf("%.2f\n", (float)(foo));
    
    return 0;
}
```

## Example 2
Input
```
# Compute average of given values.

LET a = 0
WHILE a < 1 REPEAT
    PRINT "Enter number of scores: "
    INPUT a
ENDWHILE

LET b = 0
LET s = 0
PRINT "Enter one value at a time: "
WHILE b < a REPEAT
    INPUT c
    LET s = s + c
    LET b = b + 1
ENDWHILE

PRINT "Average: "
PRINT s / a
```
  
Output  
```
#include <stdio.h>

int main(void) {
    float a;
    float b;
    float s;
    float c;

    a = 0;
    while (a < 1) {
        printf("Enter number of scores: \n");
        if (0 == scanf("%f", &a)) {
            a = 0;
            scanf("%*s");
        }
    }
    b = 0;
    s = 0;
    printf("Enter one value at a time: \n");
    while (b < a) {
        if (0 == scanf("%f", &c)) {
            c = 0;
            scanf("%*s");
        }
        s = s + c;
        b = b + 1;
    }
    printf("Average: \n");
    printf("%.2f\n", (float)(s / a));
    
    return 0;
}
```
