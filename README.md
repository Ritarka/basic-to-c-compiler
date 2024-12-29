# BASIC-to-C-compiler

Simple compiler built in Rust. With help and inspiration from Austin Henley's [blog](https://austinhenley.com/blog/teenytinycompiler1.html).

## Program grammar  
```
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
