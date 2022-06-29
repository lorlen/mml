# MML - Minimal Markup Language

A minimal markup language that's designed to be more concise and readable than, but also compatible with some of the most commonly used markup or data interchange languages, such as XML or JSON.

## Example

```
html {
    head {
        title {
            "Example HTML in MML"
        }
        meta(charset: "utf-8")
    }
    body {
        p(id: "content") {
            "A paragraph"
        }
    }
}
```

## Building

To build the transpiler in release mode, run:

```
cargo build --release
```

The built executable will be located in `target/release/mml`.

## Usage

**TODO**

## Grammar

Formal grammar of the language, in W3C EBNF format. Primitive value elements are omitted for clarity.

```
element ::= name attr-list children?;

attr-list ::= '(' attr (',' attr)* ','? ')';
children ::= '{' (element* | string) '}';

attr ::= name (':' | '=') value;
value ::= integer | float | string | bool;
```
