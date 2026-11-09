# Understanding Variables in Rust

## 1. The Foundation: Why Do We Need Variables?

The fundamental ideas behind most programming languages are similar. What changes between languages is their syntax and the specific rules they follow.

For example, almost every programming language has variables, data types, functions, and conditions. Rust has these concepts too, but it introduces particular rules around variables, such as immutability and ownership.

We will learn these concepts using Rust. More importantly, we will understand how to look at a real problem, identify the information our program needs, and express that information using Rust.

Let's begin with a practical example.

Imagine that you own a tea shop.

Every day, you sell a different number of cups of tea:

```text
Monday:     120 cups
Tuesday:    150 cups
Wednesday:   95 cups
```

Your shop needs a program to calculate daily sales, revenue, costs, and profit.

To do these calculations, the program needs to work with information such as:

- The number of cups sold.
- The selling price of each cup.
- The cost of making each cup.
- The customer's name.
- The payment method.
- The total profit.

But how does the program remember these values and refer to them when needed?

This is where variables become useful.

## 2. What Is a Variable?

A variable is a named binding that allows a program to refer to a value.

Let's understand this using a simple example.

Suppose you have the number `120`, which represents the number of cups sold on Monday.

Your program might need this number several times. For example, it may use it to calculate total revenue, total cost, and daily profit.

Instead of repeatedly writing `120`, we can give this information a name.

In Rust:

```rust
let cups_sold = 120;
```

Now, whenever we need the number of cups sold, we can use `cups_sold`.

For example:

```rust
let cups_sold = 120;

let selling_price = 25;
let cost_per_cup = 10;

let total_revenue = cups_sold * selling_price;
let total_cost = cups_sold * cost_per_cup;

let daily_profit = total_revenue - total_cost;
```

Notice what happened.

We gave names to the information our program needs. We can now use those names in different calculations without repeatedly writing the original numbers.

That is the central purpose of a variable:

**A variable gives a value a name so that our program can access, reuse, and work with that value.**

### Do We Always Need Variables?

No.

Sometimes we calculate something and use the result immediately.

For example:

```rust
println!("{}", 10 + 20);
```

We do not need to declare a variable because we calculate the result and immediately print it.

However, suppose we need the result in different parts of our program:

```rust
let result = 10 + 20;

println!("{result}");
println!("{result}");
```

Now giving the result a name is useful.

The idea is simple: we declare a variable when having a name for a value helps our program work with it.

### Does a Variable Have to Change?

No.

The name *variable* might make you think that its value must change. However, a variable is useful even when its value remains the same.

For example:

```rust
let selling_price = 25;
```

We might use `selling_price` in ten different calculations without ever changing its value.

It is still a useful variable because it gives us a name through which we can access the selling price.

Now consider another example:

```rust
let mut cups_sold = 120;
```

The number of cups sold may increase as more customers buy tea.

We have two situations:

1. We need a name for a value, but we do not need to change it.
2. We need a name for a value, and we also need to update it.

Rust handles these situations through immutability and mutability. We will understand this shortly.

---

## 3. Understanding the Parts of a Variable

Consider this Rust declaration:

```rust
let cups_sold: u32 = 120;
```

It contains several important parts:

```text
let          → Introduces a new variable binding

cups_sold    → Name of the variable

: u32        → Type of the value

=            → Assigns the initial value

120          → Actual value

;            → Ends the statement
```

Let's understand what each part means.

### 1. Name

The name is what we use to refer to the information.

In our example:

```rust
cups_sold
```

We know that this name represents the number of cups sold.

### 2. Value

The value is the actual data currently associated with the variable.

In our example:

```rust
120
```

This is the current number of cups sold.

### 3. Type

The type tells Rust what kind of information the value represents.

In our example:

```rust
u32
```

This tells Rust that the value is an unsigned 32-bit integer.

### 4. Mutability

Mutability determines whether we are allowed to assign another value to the same variable binding.

We will explore this in a separate section.

Together, these parts help us understand what information a variable represents and how our program can work with it.

---

## 4. The Memory Slot: What Happens Inside the Computer?

Now let's understand something deeper.

When we write:

```rust
let cups_sold: u32 = 120;
```

Where does that value exist while our program is running?

Your computer has RAM (Random Access Memory). While a program is running, it uses memory to work with information, instructions, and other data.

