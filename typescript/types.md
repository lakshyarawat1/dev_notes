# All Typescript types 

## Primitive Types

- `string` : Represents textual data, string values
- `number` : Represents numeric values, both integers and floating-point numbers
- `boolean` : Represents true or false values

## Arrays

- Used to store multiple values in a single variable
- `number[]` : Array of numbers
- `string[]` : Array of strings
- `boolean[]` : Array of booleans
- Basic syntax: `let arrayName: type[] = [value1, value2, ...]`

## Any

- Represents any type of value, can be used when the type is not known

## NoImplicitAny

- Compiler flag that prevents variables from being implicitly assigned the `any` type
- Helps catch potential errors by enforcing explicit type declarations

## Functions

- Functions can have explicit type annotations for parameters and return types
- Syntax: `function functionName(param1: type1, param2: type2):
    returnType { /* function body */ }`
- `Promise` : Represents a value that may be available now, or in the future, or never i.e. for JS Promises.

### Anonymous Functions

- Functions without a name, often used as arguments to other functions
- Parameters are automatically inferred
- Contextual typing because the function occured within informs what type it should have.

## Object Types

- Used to define custom data structures
- Syntax: `let objectName: { property1: type1, property2: type2 } = { property1: value1, property2: value2 }`
- Can also define methods within the object type
- If any property is not there, we get undefined instead of runtime error.

## Union Types

- We can combine multiple types into one using the pipe `|` operator
- Example: `let variableName: type1 | type2 = value`
- Only those operations will be valid that are valid for all types in the union

## Type aliases

- We can create a new name for an existing type using the `type` keyword
- Syntax: `type AliasName = ExistingType`

## Interfaces

- Used to define the structure of an object
- Interfaces can be extended to create new interfaces unlike types which cannot be extended
- Syntax: `interface InterfaceName { property1: type1; property2: type2; }`
- Functions can be defined in interfaces as well
