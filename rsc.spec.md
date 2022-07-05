# Tokens
## Punctuators
- \(
- \)
- \[
- \]
- \{
- \}
- \+
- \-
- \*
- \/
- \%
- \+=
- \-=
- \*=
- \/=
- \%=
- \&&
- \||
- \&
- \|
- \^
- \.
- \,
- \?
- \:
- \=
- \=>

## Keywords
- typeof
  - Gets the `stringrep` of the type of the object passed 
- final
  - denotes the value passed as immutable & a static type
  - `typeof final { a: 'hello' } == 'final { a: \'hello\' }'`
  - `typeof { a: 'hello' } == 'object { a: string }'`
- type
  - creates a defined object type, as opposed to a `final` type, or an anonymous object
  - allows static & non-static methods to be defined in the following block
- oneof 
  - defines a strict string type
  - `'test123'` is not valid for the type `oneof 'test1', 'test2'`
- if 
- else
- when
- for
- while
- loop
  - basically sugar for `while (1 || true)`
- static
- return
- in
  - sugar for iterable.contains


## Literals
- String constant
  - any characters enclosed by either single or double quotes
  - `'Hello' + "world!"`
- numbers
  - ints or floats
  - usual syntax for dec/bin/oct/hex ints
  - all numbers are 64 bit
- identifiers
  - they're idents, p ez
  - can start with a-zA-Z_, after that a-zA-Z_0-9
- bool
    - `true`
    - `false`
- nil

# Basic syntax
Semicolons are optional (for now)
```js
// Variables
name: string = "Charlie";
// <ident> <: typename>? <=> <expr> 

// function definitions
sayHi = (name: string) => print("Hello, $name")

// function calls
sayHi(name)
// <ident | path> <(> <args>? <)> 

// objects
{
    name = "Charlie",
    age = 21
}
// Trailing comma allowed, not required

```