For a simple mental model, imagine RAM as a large collection of storage spaces.

Each space can be thought of as having:

- A location or address.
- Some available space.
- A value stored there.

Imagine something like this:

```text
              COMPUTER MEMORY
              (Mental Model)


       +---------------------------+
       |                           |
       |        cups_sold          |  ← Name
       |                           |
       |       +---------+         |
       |       |   120   |         |  ← Value
       |       +---------+         |
       |                           |
       |        Type: u32          |  ← Type
       |                           |
       +---------------------------+
```

Think of it as a labelled storage slot.

The slot has a name that allows us to refer to its associated value.

When we write:

```rust
let cups_sold = 120;
```

Conceptually, the following things happen:

**Step 1: We declare a new binding.**

The `let` keyword tells Rust that we are introducing a new variable binding.

**Step 2: We give it a name.**

We call it `cups_sold`.

**Step 3: We provide an initial value.**

The initial value is `120`.

**Step 4: Rust determines its type.**

Rust needs to know what kind of value it is working with.

**Step 5: We can use the name later.**

Whenever we write `cups_sold`, the compiled program can access the value associated with that binding.

The relationship looks like this:

```text
           RUST CODE

           cups_sold
                |
                |
                v
          Associated value
                |
                v
               120
```

The important idea is that the name gives us a way to refer to the value.

### Is the Variable Name Actually Stored in RAM?

Not necessarily.

The name `cups_sold` is mainly useful to us as programmers.

During compilation, Rust resolves variable names and generates machine instructions that work with the corresponding values.

The computer does not need to understand that `cups_sold` means the number of tea cups sold.

It only needs to execute the instructions produced by the compiler.

There is another important detail: the compiler might keep a value in a CPU register, place it in memory, or optimize away its storage entirely when possible.

Therefore, the memory-slot analogy is a way to understand the relationship between a name and its associated value. It does not mean that every variable always occupies one permanent physical memory location.

---

## 5. Understanding Values and Changes Over Time

A variable represents the current value associated with its binding.

Let's return to our tea shop.

```text
Monday:      120 cups sold
Tuesday:     150 cups sold
Wednesday:    95 cups sold
```

The information we are tracking is the same:

**How many cups have been sold?**

However, its value changes from day to day.

We can represent the current number using a mutable variable:

```rust
let mut cups_sold = 120;

cups_sold = 150;
```

Initially:

```text
cups_sold
    |
    v
  +-----+
  | 120 |
  +-----+
```

After reassignment:

```text
cups_sold
    |
    v
  +-----+
  | 150 |
  +-----+
```

The current value is now `150`.

Notice something important: the old value is replaced in this variable binding.

The variable does not automatically remember that its previous value was `120`.

If we need to preserve the numbers for Monday, Tuesday, and Wednesday, we need a different way to organize the information.

For example:

```rust
let daily_sales = [120, 150, 95];
```

Now the three values are stored together in an array.

This gives us an important distinction:

- A variable can represent the current value we are working with.
- A collection can hold multiple related values.
- A database can preserve information beyond the lifetime of the running program.

We choose the appropriate way to represent information based on what our application needs to do.

---

## 6. What Does the Equal Sign (`=`) Mean?

In Rust, the equal sign is used to assign a value.

Consider:

```rust
let cups_sold = 120;
```

We can read it as:

"Create a new binding named `cups_sold` and initialize it with the value `120`."

The equal sign does not mean that the name and the value are mathematically equal.

In programming, we should understand the difference between `=` and `==`.

### Assignment: `=`

```rust
let mut cups_sold = 120;

cups_sold = 150;
```

Here, we assign `150` to the mutable binding.

### Comparison: `==`

```rust
let cups_sold = 120;

let is_equal = cups_sold == 120;
```

Here, Rust compares the value of `cups_sold` with `120`.

The result is a Boolean value:

```rust
true
```

So remember:

```text
=   → Assign a value

==  → Compare two values
```

---

## 7. Why Do We Need Data Types?

Imagine that our tea shop program needs to work with these two pieces of information:

```text
Number of cups sold: 120

Customer name: Mekdes
```

Both are values, but they represent different kinds of information.

The first is a number.

The second is text.

The computer needs to know how to represent and work with each kind of value.

