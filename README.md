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

statement      = exprStmt
               | printStmt
               | declaration ;
exprStmt       = expression ";" ;
printStmt      = "print" expression ";" ;
declaration    = "var" IDENTIFIER ( "=" expression )? ";" ;

expression     = equality ;
equality       = comparison ( ( "!=" | "==" ) comparison )* ;
comparison     = addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       = multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication = unary ( ( "/" | "*" ) unary )* ;
unary          = ( "!" | "-" ) unary | primary ;
primary        = NUMBER | STRING | "true" | "false" | "nil"
               | IDENTIFIER | "(" expression ")" ;
```