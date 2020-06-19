# rlox

## Table of Contents
<!-- TOC -->

- [rlox](#rlox)
    - [Table of Contents](#table-of-contents)
    - [General](#general)
    - [Language Specs](#language-specs)
    - [Grammars](#grammars)

<!-- /TOC -->

## General
A rust implementation of https://craftinginterpreters.com/


## Language Specs
## Grammars

```
program        = statement* EOF ;

declaration    = funDecl
               | varDecl
               | statement ;


funDecl        = "fun" function ;
function       = IDENTIFIER "(" parameters? ")" block;
parameters     = IDENTIFIER ( "," IDENTIFIER )* ;

varDecl        = "var" IDENTIFIER "=" expression ";" ;

statement      = exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

exprStmt       = expression ";" ;
forStmt        = "for" "(" ( varDecl | exprStmt | ";" )
                           expression? ";"
                           expression? ")" statement ;
ifStmt         = "if" "(" expression ")" statement ( "else" statement )?      ;
printStmt      = "print" expression ";" ;
returnStmt     = "return" expression? ";" ;
whileStmt      = "while" "(" expression ")" statement ;
block          = "{" declaration* "}" ;

expression     = assigment ;
assignment     = IDENTIFIER "=" equality
               | logic_or ;
logic_or       = logic_and ( "or" logic_and )* ;
logic_and      = equality ( "and" equality )* ;

arguments      = expression ( "," expression )* ;

equality       = comparison ( ( "!=" | "==" ) comparison )* ;
comparison     = addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       = multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication = unary ( ( "/" | "*" ) unary )* ;
unary          = ( "!" | "-" ) unary | call ;
call           = primary ( "(" arguments? ")" )* ;
primary        = NUMBER | STRING | IDENTIFIER | "true" | "false" | "nil"
               | "(" expression ")" ;
```