That is why programming languages use data types.

**A data type tells Rust what kind of value a binding represents and which operations are valid for that value.**

For example:

| Information | Example | Rust type |
|---|---|---|
| Number of cups | `120` | `u32` |
| Price | `25.50` | `f64` |
| Customer name | `"Mekdes"` | `String` |
| Is paid? | `true` | `bool` |
| First letter | `'M'` | `char` |

Let's look at some examples:

```rust
let cups_sold: u32 = 120;

let customer_name: String = String::from("Mekdes");

let is_paid: bool = true;
```

Each variable represents a different kind of information.

Rust uses the types to understand which operations are allowed.

For example, multiplying two numbers is valid:

```rust
let total = 10 * 5;
```

But multiplying a customer's name by a selling price does not make sense:

```rust
// Not valid:
let total = "Mekdes" * 25;
```

The types help Rust detect this kind of problem before the program runs.

### Common Rust Types

Here are some types we will use frequently.

**Integer types**

Integers represent whole numbers without decimal points.

Examples:

```rust
let cups_sold: u32 = 120;
let temperature: i32 = -5;
```

`u32` represents an unsigned integer. It cannot represent negative numbers.

`i32` represents a signed integer. It can represent both positive and negative numbers.

**Floating-point types**

Floating-point types represent numbers with decimal points.

```rust
let selling_price: f64 = 25.50;
```

`f64` is commonly used for floating-point calculations.

**Boolean type**

A Boolean represents one of two possible values:

```rust
let is_paid: bool = true;
```

The values are `true` and `false`.

**Character type**

A character represents one Unicode character:

```rust
let first_letter: char = 'M';
```

Notice that Rust uses single quotes for a `char`.

**String types**

Rust has two common string types that beginners should recognize.

A string literal has the type `&str`:

```rust
let customer_name: &str = "Mekdes";
```

An owned, growable string uses `String`:

```rust
let customer_name: String = String::from("Mekdes");
```

We will understand why `String` is important when we study ownership.

### Does Rust Always Require Us to Write the Type?

No.

Rust can often determine a value's type from the initial value. This is called **type inference**.

For example:

```rust
let cups_sold = 120;
```

Rust recognizes that `120` is an integer.

For an ordinary integer literal like this, Rust normally infers `i32` when there is no other information.

We can also tell Rust the type ourselves:

```rust
let cups_sold: u32 = 120;
```

The `: u32` part is called a type annotation.

We use type annotations when we want to make the intended type clear or when Rust needs more information.

---

## 8. Mutability: Can We Change the Value?

Now we understand that a variable has a name, a value, and a type.

But there is another important question:

**Are we allowed to assign another value to this variable?**

In Rust, variables are immutable by default.

Let's understand what that means.

### Immutable Bindings

```rust
let cups_sold = 120;
```

After initialization, we cannot assign another value to this same binding.

For example:

```rust
let cups_sold = 120;

// Not allowed:
cups_sold = 150;
```

Rust will report an error.

Why?

Because Rust wants us to make an explicit decision about which bindings are allowed to change.

If a value does not need to be reassigned, we can leave it immutable.

This helps prevent accidental changes.

### Mutable Bindings

When we know that a binding needs reassignment, we use the `mut` keyword.

```rust
let mut cups_sold = 120;

cups_sold = 150;
```

Now reassignment is allowed.

The `mut` keyword tells Rust:

"This binding is allowed to be assigned another value."

We can visualize the difference.

Without `mut`:

```text
let cups_sold = 120;

      cups_sold
          |
          v
         120

   Reassignment is not allowed.
```

With `mut`:

```text
let mut cups_sold = 120;

      cups_sold
          |
          v
         120

      cups_sold = 150;

      cups_sold
          |
          v
         150

   Reassignment is allowed.
```

### Does Every Variable Need `mut`?

No.

We should decide based on how the program uses each binding.

Consider:

```rust
let selling_price = 25;
let cost_per_cup = 10;

let mut cups_sold = 120;
```

The selling price and cost per cup do not need reassignment in this example.

However, the number of cups sold may increase.

Therefore, we use `mut` only for `cups_sold`.

An important distinction:

A value may change in the real world without requiring a mutable binding in our program.

