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
expression = literal
           | unary
           | binary
           | grouping ;

literal    = NUMBER | STRING | "true" | "false" | "nil" ;
grouping   = "(" expression ")" ;
unary      = ( "-" | "!" ) expression ;
binary     = expression operator expression ;
operator   = "==" | "!=" | "<" | "<=" | ">" | ">="
           | "+"  | "-"  | "*" | "/" ;
```