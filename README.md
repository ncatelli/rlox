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

declaration    = varDecl
               | statement ;

varDecl        = "var" IDENTIFIER "=" expression ";" ;

statement      = exprStmt
               | ifStmt
               | printStmt
               | block ;

exprStmt       = expression ";" ;
ifStmt         = "if" "(" expression ")" statement ( "else" statement )? ;
printStmt      = "print" expression ";" ;
block          = "{" declaration* "}" ;

expression     = assigment ;
assignment     = IDENTIFIER "=" equality
               | equality ;
equality       = comparison ( ( "!=" | "==" ) comparison )* ;
comparison     = addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       = multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication = unary ( ( "/" | "*" ) unary )* ;
unary          = ( "!" | "-" ) unary | primary ;
primary        = NUMBER | STRING | IDENTIFIER | "true" | "false" | "nil"
               | "(" expression ")" ;
```