For example, the selling price might change next month. That does not mean the current `selling_price` binding must be mutable. It depends on whether our program needs to reassign that binding while it is running.

**Remember: `mut` controls whether we can change a binding through reassignment.**

---

## 9. Identifying Variables from a Real Problem

Now let's move from understanding variables to thinking like a developer.

Imagine someone gives us this requirement:

"I want to build a system for my tea shop that tracks how many cups I sell each day, the price per cup, the cost to make each cup, the customer's name, whether the customer paid by cash or UPI, and calculates the daily profit."

Before writing Rust code, we should understand the problem.

We should not immediately start writing `let` statements.

First, ask:

**What information does this system need to remember, access, update, or use in calculations?**

Let's break the requirement into smaller parts.

### 1. Number of Cups Sold

The system needs to know how many cups have been sold.

For example:

```text
120
150
95
```

We can represent the current number using:

```rust
let mut cups_sold: u32 = 120;
```

Why `u32`?

Because the number of cups sold is a non-negative whole number.

Why `mut`?

Because the number may increase when new sales happen.

### 2. Selling Price Per Cup

The system also needs to know how much the shop charges for each cup.

For example:

```rust
let selling_price_per_cup: f64 = 25.50;
```

We need this value when calculating revenue.

The price might change between shops or over time. However, if this particular binding does not need reassignment while the program runs, ordinary `let` is enough.

### 3. Cost Per Cup

The system needs to know how much it costs to make one cup.

```rust
let cost_per_cup: f64 = 10.00;
```

This is different from the selling price.

We need both because profit depends on the difference between revenue and cost.

### 4. Customer Name

The system needs to work with customer information.

For example:

```rust
let customer_name = String::from("Mekdes");
```

The value is text, so we use `String`.

### 5. Payment Method

The system also needs to know how the customer paid.

For a simple example:

```rust
let payment_method = "cash";
```

However, a more structured application might have a defined set of payment methods.

Rust provides a feature called an `enum` for representing a fixed set of choices:

```rust
enum PaymentMethod {
    Cash,
    Upi,
}
```

We can then create a variable:

```rust
let payment_method = PaymentMethod::Cash;
```

This variable represents one of the payment methods defined by our program.

### 6. Daily Profit

Finally, the system needs to calculate daily profit.

The formula is:

```text
Daily Profit = Total Revenue - Total Cost

Total Revenue = Cups Sold × Selling Price

Total Cost = Cups Sold × Cost Per Cup
```

In Rust:

```rust
let total_revenue = cups_sold as f64 * selling_price_per_cup;

let total_cost = cups_sold as f64 * cost_per_cup;

let daily_profit = total_revenue - total_cost;
```

Notice what we are doing.

We are using previously declared variables to calculate new values.

Then we give the results meaningful names:

```rust
total_revenue
total_cost
daily_profit
```

This makes the program easier to understand and allows us to reuse the calculated values.

One important detail: these calculations use the values available when the calculations are performed. If `cups_sold` changes afterward, the previous `daily_profit` value does not update automatically. We must calculate it again when we need the new result.

### A Note About Money

For learning, `f64` makes the calculation easy to understand.

However, real applications should generally avoid using floating-point numbers for exact monetary calculations.

A better approach is to store money in the smallest currency unit.

For example, if the price is 25.50, store it as 2550 cents:

```rust
let selling_price_cents: u64 = 2550;
let cost_per_cup_cents: u64 = 1000;
```

This avoids common floating-point precision problems.

---

## 10. How Should We Think When Identifying Variables?

When reading a requirement, we need to understand the information before translating it into Rust.

Here is a practical way to approach it.

### Question 1: What Information Does the System Need?

Look at the problem and identify the information involved.

In our tea shop example:

- Number of cups sold.
- Selling price.
- Cost per cup.
- Customer name.
- Payment method.
- Daily profit.

These are possible variables or data fields.

However, not every word in a problem needs to become a variable. First, understand what information the system actually needs to work with.

### Question 2: Does the Information Need to Change?

For example:

- The number of cups sold increases throughout the day.
- The selling price may change when the shop updates its menu.
- The customer name changes from one customer to another.
- The payment method changes from one transaction to another.

This helps us understand how the information behaves.

### Question 3: Along Which Dimension Does It Change?

This is an important question because different pieces of information may belong to different things.

