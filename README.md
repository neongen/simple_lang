
![simple_lang Logo](./assets/logo/simple_lang_logo_design.png)

> A simple programming language created by GenAI with minimum human intervention.

---

## 🌞 What is `simple_lang`?

**`simple_lang`** is a minimal, strict, statically-typed programming language designed and implemented via a generative AI-driven development workflow. 

It supports:

- Explicit types (no inference or implicit conversions)


  ✅ Allowed (with explicit conversion):
  ```
  text: string = int_to_string(42);
  ```
  🚫 Not Allowed:
  ```
  text: string = 42;  // ❌ No implicit i32 → string
  ```

- Basic arithmetic operations (`+`, `-`, `*`, `/`)

  ```
  count: i32 = 42;
  ```
  
- Simple string and integer handling

  You can declare variables of type string.
  Assign and store literal text values.

  ```
  message: string = "Hello, World! Your code belongs to the Entity!";
  ```
  
- Function declarations with typed parameters and return values

  Parameter types must be specified, and
  the return type must also be specified.

  ```
  add_numbers: function(a: i32, b: i32) -> i32 {
    return a + b;
  };
  ```

- Strict syntax rules (every statement ends in `;`)

  ```
  count: i32 = 42;
  ```
  
- In-file unit tests and readable modular code

---

## 🚀 Run the Demo

Run a sample `simple_lang` program using the built-in runner:

hello_world.lang
```
main: function() -> i32 {
    message: string = "Hello, World! Your code belongs to the Entity!";
    count: i32 = 42;
    
    // Display message
    print(message);
    
    // Arithmetic operations
    result: i32 = count + 8;
    print_number(result);

    return 0;
};
```
Build and Run

```bash
cd simple_lang_demo_runner
cargo run
```

⚠️ **Status: Work in Progress**  
This project is not finished yet — it is under development and evolving. 
