// Math is normal
print([
    5 + 1 == 6,
    2 / 2 == 1,
    5 - 2 == 3,
    2 * 2 == 4,
]);

// functions
sayHi = (name) => {
    print("hi, $name");
    // or:
    print("how are you, ${name}");
}

// types
/* 
    built-in types:
        string
        number
        array
        object
        function
        nil
        any
    constructions:
        static : function
        final : any        
*/

// typeof
print(typeof ( 10 )) // -> "number"
print(typeof ( { a: "hello", b: "world" } )) // -> "object { a: string, b: string }"
print(typeof ( final { a: "hello", b: "world" } )) // -> "final { a: "hello", b: "world" }"



// Custom types
animal = type {
    // types have the same optional typing as variables
    name: string?;
    age: number;
    
    // Type methods
    // in methods, the `this` keyword can be dropped, .ident refers to the
    // current instance's value, note how one needs to qualify the static 
    // method call with its parent type, this is to avoid namespace pollution
    printInfo = () => print(animal.generateInformationString(.name, .age));

    stringInterpolationExample = () => print("my name is $.name")

    // Static methods 
    generateInformationString = 
        static (name: string, age: number) => "I am $name, $age years old :)";

}

// Extending a custom type 
// unlike a method definition in the type definition, the `this` keyword is
// required on extension methods, and is not valid in `static` scopes, as
// it refers to the current instance, which static methods do not have
animal.grow = () => this.age++

//

frank: animal = {
    name = "Franco",
    age = 5,
}

// Should print "I am Franco, 5 years old"
frank.printInfo()

frank.grow()

// Should print "I am Franco, 6 years old"
frank.printInfo()

// anonymous objects
lemon = {
    color = "yellow",
    weight = 200
}

apple = {
    color = "red",
    weight = 400
}

// string types

frankNicknames = oneof "Frank", "Franco", "Fritz", "Francesca"

// Function overloading
/*
    For functions with multiple definitions, we filter by functions that can accept the 
    argument passed (in this case we wouldn't be able to resolve a function if we passed
    a definite string or object type, because all definitions of `fib` expect _a_ number),
    then we order by the most restrictive argument type (in this case `final 1`), and 
    call the first definition that matches the argument passed.

    for `fib(<final> 3)`:
        fib(number { n = 3 }) => fib(number { n = 3 }) => fib(final 1)
    
    for `fib(1)`, it will match the literal value passed, not it's finality. the `final` in the
    function definition is to confirm that the argument will & can only be 1, thus it will call 
    the `final 1` implementation & return 1
*/
fib = (final 1) => 1
fib = (n: number) => n * fib(n - 1)

// procedural examples
fizzBuzzProcedural = (until: number) => {
    for i = [0, until) => {
        line = '';
        if i % 3 == 0 => line += 'Fizz';
        if i % 5 == 0 => line += 'Buzz';

        print(line.isEmpty ? i : line)
    }
}

fizzBuzzGood = (until: number) => {
    for i = [0, until) => {
        when {
            i % 15 == 0 => print("FizzBuzz")
            i % 3 == 0 => print("Fizz")
            i % 5 == 0 => print("Buzz")
        } else print(i)
    }
}