Consider:

| Information | How it changes |
|---|---|
| Cups sold | Day by day |
| Selling price | When the shop updates its price |
| Cost per cup | When production costs change |
| Customer name | Customer by customer |
| Payment method | Transaction by transaction |

Notice that these pieces of information do not all belong to the same record.

For example:

- `cups_sold` belongs to a daily sales record.
- `customer_name` belongs to a customer.
- `payment_method` belongs to a transaction.

This helps us understand how to organize our information later.

### Question 4: What Type of Information Is It?

Ask what kind of value the program needs:

- A whole number?
- A decimal number?
- Text?
- A true/false value?
- A collection of values?
- A more complex structure?

This helps us choose the appropriate Rust type.

### Question 5: Does the Binding Need to Be Mutable?

Ask whether the program needs to assign another value to the same binding.

If yes, use `mut`.

If not, ordinary `let` is enough.

### Question 6: Do We Need One Value or Multiple Values?

Sometimes one variable is enough.

Sometimes we need a collection or a structure containing several related values.

Understanding the problem helps us make that decision.

---

## 11. A Variable Can Represent More Than One Simple Value

A variable does not always represent just one number or one piece of text.

Its value can also be a collection or a more complex structure.

### Arrays

Suppose we want to remember the number of cups sold on three different days.

We can use an array:

```rust
let daily_sales = [120, 150, 95];
```

Here, one variable represents an array containing three numbers.

### Vectors

A vector is a collection that can grow or shrink while the program runs.

```rust
let mut daily_sales = vec![120, 150, 95];

daily_sales.push(180);
```

We use a vector when we need a growable collection.

### Structures

Now suppose we want to represent a customer.

A customer is not just one number or one piece of text.

A customer may have:

- A name.
- A payment method.

We can organize this information using a Rust `struct`.

```rust
struct Customer {
    name: String,
    payment_method: String,
}
```

Then we create a variable that holds a customer value:

```rust
let customer = Customer {
    name: String::from("Mekdes"),
    payment_method: String::from("cash"),
};
```

The variable `customer` now refers to a value containing multiple related pieces of information.

This gives us another important idea:

**A variable provides a name for a value. That value can be simple or complex.**

The type determines the structure of the value.

---

## 12. Understanding Variable Names

Consider this code:

```rust
let a = 120;
let b = 25;
let c = 10;

let d = a * b - a * c;
```

The code may work, but someone reading it has to guess what the names mean.

What does `a` represent?

What is `b`?

What does `d` contain?

Now compare it with:

```rust
let cups_sold = 120;
let selling_price = 25;
let cost_per_cup = 10;

let daily_profit =
    cups_sold * selling_price - cups_sold * cost_per_cup;
```

The second example is much easier to understand.

Why?

Because the variable names describe the information they represent.

**Variable names are mainly for programmers, not for the computer.**

The computer uses identifiers to refer to bindings in the program. The name itself does not explain the real-world meaning of the information to the computer.

Good names help us understand the purpose of a value, read calculations, maintain code, and work with other developers.

### Rust Naming Convention

Different programming languages follow different naming conventions.

For example, JavaScript commonly uses camelCase:

```javascript
let cupsSold = 120;
```

Rust normally uses snake_case:

```rust
let cups_sold = 120;
```

In snake_case, we use lowercase letters and separate words with underscores.

Examples:

```rust
cups_sold
customer_name
selling_price
daily_profit
total_revenue
```

For constants, Rust normally uses uppercase letters with underscores:

```rust
const MAX_CUPS_PER_ORDER: u32 = 100;
```

Following the language's naming convention makes code more consistent and easier for other Rust developers to read.

---

## 13. Understanding Constants

Sometimes our program needs information that should remain fixed.

For example, imagine that our tea shop allows a maximum of 100 cups in one order.

We can represent this using a constant:

```rust
const MAX_CUPS_PER_ORDER: u32 = 100;
```

A constant is a named value that cannot be changed after it is defined.

Rust treats constants differently from ordinary variable bindings.

A Rust constant:

- Must have an explicit type.
- Cannot use `mut`.
- Must have a value that can be determined at compile time.
- Normally uses an uppercase name with underscores.

For example:

```rust
const MAX_CUPS_PER_ORDER: u32 = 100;
const SHOP_NAME: &str = "Habesha Tea";
const TAX_RATE: f64 = 0.15;
```

### What Is the Difference Between `let` and `const`?

Consider:

```rust
let cups_sold = 120;

const MAX_CUPS_PER_ORDER: u32 = 100;
```

`cups_sold` is an ordinary variable binding.

`MAX_CUPS_PER_ORDER` is a constant.

There is an important distinction:

An ordinary `let` binding can be immutable, but it is still not a constant.

A constant must satisfy Rust's compile-time rules and is intended to represent a fixed value defined by the program.

---

## 14. Understanding Scope: Where Can We Use a Variable?

Declaring a variable does not mean that we can use its name everywhere in our program.

A variable binding has a scope.

**Scope is the part of the program where a particular variable binding is available to use.**

Consider:

```rust
fn main() {
    let cups_sold = 120;

    println!("{cups_sold}");
}
```

The variable is declared inside `main`.

We can use it within the part of the program where it is in scope.

Now consider another example:

```rust
fn main() {
    let cups_sold = 120;

    {
        let customer_name = "Mekdes";

        println!("{customer_name}");
        println!("{cups_sold}");
    }

    println!("{cups_sold}");

    // Not allowed:
    // println!("{customer_name}");
}
```

Notice that we have an inner block.

The inner block can access `customer_name`, which is declared inside it.

It can also access `cups_sold`, which is declared in the outer scope.

However, the outer block cannot access `customer_name` after the inner block ends.

Why?

Because `customer_name` belongs to the inner scope.

When that block ends, the binding goes out of scope.

For values that own resources, Rust also performs the appropriate cleanup when their owner goes out of scope.

For example, an owned `String` is normally dropped when its owner goes out of scope.

Remember:

**Giving a value a name does not mean we can access it everywhere. Its binding must still be in scope.**

---

## 15. Shadowing: Declaring Another Variable with the Same Name

Rust has another useful feature called shadowing.

Shadowing means declaring a new variable binding using the same name as an existing binding.

For example:

```rust
fn main() {
    let price = "25";

    let price = 25.0;

    println!("{price}");
}
```

This is allowed in Rust.

The second `let` creates a new binding named `price`. It shadows the previous binding.

Notice something important: we changed the type from text to a floating-point number.

This is different from reassignment.

### Reassignment

```rust
let mut price = 25.0;

price = 30.0;
```

Here, we are assigning a new value to the same mutable binding.

### Shadowing

```rust
let price = 25.0;

let price = 30.0;
```

Here, we are declaring a new binding with the same name.

The difference is:

```text
Reassignment:

The same binding receives another value.


Shadowing:

A new binding is declared using
the same name as the previous binding.
```

Shadowing is useful when we want to transform a value without introducing a completely different variable name.

---

## 16. Stack and Heap: Where Does the Data Live?

To understand Rust variables more deeply, we need to understand two important memory areas:

- The stack.
- The heap.

### The Stack

Think of the stack as an organized area of memory commonly used for function calls and their local data.

When a function is called, the program typically creates a stack frame containing information needed by that function.

For example:

```rust
fn main() {
    let cups_sold: u32 = 120;
    let price: f64 = 25.0;
}
```

Values with known sizes can commonly be stored in this area, although the compiler may use CPU registers or optimize away unnecessary storage.

When the function returns, its stack frame can be released efficiently.

### The Heap

Now imagine that your tea shop needs to store a list of customer names.

The number of customers may grow while your program is running.

We need memory that can be allocated dynamically.

This is where the heap becomes useful.

For example:

```rust
fn main() {
    let customer_names = vec![
        String::from("Abebe"),
        String::from("Mekdes"),
        String::from("Dawit"),
    ];
}
```

A `Vec` is a growable collection.

Its elements are stored in a heap allocation. The `Vec` value contains information that allows Rust to access that allocation.

### Stack vs. Heap

| Stack | Heap |
|---|---|
| Commonly holds function-related data and local values with known sizes. | Used for dynamically allocated data. |
| Organized around function calls and stack frames. | Memory is requested dynamically. |
| Allocation and release are generally efficient. | Allocation involves additional work. |
| A simple integer value is a useful mental example. | The element buffer of a `Vec` is a useful example. |

### An Important Example: `String`

Consider:

```rust
let customer_name = String::from("Abebe");
```

Do not imagine that the entire text must be stored directly inside the variable's local memory location.

A useful mental model is:

```text
              STACK

       +-------------------+
       | customer_name     |
       |-------------------|
       | Pointer           | --------+
       | Length            |         |
       | Capacity          |         |
       +-------------------+         |
                                     |
                                     v
                               HEAP MEMORY

                              +----------------+
                              | A b e b e      |
                              +----------------+
```

Conceptually, a `String` contains three important pieces of information:

1. A pointer that identifies where the text is stored.
2. A length that tells us how much text is currently being used.
3. A capacity that tells us how much space has been allocated.

The actual text is stored in heap memory.

This is why a variable and the data it gives us access to are not always the same physical piece of memory.

The variable contains the information Rust needs to work with the actual data.

---

## 17. Variables and Ownership: Who Owns the Value?

Now we reach one of Rust's most important concepts: ownership.

In Rust, variables are important because they help determine who owns a value.

Ownership is the system Rust uses to manage memory safely without requiring a garbage collector.

To understand this, let's compare two examples.

### Example 1: Copying a Simple Value

```rust
fn main() {
    let cups_sold = 120;

    let another_count = cups_sold;

    println!("{cups_sold}");
    println!("{another_count}");
}
```

Both variables can be used.

Why?

Because integer types such as `u32` implement a Rust feature called `Copy`.

When a value that implements `Copy` is assigned to another binding, Rust copies the value.

Conceptually:

```text
cups_sold      = 120

another_count  = 120
```

Each binding has its own independent value.

### Example 2: Moving an Owned Value

Now consider a `String`.

```rust
fn main() {
    let shop_name = String::from("My Tea Shop");

    let another_name = shop_name;

    println!("{another_name}");

    // Not allowed:
    // println!("{shop_name}");
}
```

Why can't we use `shop_name` after assigning it to `another_name`?

Because `String` does not implement `Copy`.

Instead, Rust moves ownership.

Let's understand what happens.

Remember the structure of a `String`:

```text
shop_name
    |
    v
+----------------------+
| Pointer              |
| Length               |
| Capacity             |
+----------------------+
    |
    v
  HEAP MEMORY

+----------------------+
| "My Tea Shop"        |
+----------------------+
```

The `String` value contains information that allows Rust to access the text stored in the heap.

Now consider:

```rust
let another_name = shop_name;
```

Rust transfers ownership to `another_name`.

Conceptually, the relationship changes:

```text
BEFORE THE MOVE

shop_name
    |
    v
"My Tea Shop"
```

After the move:

```text
AFTER THE MOVE

another_name
    |
    v
"My Tea Shop"
```

The original binding, `shop_name`, is no longer usable for that value.

Why does Rust do this?

Imagine that both bindings independently believed they owned the same heap allocation.

When the program finished using them, both might try to free the same memory.

That could cause serious memory problems.

Rust prevents this by enforcing ownership rules.

### The Important Difference

```text
For Copy types:

Assignment copies the value.

For non-Copy owned values:

Assignment usually moves ownership.
```

After a move, the previous binding cannot be used as though it still owns the value.

### What If We Need Two Independent Strings?

We can explicitly clone the value.

```rust
fn main() {
    let shop_name = String::from("My Tea Shop");

    let another_name = shop_name.clone();

    println!("{shop_name}");
    println!("{another_name}");
}
```

Here, `clone()` creates a separate copy of the string data.

Both variables can now be used independently.

---

## 18. What Happens When We Load Information from a Database?

Imagine that our tea shop stores customer information in a database.

Later, our program retrieves a customer's information.

The database returns something like this:

```text
Customer name: Mekdes
Payment method: Cash
```

Our program can work with those values using variables:

```rust
let customer_name = String::from("Mekdes");

let payment_method = String::from("cash");
```

Now the program can display the information, perform calculations, or make decisions.

However, we need to understand the difference between a variable and a database.

**A variable gives our running program a way to work with a value.**

**A database stores information persistently so it can remain available beyond the current program execution.**

For example, suppose we change:

```rust
let mut cups_sold = 120;

cups_sold = 150;
```

The new value exists in our running program.

But this does not automatically update the database.

We must explicitly perform a database operation to save the new information.

Similarly, when our program stops running, its ordinary local variable bindings do not automatically preserve their values for the next execution.

This distinction becomes important when building real applications.

---

## 19. The Golden Rule: How Do We Identify Variables?

When reading a problem or designing a system, do not immediately start writing Rust syntax.

First, understand what information the system needs.

Ask yourself:

**Does this piece of information need to be represented, accessed, updated, remembered, or used in a calculation?**

If yes, it may need a variable or another data structure.

Use these questions to guide your thinking:

| Question | What it helps us decide |
|---|---|
| What information does the program need? | Possible variables and data fields. |
| What does the information represent? | Meaningful variable names. |
| What kind of value is it? | Data type. |
| Does the binding need to change? | Mutability. |
| Where must we use it? | Scope and organization. |
| Do we need one value or multiple values? | A variable, collection, or structure. |
| Who owns the value? | Ownership and borrowing rules. |
| Must we remember it after the program stops? | Persistent storage, such as a database. |

For our tea shop, the information could be represented like this:

| Information | Possible Rust representation |
|---|---|
| Number of cups sold | `u32` |
| Selling price | `u64` for an amount in cents |
| Cost per cup | `u64` for an amount in cents |
| Customer name | `String` |
| Payment method | `PaymentMethod` enum |
| Daily profit | `u64` for an amount in cents |
| Daily sales history | `Vec<u32>` or an array |

This is how we move from understanding a real-world requirement to choosing Rust variables and data structures.

---

## 20. The Complete Rust Mental Model

Let's bring everything together.

Whenever we encounter information in a problem, we should think about what our program needs to do with it.

For example:

```rust
let mut cups_sold: u32 = 120;
```

Instead of seeing only `let`, `mut`, and a name, break the declaration down:

**1. Declaration**

Am I introducing a new variable binding?

```rust
let
```

**2. Name**

What name will I use to refer to this information?

```rust
cups_sold
```

**3. Type**

What kind of information does it represent?

```rust
u32
```

**4. Initial value**

What value does it have when it is created?

```rust
120
```

**5. Purpose**

Why does my program need this information?

In this example, we need the number of cups sold to calculate revenue and profit.

**6. Mutability**

Will my program need to assign another value to this binding?

```rust
mut
```

**7. Ownership**

Who owns the value? Can it be copied, or does assigning it elsewhere move ownership?

**8. Memory**

Does the value contain data stored elsewhere, such as in heap memory?

**9. Scope**

Where can this binding be accessed, and when does it stop being available?

These questions help us understand a variable beyond its syntax.

---

## 21. Quick Summary

| Concept | Understanding |
|---|---|
| Variable | Gives a value a name so our program can access and use it. |
| Name | Helps programmers understand what the information represents. |
| Value | The actual data currently associated with a binding. |
| Type | Defines the kind of value and the operations allowed on it. |
| `let` | Introduces a new variable binding. |
| `=` | Assigns or initializes a value. |
| `==` | Compares two values. |
| `mut` | Allows a binding to be reassigned. |
| Immutable | Cannot be reassigned after initialization. |
| Mutable | Can be reassigned. |
| Array | Holds multiple values of the same type with a fixed length. |
| Vector | Holds multiple values of the same type and can grow or shrink. |
| Struct | Organizes several related pieces of information. |
| Enum | Represents one choice from a defined set of possibilities. |
| Constant | A fixed, compile-time value. |
| Scope | Determines where a binding can be accessed. |
| Shadowing | Declares a new binding using an existing name. |
| Ownership | Determines who owns a value and manages its resources. |
| Copy | Allows certain values to be copied when assigned. |
| Move | Transfers ownership and makes the previous binding unusable for that value. |
| Stack | Memory commonly used for function-related data and local values. |
| Heap | Memory used for dynamically allocated data. |
| Database | Preserves information beyond the lifetime of a running program. |

## Final Idea

A variable is not just a line of Rust code containing `let` and a name.

It is a way for our program to work with information.

When we receive a problem, we first identify the information the system needs. Then we decide what each piece of information represents, what type of value it requires, whether it needs to change, and where it needs to be used.

Finally, we express those decisions using Rust variables, types, mutability, collections, structures, and ownership.

**The goal is to understand the information first and then translate that understanding into Rust code.**