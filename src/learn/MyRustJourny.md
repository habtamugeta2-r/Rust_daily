# Rust Variables — The Complete Reference

---

## The Core Mental Model

```
A Rust variable is NOT a box you stuff values into.
It's a NAME TAG you clip onto a value.

By default, that name tag is glued on. You can read it,
reference it, pass it around — but you cannot rip it off
and stick it on something else.

If you want to swap what the tag points to, you must
explicitly ask Rust for permission.

And here is the most important thing to internalize
before reading anything else:

    A variable holds ONLY the current value.
    It has no memory of what came before.
    It is a snapshot of the present, not a journal
    of the past.

This one design choice — safety by default — combined
with the "current value only" reality, is the heartbeat
of everything below.
```

    Variable
        ├── A name you use in source code.
        ├── Attached to the CURRENT value (and only that).
        ├── Has a type (what kind of value is allowed).
        ├── Governed by rules: mutability, scope, ownership.
        ├── Holds no history of previous values.
        └── Not necessarily a fixed memory slot.
            └── The compiler may store the value in a
                register, on the stack, on the heap,
                or optimize it away entirely.

---

## 0. Why Variables Exist in the First Place

Before we touch syntax, let's ground this in reality.

    Imagine you own a tea shop.
    Every day you sell a different number of cups.

        Monday:    120 cups
        Tuesday:   150 cups
        Wednesday:  95 cups

    Your program needs to calculate revenue, cost, and profit.
    To do that, it must remember and refer to things like:

        ├── Number of cups sold
        ├── Selling price per cup
        ├── Cost per cup
        ├── Customer name
        ├── Payment method
        └── Daily profit

    Without variables, you'd write the raw number `120`
    everywhere it appears. One price change and you're
    hunting through your entire codebase.

    With variables, you write it once, name it clearly,
    and let the name do the heavy lifting.

```rust
let cups_sold = 120;
let selling_price = 25;
let cost_per_cup = 10;

let daily_profit =
    cups_sold * selling_price - cups_sold * cost_per_cup;
```

    The mental shift:
        A variable is how your program REMEMBERS and
        REFERS TO information while it runs.

### Do variables have to change?

    No. The word "variable" is misleading.

    Many variables are set once and never touched again.
    They still earn their keep by giving a value a name.

        let selling_price = 25;   // never changes, still useful

    Others genuinely need to update as the program runs.

        let mut cups_sold = 120;  // will grow as customers buy

    Rust handles both cases cleanly — which is where
    immutability and mutability come in.

---

## 1. Declaring a Variable with `let`

    Syntax:
        let name = value;

    Example:
        let age = 25;

    What each piece does:
        let     → "Rust, I'm creating a new binding."
        age     → The name tag.
        =       → "Attach this name to the following value."
        25      → The actual data.
        ;       → End of statement.

    In practice:
        let age = 25;
        println!("My age is {age}");
        // Output: My age is 25

    Mental model:
        You wrote "age = 25" on a sticky note and placed
        it on your desk. From now on, whenever you say
        "age," everyone knows you mean 25.

### `=` is assignment, not equality

    This trips up people coming from math class:

        =   → "Attach this value to this name."
        ==  → "Compare these two values. Give me true/false."

    Example:
        let cups_sold = 120;              // assignment
        let is_target = cups_sold == 120; // comparison → true

    Keep them straight. The compiler will catch misuse,
    but your brain shouldn't need the compiler to remind it.

---

## 2. Default Immutability (The Glued-On Name Tag)

    Rule:
        Every variable declared with `let` is immutable
        by default. You cannot reassign it. Period.

    What this means:
        let age = 25;
        age = 26;   // ✗ COMPILE ERROR

        Rust says: "cannot assign twice to immutable variable"

    What you CAN still do:
        Read the value, use it in math, pass it to functions.

        let price = 100;
        let tax = 15;
        let total = price + tax;   // ✓ perfectly fine
        println!("{total}");       // 115

        Immutability blocks reassignment, not usage.

    Why Rust does this:
        Most real-world bugs come from things changing
        when you didn't expect them to. Rust flips the
        script: nothing changes unless you consciously
        opt in.

    Mental model:
        You wrote someone's name on a whiteboard in
        permanent marker. You can read it. You can
        point at it. But you cannot erase it and write
        something else.

    Key nuance:
        "Immutable variable" is not an oxymoron.
        The binding is alive and useful. It is the
        REASSIGNMENT that is forbidden — not the
        existence of the variable.

---

## 3. Mutability with `mut` (Asking for Permission)

    Rule:
        Adding `mut` after `let` makes the binding
        mutable. You may now reassign it.

    Syntax:
        let mut score = 0;
        score = 10;           // ✓
        score = score + 5;    // ✓ score is now 15

    Mental model:
        Same whiteboard, but now you are using a
        dry-erase marker. You can erase and rewrite.

    Hard rules:
        ├── The new value must be the SAME TYPE.
        │       let mut name = "Alice";
        │       name = 42;    // ✗ expected &str, found integer
        │
        ├── `mut` controls reassignment, not type.
        │       mut  → "Can I swap the value?"
        │       type → "What kind of value is allowed?"
        │       These are two independent rules.
        │
        └── `mut` does not guarantee a new memory address.
                The compiler may reuse the same slot
                or optimize the update however it likes.

    Practical example — a counter:

        fn main() {
            let mut counter = 0;
            counter += 1;
            counter += 1;
            counter += 1;
            println!("Counter: {counter}");  // 3
        }

    Compound assignment operators (handy shortcuts):

        let mut n = 10;
        n += 5;   // add and assign
        n -= 2;   // subtract and assign
        n *= 2;   // multiply and assign
        n /= 3;   // divide and assign
        n %= 4;   // remainder and assign

    When to use `mut`:
        ├── Counters and accumulators.
        ├── Loop variables.
        ├── Buffers being filled.
        └── Anything that genuinely transitions
            through multiple states in place.

    When NOT to use `mut`:
        If you only need a "different version" of
        something once or twice, consider shadowing
        instead (Section 5). Rust will even warn you:

        warning: variable does not need to be mutable

        That's the compiler saying: "You asked for a
        dry-erase marker but never erased anything."

---

## 4. The Most Important Thing: A Variable Holds ONLY the Current Value

    This deserves its own section because it is the
    single most misunderstood thing about variables.

    A variable stores the current value and NOTHING ELSE.
    It has no memory of what came before.
    It keeps no history.
    There is no undo.
    There is no log.

    When you reassign a mutable variable, the old value
    is replaced. From the variable's perspective, the
    old value never existed.

### Watch it happen

```rust
let mut cups_sold = 120;
// cups_sold is 120. That is the entire truth.

cups_sold = 150;
// cups_sold is 150. The 120 is GONE from this variable.
// It doesn't linger. It doesn't hide in a corner.
// The variable holds one value: 150.

cups_sold = 95;
// cups_sold is 95. Both 120 and 150 are forgotten
// as far as this variable is concerned.
```

    Mental model:
        A whiteboard with a single number written on it.
        When you erase 120 and write 150, the 120 does
        not remain in faint ghost letters. The board
        shows one number. Always one number.

### Visualizing the timeline

```
    Time ──────────────────────────────────────►

    let mut cups_sold = 120;
         ┌─────────┐
         │   120   │  ← current value
         └─────────┘

    cups_sold = 150;
         ┌─────────┐
         │   150   │  ← current value (120 is gone)
         └─────────┘

    cups_sold = 95;
         ┌─────────┐
         │    95   │  ← current value (120 and 150 gone)
         └─────────┘
```

    At every point in time, the variable holds exactly
    one value. The previous values are not stored
    anywhere by the variable itself.

### So what if you need to remember the old values?

    That is when you stop using a single variable and
    reach for a different tool entirely.

    Option 1 — A collection (array or vector):

```rust
// One variable — only remembers the latest
let mut cups_sold = 120;
cups_sold = 150;
cups_sold = 95;
// cups_sold is 95. Monday and Tuesday are forgotten.

// A collection — remembers ALL of them
let daily_sales = [120, 150, 95];
// Now you have Monday, Tuesday, AND Wednesday.
// daily_sales[0] is 120
// daily_sales[1] is 150
// daily_sales[2] is 95
```

    Option 2 — Separate named variables:

```rust
let monday_sales = 120;
let tuesday_sales = 150;
let wednesday_sales = 95;
// Each variable holds one value, but together
// they preserve the full picture.
```

    Option 3 — A growable vector for an unknown number of days:

```rust
let mut daily_sales = vec![120, 150];
daily_sales.push(95);
daily_sales.push(180);
// daily_sales now holds [120, 150, 95, 180]
// Every value is preserved in order.
```

### The practical hierarchy of memory

```
┌──────────────────┬──────────────────────────────────────┐
│ Tool             │ What it remembers                    │
├──────────────────┼──────────────────────────────────────┤
│ Single variable  │ The value RIGHT NOW. Nothing else.   │
│ Array / Vector   │ ALL values you explicitly stored.    │
│ Struct           │ Multiple related current values.     │
│ Database / File  │ Values that survive program restart. │
└──────────────────┴──────────────────────────────────────┘
```

    This is not a Rust-specific idea. It is true in every
    programming language. But it is worth stating loudly
    because it shapes how you design your programs.

    If your tea shop needs to show a weekly sales chart,
    a single `cups_sold` variable will not cut it. You
    need a collection. The variable gives you the present.
    The collection gives you the past.

### A common mistake this prevents

    Beginners sometimes write code like this and wonder
    why they lost data:

```rust
let mut total = 0;

total = 120;   // Monday
total = 150;   // Tuesday — wait, where did Monday go?
total = 95;    // Wednesday — now Tuesday is gone too

println!("{total}");  // 95. Only Wednesday survived.
```

    The fix:

```rust
let mut total = 0;

total += 120;   // Monday:   total is now 120
total += 150;   // Tuesday:  total is now 270
total += 95;    // Wednesday: total is now 365

println!("{total}");  // 365. The running sum is preserved.
```

    Notice the difference. We used `+=` instead of `=`.
    We did not replace the value — we accumulated into it.
    The variable still holds only the current value (365),
    but that current value now reflects all three days.

    If you need the individual daily numbers later, you
    still need a collection. The running total only tells
    you the sum, not the breakdown.

---

## 5. Shadowing (The Sticky-Note Stack)

    Rule:
        You can declare a NEW variable with the SAME
        name by writing `let` again. The old binding
        is eclipsed — still there underneath, but
        invisible from this point forward.

    Syntax:
        let x = 5;
        let x = x + 1;    // x is now 6
        let x = x * 2;    // x is now 12

    Mental model:
        You write "x = 5" on a sticky note. Then you
        place a new note on top: "x = 6." The original
        is still physically there. You just can't see
        it anymore. You always read the top note.

    Important connection to "current value only":
        Even though shadowing creates multiple bindings
        under the hood, from YOUR perspective as the
        programmer, the name `x` refers to one value —
        the latest one. The shadowed values are not
        accessible through that name anymore.

        So shadowing does NOT give you history either.
        It gives you a clean name for the latest stage
        of a transformation.

    The superpower — type changing:
        Unlike `mut`, shadowing lets you change the type
        entirely because each `let` creates a fresh binding.

        let spaces = "     ";        // &str
        let spaces = spaces.len();   // usize ← totally fine!

        With `mut` this would fail:
        let mut spaces = "     ";
        spaces = spaces.len();       // ✗ type mismatch

    Shadowing does NOT require `mut`:
        let name = "Habtish";
        let name = "Developer";      // ✓ both immutable

    Scoped shadowing:
        let x = 5;
        {
            let x = x * 2;
            println!("{x}");         // 10
        }
        println!("{x}");             // 5 — outer x untouched

        The inner shadow only lives inside its block.
        Once the block ends, the outer binding reappears.

    Real-world use — parsing input:
        let price = "100";                        // &str from user
        let price = price.parse::<i32>().unwrap(); // now i32
        let price = price + 50;                    // transformed i32
        // Same concept flowing through stages,
        // one clean name: `price`.

    When to use shadowing:
        ├── Transforming a value through a pipeline.
        ├── Parsing: string → number (same concept,
        │   different type).
        └── Keeping names clean instead of inventing
            input_str, input_num, input_final, etc.

    When to be careful:
        Shadowing the same name 6 times in 20 lines
        makes code hard to follow. Use judgment.

### Shadowing vs. `mut` — the decision guide

```
┌──────────────────────────────┬─────────────┬─────────────┐
│ Scenario                     │  Use mut    │  Use shadow │
├──────────────────────────────┼─────────────┼─────────────┤
│ Same type, updating in place │     ✓       │             │
│ Type needs to change         │             │     ✓       │
│ Loop counter                 │     ✓       │             │
│ Parse string → number        │             │     ✓       │
│ Value transforms once        │             │     ✓       │
│ Value changes many times     │     ✓       │             │
│ Want to keep immutability    │             │     ✓       │
└──────────────────────────────┴─────────────┴─────────────┘
```

---

## 6. Constants (Carved in Stone)

    Rule:
        Constants are values baked into your program at
        compile time. They are not merely immutable —
        they are eternal within their scope.

    Syntax:
        const MAX_PLAYERS: u32 = 100;
        const TAX_RATE: f64 = 0.07;
        const APP_NAME: &str = "RustNotes";

    Mental model:
        An immutable `let` is permanent marker on a
        whiteboard. A `const` is carved into stone and
        bolted to the wall. It was decided before the
        program ever ran.

    Hard rules:
        ├── MUST use `const`, not `let`.
        ├── MUST annotate the type explicitly. Always.
        │       const MAX: u32 = 100;   // ✓
        │       const MAX = 100;        // ✗ missing type
        │
        ├── MUST be a compile-time constant expression.
        │       const SEC_PER_HOUR: u32 = 60 * 60;  // ✓
        │       const NOW: u64 = get_time();         // ✗ runtime
        │
        ├── CANNOT use `mut`. Ever.
        │       const mut X: i32 = 5;   // ✗ nonsensical
        │
        └── Naming convention: SCREAMING_SNAKE_CASE.
                Not enforced by the compiler, but violating
                it triggers a warning and earns sideways
                glances from other Rust developers.

    Where you can declare them:
        ├── Inside functions (scoped to that function).
        ├── Outside functions (available to the whole module).
        └── Anywhere items are allowed, really.

    How the compiler treats them:
        Constants are inlined wherever they are used.
        There is no memory address, no runtime lookup.
        The compiler effectively copy-pastes the value.

### `const` vs. `let` — at a glance

```
┌──────────────────────────────┬──────────────────┬──────────────────┐
│ Property                     │  let x = 5       │  const X: u32=5  │
├──────────────────────────────┼──────────────────┼──────────────────┤
│ Type annotation required?    │  No (inferred)   │  Yes (always)    │
│ Can shadow?                  │  Yes             │  No              │
│ Computed at runtime?         │  Yes             │  No              │
│ Valid at module level?       │  No              │  Yes             │
│ Has a memory address?        │  Yes             │  No (inlined)    │
│ Can use `mut`?               │  Yes             │  No              │
│ Value can change?            │  Only with mut   │  Never           │
└──────────────────────────────┴──────────────────┴──────────────────┘
```

    Bonus — `static` (constant's cousin):
        If you need a value with a fixed memory address
        that lives for the entire program (e.g., for FFI
        or global shared state):

        static GREETING: &str = "Hello";

        `static` → real home in memory, fixed address.
        `const`  → copy-pasted everywhere it is used.

        For 95% of cases, prefer `const`.

---

## 7. Type Inference (The Compiler Reads Your Mind)

    Rule:
        Rust figures out a variable's type from the
        value you assign, how you use it later, or both.
        You usually do not need to write the type.

    Mental model:
        You hand someone an apple and say "hold this."
        They don't need you to say "hold this Apple™
        brand fruit, genus Malus." They can see it's
        an apple. Rust's compiler works the same way.

    Basic inference:
        let x = 5;          // i32  (default integer)
        let y = 3.14;       // f64  (default float)
        let active = true;  // bool
        let name = "Alice"; // &str
        let letter = 'z';   // char

    Default types when no other context exists:
        ├── Whole numbers   → i32
        └── Decimal numbers → f64
            These are Rust's sensible defaults: fast on
            most hardware, wide enough for most tasks.

    Inference from context (the clever part):
        Rust looks forward in your code to resolve types.

        let mut numbers = Vec::new();  // what type? unknown yet
        numbers.push(5u8);             // AH — Vec<u8>

    When you MUST annotate explicitly:
        ├── The compiler genuinely cannot decide:
        │       let guess = "42".parse().unwrap();
        │       // parse into WHAT? i32? u64? f64?
        │
        │       let guess: u32 = "42".parse().unwrap();
        │       // Now it knows.
        │
        ├── Constants (always — see Section 6).
        ├── Function signatures (always):
        │       fn add(a: i32, b: i32) -> i32 { a + b }
        │
        └── You want a non-default numeric type:
                let small: i8 = 5;
                let precise: f32 = 3.0;

    Turbofish syntax (annotate the call, not the variable):
        let guess = "42".parse::<u32>().unwrap();
                                ^^^^^^
                           this is the "turbofish"

        Equivalent to annotating the variable. Use
        whichever reads better in context.

    Critical distinction — inference ≠ dynamic typing:
        ├── Python: type decided at runtime, can change.
        └── Rust:   type decided at compile time, locked.
                Rust just saves you from writing what
                it already knows. Zero runtime cost.

### The numeric types you'll meet

    Integers (whole numbers):
        i8, i16, i32, i64, i128, isize   ← signed (±)
        u8, u16, u32, u64, u128, usize   ← unsigned (0 or +)

    Floats (decimals):
        f32, f64

    You don't need to memorize all of these today.
    Default to i32 and f64. Pick others when you need
    a specific range (u8 for a byte, u64 for big counts,
    i128 for enormous signed math, etc.).

---

## 8. Memory — How Variables Actually Live in Hardware

    This section is the deep dive. Everything above told
    you what a variable IS conceptually. This section
    tells you WHERE it lives physically and how the bytes
    are actually organized at every level.

    If you understand memory, you understand why Rust's
    rules exist. If you don't, the rules feel arbitrary.

### 8.1. The Memory Hierarchy — From Registers to Disk

    Your computer has multiple layers of storage.
    Each layer trades speed for capacity:

```
┌────────────────────────────────────────────────────────────────┐
│ Layer          │ Speed       │ Size        │ Managed by        │
├────────────────┼─────────────┼─────────────┼───────────────────┤
│ CPU Registers  │ ~0.3 ns     │ ~dozens     │ Compiler / CPU    │
│ L1 Cache       │ ~1 ns       │ ~64 KB      │ CPU               │
│ L2 Cache       │ ~4 ns       │ ~256 KB     │ CPU               │
│ L3 Cache       │ ~12 ns      │ ~8-32 MB    │ CPU               │
│ RAM (Stack)    │ ~100 ns     │ ~8 MB*      │ Compiler / OS     │
│ RAM (Heap)     │ ~100 ns     │ ~GBs        │ Allocator         │
│ SSD / Disk     │ ~100,000 ns │ ~TBs        │ OS / Filesystem   │
│ Network        │ ~millions   │ unlimited   │ Protocols         │
└────────────────┴─────────────┴─────────────┴───────────────────┘
  * Default thread stack size; configurable.
```

    When you write `let x = 42;`, you do NOT choose
    which layer stores it. The compiler does. Your job
    is to understand the IMPLICATIONS of where data ends
    up, not to micromanage placement.

    Mental model:
        Registers are your hands — the fastest place
        to hold something, but you only have a few.

        The stack is your desk — fast to reach,
        organized, but limited in space.

        The heap is the warehouse out back — virtually
        unlimited, but you need to walk there and back,
        and you have to remember where you put things.

        Disk is the filing cabinet in the basement —
        survives power loss, but agonizingly slow.

### 8.2. Registers — Where the Fastest Variables Live

    What is a register?
        A register is a tiny, named storage slot built
        directly into the CPU. There are typically
        16 general-purpose registers on x86-64
        (rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp,
        r8 through r15), each 64 bits wide.

    When does a variable live in a register?
        The compiler's optimizer (LLVM in Rust's case)
        decides. Small, short-lived, frequently-used
        values are prime candidates.

```rust
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;    // `sum` will almost certainly
    sum                 // live in a register and never
}                       // touch RAM at all.
```

    Why this matters to you:
        ├── You cannot take a reference (&) to a register.
        │   If you write `&sum`, the compiler must "spill"
        │   the value to the stack so it has an address.
        │
        ├── Register allocation is the single biggest
        │   factor in low-level performance.
        │
        └── Rust's ownership model helps the compiler
            here: since only one owner exists at a time,
            the optimizer can keep values in registers
            longer without worrying about aliasing.

    Register spilling:
        When the compiler runs out of registers (because
        you have too many live variables at once), it
        "spills" the least-needed values to the stack.
        This is automatic and invisible in your source
        code, but it costs performance.

```
    Scenario: 20 variables alive at the same time
    on a CPU with 16 general-purpose registers.

    Compiler decides:
        ├── 14 variables stay in registers (fast).
        ├── 2 variables are computed from others
        │   and never stored at all (optimized away).
        └── 4 variables spill to the stack (slower).
```

    You cannot control this directly in safe Rust.
    But you can influence it:
        ├── Smaller scopes → fewer live variables → less spilling.
        ├── Fewer `&` references → less forced spilling.
        └── Simple types → more likely to stay in registers.

### 8.3. The Stack — Organized, Fast, Automatic

    What is the stack?
        A contiguous block of memory allocated per thread.
        It grows and shrinks in a strictly last-in-first-out
        (LIFO) order. Every function call pushes a "frame."
        Every return pops it.

    What lives on the stack?
        ├── Local variables with known, fixed sizes.
        │       let x: i32 = 42;        // 4 bytes on the stack
        │       let point: (f64, f64) = (1.0, 2.0);  // 16 bytes
        │       let arr: [u8; 256] = [0; 256];        // 256 bytes
        │
        ├── Function arguments (if not passed in registers).
        ├── Return addresses.
        └── Spilled registers.

    How a stack frame looks in memory:

```
    High addresses
    ┌──────────────────────────────────────┐
    │         main() frame                  │
    │  ┌──────────────────────────────┐    │
    │  │ cups_sold: i32     [4 bytes] │    │
    │  │ price: i32         [4 bytes] │    │
    │  │ total: i32         [4 bytes] │    │
    │  └──────────────────────────────┘    │
    │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─    │
    │  │ return address     [8 bytes] │    │
    ├──────────────────────────────────────┤
    │       calculate() frame              │
    │  ┌──────────────────────────────┐    │
    │  │ revenue: i64       [8 bytes] │    │
    │  │ cost: i64          [8 bytes] │    │
    │  └──────────────────────────────┘    │
    │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─    │
    │  │ return address     [8 bytes] │    │
    └──────────────────────────────────────┘
    Low addresses  ← stack pointer (rsp)
```

    Why the stack is fast:
        ├── Allocation = move the stack pointer.
        │   One CPU instruction. No searching.
        │
        ├── Deallocation = move the pointer back.
        │   One instruction. No bookkeeping.
        │
        ├── Memory is contiguous → cache-friendly.
        │   L1 cache hits almost every time.
        │
        └── No fragmentation. Ever.

    The stack's limitation:
        ├── Fixed maximum size (typically 8 MB per thread).
        │   Exceed it → stack overflow.
        │
        ├── LIFO only. You cannot free something in
        │   the middle of the stack.
        │
        └── Size must be known at compile time.
            You cannot put a `Vec<T>` (whose contents
            grow at runtime) entirely on the stack.
            The Vec header (pointer, length, capacity)
            goes on the stack. The actual elements go
            on the heap.

    Stack overflow — what actually happens:

```rust
fn infinite_recursion() {
    infinite_recursion();  // each call adds a frame
}
// Eventually: thread 'main' has overflowed its stack
```

        Each call pushes a frame (~dozens to hundreds of
        bytes). After ~100,000+ calls (depends on frame
        size), you exhaust 8 MB and the OS kills the thread.

    What Rust puts on the stack automatically:
        ├── All primitive types (i32, f64, bool, char).
        ├── Tuples of stack types: (i32, f64).
        ├── Fixed-size arrays: [u8; 1024].
        ├── Structs where all fields are stack types:
        │       struct Point { x: f64, y: f64 }
        │       // 16 bytes, entirely on the stack.
        │
        └── Enum variants (sized to the largest variant).

### 8.4. The Heap — Flexible, Unlimited, Costs You

    What is the heap?
        A large pool of memory managed by an allocator
        (Rust uses the system allocator by default, which
        is typically `malloc`/`free` from the C library,
        or `jemalloc` on some platforms).

    When does data go on the heap?
        ├── Size unknown at compile time.
        │       let input = String::from("hello");
        │       // "hello" could be any length at runtime.
        │
        ├── Size too large for the stack.
        │       let big = vec![0u8; 10_000_000];
        │       // 10 MB — wouldn't fit on the 8 MB stack.
        │
        └── Data needs to outlive the current stack frame.
                let boxed = Box::new(42);
                // The 42 lives on the heap, survives being
                // returned from a function.

    How heap allocation works at a low level:

```
    Your code:  let s = String::from("Rust");

    What happens:
    1. The allocator searches its free list for a
       contiguous block ≥ 4 bytes (length of "Rust").
    2. It marks that block as used.
    3. It returns a pointer to the start of the block.
    4. The String struct is created on the STACK with:
           ├── ptr: *mut u8  → points to heap block
           ├── len: usize    → 4 (current length)
           └── cap: usize    → ≥ 4 (allocated capacity)
    5. The bytes [82, 117, 115, 116] ('R','u','s','t')
       are written to the heap block.
```

    Visual layout of a String:

```
    Stack                          Heap
    ┌──────────────────────┐       ┌───┬───┬───┬───┐
    │ ptr  ─────────────────────►  │ R │ u │ s │ t │
    │ len  = 4             │       └───┴───┴───┴───┘
    │ cap  = 4             │       address: 0x7f3a...
    └──────────────────────┘
    (24 bytes on stack:            (4 bytes on heap)
     3 × 8 bytes for
     pointer, length, capacity
     on 64-bit system)
```

    Why the heap is slower:
        ├── Allocation requires searching free lists.
        │   Best case: O(1). Worst case: O(n) with
        │   fragmentation.
        │
        ├── Deallocation requires updating bookkeeping.
        │
        ├── Memory is scattered → cache misses likely.
        │   Following a pointer to the heap is a cache
        │   miss that costs ~100 ns instead of ~1 ns.
        │
        └── Fragmentation over time: holes between
            allocations waste space and slow searches.

    Heap fragmentation — visual:

```
    After many alloc/dealloc cycles:

    ┌────┬──────┬────┬──────────┬────┬─────────────┐
    │USED│ free │USED│   free   │USED│    free      │
    │ 64B│ 32B  │128B│   16B    │ 64B│   2048B      │
    └────┴──────┴────┴──────────┴────┴─────────────┘

    You need 48 bytes. The 32B hole is too small.
    The 16B hole is too small. You use the 2048B hole —
    wasting 2000 bytes of potential. The small holes
    remain unusable.
```

    Rust's heap-allocated types:
        ├── String        → UTF-8 text, growable.
        ├── Vec<T>        → growable array of T.
        ├── Box<T>        → single value, heap-allocated.
        ├── HashMap<K,V>  → key-value store.
        ├── Rc<T>         → reference-counted shared ownership.
        ├── Arc<T>        → atomic reference-counted (thread-safe).
        └── Any type you explicitly Box::new().

### 8.5. Stack vs. Heap — The Complete Comparison

```
┌─────────────────────────┬──────────────────────┬──────────────────────┐
│ Property                │ Stack                │ Heap                 │
├─────────────────────────┼──────────────────────┼──────────────────────┤
│ Allocation speed        │ 1 instruction        │ 100s of instructions │
│ Deallocation speed      │ 1 instruction        │ 100s of instructions │
│ Size limit              │ ~8 MB (per thread)   │ Available RAM        │
│ Fragmentation           │ Never                │ Common               │
│ Layout                  │ Contiguous, ordered   │ Scattered            │
│ Cache behavior          │ Excellent (L1 hits)  │ Poor (cache misses)  │
│ Size known at           │ Compile time         │ Runtime              │
│ Lifetime                │ Until scope ends     │ Until owner drops    │
│ Thread safety           │ Local to thread      │ Shared if you choose │
│ Requires allocator?     │ No                   │ Yes                  │
│ Overflow risk           │ Stack overflow       │ Out of memory (OOM)  │
│ Access pattern          │ LIFO only            │ Random access        │
│ Rust management         │ Automatic (scopes)   │ Automatic (Drop)     │
└─────────────────────────┴──────────────────────┴──────────────────────┘
```

### 8.6. Memory Layout of Common Types

    Understanding how many bytes each type uses and where
    those bytes live is essential for systems programming.

    Primitive types — size and alignment:

```
┌──────────┬──────────┬───────────┬────────────────────────┐
│ Type     │ Size     │ Alignment │ Range                  │
├──────────┼──────────┼───────────┼────────────────────────┤
│ bool     │ 1 byte   │ 1         │ true / false           │
│ u8       │ 1 byte   │ 1         │ 0 to 255              │
│ i8       │ 1 byte   │ 1         │ -128 to 127           │
│ u16      │ 2 bytes  │ 2         │ 0 to 65,535           │
│ i16      │ 2 bytes  │ 2         │ -32,768 to 32,767     │
│ u32      │ 4 bytes  │ 4         │ 0 to 4,294,967,295    │
│ i32      │ 4 bytes  │ 4         │ -2^31 to 2^31-1       │
│ u64      │ 8 bytes  │ 8         │ 0 to 2^64-1           │
│ i64      │ 8 bytes  │ 8         │ -2^63 to 2^63-1       │
│ u128     │ 16 bytes │ 16        │ 0 to 2^128-1          │
│ i128     │ 16 bytes │ 16        │ -2^127 to 2^127-1     │
│ f32      │ 4 bytes  │ 4         │ IEEE 754 single       │
│ f64      │ 8 bytes  │ 8         │ IEEE 754 double       │
│ char     │ 4 bytes  │ 4         │ Unicode scalar value  │
│ usize    │ 8 bytes* │ 8*        │ 0 to 2^64-1 (on 64b) │
│ isize    │ 8 bytes* │ 8*        │ -2^63 to 2^63-1       │
│ &T       │ 8 bytes* │ 8*        │ pointer (address)     │
│ &[T]     │ 16 bytes*│ 8*        │ ptr + length          │
│ &str     │ 16 bytes*│ 8*        │ ptr + length          │
└──────────┴──────────┴───────────┴────────────────────────┘
  * On 64-bit systems. Half on 32-bit.
```

    Compound types — how Rust lays them out:

```rust
struct Example {
    a: u8,     // 1 byte
    b: u64,    // 8 bytes
    c: u8,     // 1 byte
}
```

    Naive layout (C-style, with `#[repr(C)]`):

```
    Offset:  0    1    2    3    4    5    6    7    8    ...15   16
            ┌────┬────┬────┬────┬────┬────┬────┬────┬────────────┬────┐
            │ a  │pad │pad │pad │pad │pad │pad │pad │     b      │ c  │
            └────┴────┴────┴────┴────┴────┴────┴────┴────────────┴────┘
    Total:   24 bytes (with 7 bytes of padding after a,
             plus padding after c to reach alignment of 8)
```

    Rust's default layout (compiler reorders fields):

```
    The compiler may reorder to:  b, a, c

    Offset:  0         ...7     8    9    10
            ┌─────────────────┬────┬────┬─────┐
            │       b         │ a  │ c  │ pad │
            └─────────────────┴────┴────┴─────┘
    Total:   16 bytes (less padding, same data)
```

    Why alignment matters:
        ├── CPUs read memory in aligned chunks (4 or 8 bytes).
        ├── An unaligned u64 may require two memory reads
        │   instead of one, halving throughput.
        ├── Some architectures FAULT on unaligned access.
        └── Rust's default `repr(Rust)` reorders fields to
            minimize padding. Use `#[repr(C)]` when you need
            exact C-compatible layout (FFI, serialization).

    You can check sizes at compile time:

```rust
use std::mem;
println!("i32:    {} bytes", mem::size_of::<i32>());     // 4
println!("String: {} bytes", mem::size_of::<String>());  // 24
println!("&str:   {} bytes", mem::size_of::<&str>());    // 16
println!("bool:   {} bytes", mem::size_of::<bool>());    // 1
println!("[u8;100]: {} bytes", mem::size_of::<[u8; 100]>());  // 100
```

### 8.7. How Vec Grows — Allocation Strategy

    A `Vec<T>` is three stack values: pointer, length, capacity.
    The actual elements live on the heap.

```rust
let mut v: Vec<i32> = Vec::new();
// Stack: ptr=dangling, len=0, cap=0
// Heap: nothing allocated

v.push(1);
// Allocator gives us room for 4 i32s (16 bytes).
// Stack: ptr=0x..., len=1, cap=4
// Heap: [1, _, _, _]

v.push(2); v.push(3); v.push(4);
// Stack: ptr=0x..., len=4, cap=4
// Heap: [1, 2, 3, 4]   — full!

v.push(5);
// Capacity exceeded! Vec must grow:
// 1. Allocate new block for 8 i32s (capacity doubles).
// 2. Copy [1,2,3,4] from old block to new block.
// 3. Write 5 into position 4 of new block.
// 4. Deallocate old block.
// Stack: ptr=NEW_ADDR, len=5, cap=8
// Heap: [1, 2, 3, 4, 5, _, _, _]
```

    Growth visualization over time:

```
    push #  │ len │ cap │ reallocation?
    ────────┼─────┼─────┼────────────────────
      1     │  1  │  4  │ YES (first alloc)
      2     │  2  │  4  │ no
      3     │  3  │  4  │ no
      4     │  4  │  4  │ no
      5     │  5  │  8  │ YES (copy 4 items)
      6     │  6  │  8  │ no
      7     │  7  │  8  │ no
      8     │  8  │  8  │ no
      9     │  9  │ 16  │ YES (copy 8 items)
     ...    │     │     │
    1024    │1024 │1024 │ no
    1025    │1025 │2048 │ YES (copy 1024 items)
```

    Why this matters:
        ├── Each reallocation copies ALL existing elements.
        │   Pushing N items costs O(N) total (amortized O(1)
        │   per push), but individual pushes can be expensive.
        │
        ├── Use `Vec::with_capacity(n)` when you know the
        │   approximate size in advance:
        │       let mut v = Vec::with_capacity(10_000);
        │       // One allocation upfront, no reallocations
        │       // until you exceed 10,000 elements.
        │
        └── After reallocation, the old heap memory is freed.
            Any raw pointers or references to the old data
            are now DANGLING. This is exactly why Rust's
            borrow checker prevents holding a reference
            across a push:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // borrow the first element
v.push(4);            // ✗ COMPILE ERROR
// Why? push might reallocate, making `first` a
// dangling pointer. Rust prevents this at compile time.
// In C++, this would compile and maybe crash at runtime.
```

### 8.8. Box, Rc, Arc — Heap Allocation Patterns

    Box<T> — single owner on the heap:

```rust
let b = Box::new(42);
// Stack: b is a pointer (8 bytes on 64-bit).
// Heap: the value 42 sits in a 4-byte allocation.
// When b goes out of scope, the heap memory is freed.
```

```
    Stack              Heap
    ┌─────────┐        ┌──────┐
    │  ptr ────────────►│  42  │
    └─────────┘        └──────┘
    (8 bytes)          (4 bytes + allocator overhead)
```

    When to use Box:
        ├── Recursive types (a type that contains itself):
        │       enum List {
        │           Cons(i32, Box<List>),
        │           Nil,
        │       }
        │       // Without Box, List has infinite size.
        │       // Box makes it a fixed-size pointer.
        │
        ├── Large data you don't want to copy on the stack.
        │       let big = Box::new([0u8; 1_000_000]);
        │       // 1 MB on the heap, 8-byte pointer on stack.
        │
        └── Trait objects (dynamic dispatch):
                let drawable: Box<dyn Draw> = Box::new(Circle);

    Rc<T> — reference-counted, shared ownership, single-thread:

```rust
use std::rc::Rc;
let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);   // cheap: increments count, no copy
let c = Rc::clone(&a);
// Reference count is now 3.
// The String is freed when the last Rc is dropped.
```

```
    Stack                    Heap
    a ──► ┐                 ┌──────────────────┐
    b ──► ┤ (all point to)  │ ref_count: 3     │
    c ──► ┘                 │ data: "shared"   │
                            └──────────────────┘
```

    Arc<T> — atomic reference-counted, thread-safe:

```rust
use std::sync::Arc;
let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);
// Can safely send data_clone to another thread.
// Atomic operations make the ref count thread-safe,
// but slightly slower than Rc.
```

    The cost hierarchy:

```
┌───────────────┬─────────────────────┬──────────────────────┐
│ Type          │ Allocation cost     │ Clone cost           │
├───────────────┼─────────────────────┼──────────────────────┤
│ Stack value   │ Free (stack pointer)│ memcpy (cheap)       │
│ Box<T>        │ 1 heap alloc        │ 1 heap alloc + copy  │
│ Rc<T>         │ 1 heap alloc        │ increment counter    │
│ Arc<T>        │ 1 heap alloc        │ atomic increment     │
│ String/Vec    │ 1 heap alloc        │ 1 heap alloc + copy  │
└───────────────┴─────────────────────┴──────────────────────┘
```

### 8.9. The Drop Order and Destructors

    When a variable goes out of scope, Rust calls its
    destructor (the `Drop` trait) automatically. This is
    called RAII — Resource Acquisition Is Initialization.

```rust
{
    let a = String::from("first");
    let b = String::from("second");
    let c = String::from("third");
}   // Drop order: c, b, a (reverse declaration order)
```

    Why reverse order?
        Later variables might reference earlier ones.
        Dropping in reverse ensures no dangling references
        during cleanup.

    You can implement Drop for custom types:

```rust
struct DatabaseConnection {
    name: String,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Closing connection: {}", self.name);
        // Real code: close socket, flush buffer, etc.
    }
}

fn main() {
    let conn = DatabaseConnection {
        name: String::from("prod-db"),
    };
    // ... use conn ...
}   // "Closing connection: prod-db" printed automatically.
    // No try/finally. No manual cleanup. No forgetting.
```

    Manual early drop:

```rust
let conn = DatabaseConnection { name: String::from("db") };
// ... use conn ...
drop(conn);   // explicitly drop NOW, before scope ends
// conn is no longer usable after this line.
```

### 8.10. Zero-Cost Abstractions — What the Compiler Actually Generates

    Rust promises "zero-cost abstractions": you don't
    pay for what you don't use, and what you do use is
    as fast as hand-written assembly.

    Let's verify with a concrete example:

```rust
fn sum_squares(data: &[i32]) -> i64 {
    data.iter()
        .map(|&x| x as i64 * x as i64)
        .sum()
}
```

    This iterator chain with closures compiles to the
    SAME assembly as the hand-written loop:

```rust
fn sum_squares_manual(data: &[i32]) -> i64 {
    let mut total: i64 = 0;
    let mut i = 0;
    while i < data.len() {
        let x = data[i] as i64;
        total += x * x;
        i += 1;
    }
    total
}
```

    In both cases, with optimizations on (`--release`),
    the compiler generates SIMD-vectorized machine code
    that processes multiple elements per CPU cycle.

    This is why Rust's high-level style doesn't cost
    you performance. The abstraction is erased at compile
    time. There is no iterator object at runtime. There
    is no closure object at runtime. There is just the
    tight loop the CPU needs.

### 8.11. Unsafe and Raw Pointers — Escaping the Safety Net

    Everything above happens within safe Rust. But Rust
    also provides an escape hatch: `unsafe`.

```rust
let x: i32 = 42;
let ptr: *const i32 = &x;   // raw pointer — creation is safe

unsafe {
    println!("Value: {}", *ptr);  // dereferencing is unsafe
}
```

    Raw pointers (*const T and *mut T):
        ├── Can be null.
        ├── Can be dangling.
        ├── Can alias (multiple mutable pointers to same data).
        ├── Not tracked by the borrow checker.
        └── Dereferencing requires `unsafe` block.

    When you need unsafe:
        ├── FFI (calling C libraries).
        ├── Implementing low-level data structures
        │   (linked lists, custom allocators).
        ├── Memory-mapped I/O (embedded systems).
        └── Performance-critical code where you can
            prove safety but the compiler can't.

    The rule of unsafe:
        The programmer takes responsibility for upholding
        all of Rust's safety invariants within the unsafe
        block. The compiler still checks everything outside.

    This document will not go deeper into unsafe — it's
    a separate discipline. But knowing it exists (and
    that all safe Rust guarantees are ultimately built
    on top of carefully audited unsafe code in the
    standard library) is part of understanding the
    full memory picture.

---

## 9. Scope (Where Your Name Tags Are Visible)

    Rule:
        A variable is accessible from where it is declared
        to the end of the enclosing block, including any
        nested inner blocks.

    Example:
        fn main() {
            let name = "Habtish";     // outer scope

            {
                let age = 25;         // inner scope
                println!("{name}");   // ✓ can see outer
                println!("{age}");    // ✓ can see inner
            }

            println!("{name}");       // ✓ still alive
            // println!("{age}");     // ✗ gone — inner
            //                        //   block ended
        }

    Visual model:
        Outer scope
            └── name ─── visible here and in all inner blocks
                Inner scope
                    └── age ─── visible ONLY here
                        └── name ─── still visible (inherited)

    Scope and shadowing interact:
        let x = 5;
        {
            let x = 20;        // shadows outer x
            println!("{x}");   // 20
        }
        println!("{x}");       // 5 — outer x reappears

    Why scope matters beyond visibility:
        When an owned value (like a `String`) goes out
        of scope, Rust automatically cleans up its
        memory. No garbage collector. No manual free.
        This is the backbone of Rust's safety story.

    Scope and memory — the connection:

```rust
fn main() {
    {
        let s = String::from("hello");
        // `s` is valid here. The String owns heap memory.
        println!("{s}");
    }   // `s` goes out of scope. Drop is called.
        // The heap memory for "hello" is freed RIGHT HERE.
        // Not "eventually." Not "when the GC runs." NOW.

    // The bytes that held "hello" on the heap are returned
    // to the allocator and may be reused immediately.
}
```

    This is deterministic destruction. You know EXACTLY
    when memory is freed: at the closing brace. This is
    why Rust can manage memory without a garbage collector.

---

## 10. Unused Variables (The Compiler Nudge)

    Rule:
        Rust warns you when a declared variable is never read.

        fn main() {
            let age = 25;  // warning: unused variable
        }

    Why this helps:
        Unused variables often signal forgotten code,
        incomplete logic, or calculation mistakes.

    How to silence the warning intentionally:
        ├── Prefix with underscore:
        │       let _age = 25;   // no warning
        │       // _age is still a real variable you
        │       // can use later if you want.
        │
        └── Standalone underscore (discard entirely):
                let _ = 25;      // no warning, no binding
                // println!("{_}"); // ✗ _ is not a name

    Important subtlety — `_` drops immediately:

```rust
let _s = String::from("hello");
// _s lives until end of scope — the String is alive.

let _ = String::from("hello");
// The String is dropped IMMEDIATELY — no binding exists.
// This matters when Drop has side effects
// (closing files, flushing buffers, releasing locks).
```

---

## 11. A Variable Can Hold More Than a Single Value

    Not every variable points to a plain number or string.
    It can point to a whole collection — or a custom structure.

    This is especially relevant to the "current value only"
    rule. A single variable holds one current value — but
    that one value can be a collection containing many items.

### Arrays (fixed size)

    let daily_sales = [120, 150, 95];
    // The variable `daily_sales` holds ONE value:
    // an array of three numbers.
    // The array itself preserves all three days.
    // The variable just points to the array.

    Memory: all 12 bytes (3 × i32) sit contiguously
    on the stack. No heap allocation. Cache-friendly.

### Vectors (growable collections)

    let mut daily_sales = vec![120, 150, 95];
    daily_sales.push(180);
    // Now the vector holds 4 elements.
    // The variable still holds one value — the vector.
    // But the vector internally tracks all entries.

    Memory: 24 bytes on the stack (ptr + len + cap),
    elements on the heap. See Section 8.7 for growth.

### Structs (multiple related fields)

    struct Customer {
        name: String,
        payment_method: String,
    }

    let customer = Customer {
        name: String::from("Mekdes"),
        payment_method: String::from("cash"),
    };

    // `customer` is ONE variable holding ONE value.
    // That value happens to be a struct with two fields.

    Memory: the struct sits on the stack, but each
    String field has a pointer to heap-allocated bytes.

### Enums (one choice from a fixed set)

    enum PaymentMethod {
        Cash,
        Card,
        Upi,
    }

    let method = PaymentMethod::Cash;

    Memory: an enum is sized to fit its largest variant
    plus a discriminant (tag byte). `PaymentMethod` with
    no data in variants is likely 1 byte.

    The takeaway:
        A variable gives a NAME to a value.
        The value can be a number, a string, a list,
        a struct, an enum, or something you invented.
        The TYPE tells Rust what shape that value has
        — and the shape determines where and how
        memory is allocated.

---

## 12. Variables as Bindings — Ownership, Borrowing, and Lifetimes

    Key insight:
        Assignment in Rust does not always copy data.
        The behavior depends on the type. This is
        Rust's ownership system — the feature that
        eliminates data races, dangling pointers, and
        double frees at compile time.

### 12.1. Simple types → Copy

    let a = 10;
    let b = a;        // value is copied
    println!("{a}");  // ✓ a is still usable
    println!("{b}");  // ✓ b has its own 10

    Integers, floats, bools, chars — all cheap to copy.
    They implement a trait called `Copy`, so Rust
    duplicates them silently on assignment.

    Why Copy is cheap:
        These types live entirely on the stack. Copying
        them is a simple memcpy of a few bytes. There's
        no heap allocation to worry about, no resource
        to manage.

### 12.2. Owned heap types → Move

    let s1 = String::from("Rust");
    let s2 = s1;          // ownership MOVES to s2
    // println!("{s1}");  // ✗ s1 was moved — unusable

    Why this exists:
        A `String` owns memory on the heap. If both
        `s1` and `s2` thought they owned the same memory,
        they'd both try to free it when done — a classic
        double-free bug that crashes real software.

        Rust prevents this by transferring ownership.
        After the move, only `s2` owns the string.
        `s1` is retired from active duty.

    What a move looks like at the memory level:

```
    Before move:
        Stack                   Heap
        s1: [ptr, len=4, cap=4] ──► "Rust"

    After `let s2 = s1;`:
        Stack                   Heap
        s1: [INVALID]          
        s2: [ptr, len=4, cap=4] ──► "Rust"

    The 24 bytes on the stack (ptr, len, cap) are
    memcpy'd from s1's slot to s2's slot. The heap
    data is NOT copied. Only one owner exists, so
    only one Drop will run.
```

### 12.3. Need two independent copies? Clone.

    let s1 = String::from("Rust");
    let s2 = s1.clone();   // full, separate copy
    println!("{s1}");      // ✓ still works
    println!("{s2}");      // ✓ independent string

    `clone()` is explicit on purpose. It can be expensive,
    so Rust makes you ask for it out loud.

    What clone does at the memory level:

```
    After clone:
        Stack                   Heap
        s1: [ptr_A, 4, 4]  ──► "Rust" (at address A)
        s2: [ptr_B, 4, 4]  ──► "Rust" (at address B)

    Two completely independent heap allocations.
    Modifying one does not affect the other.
    Each will be freed independently.
```

### 12.4. Borrowing — References Without Ownership Transfer

    Moving ownership everywhere would be crippling.
    Most of the time, you want to LOOK at data without
    TAKING it. That's what references do.

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}   // s (the reference) goes out of scope, but since
    // it doesn't own the String, nothing is freed.

fn main() {
    let greeting = String::from("Hello, world!");
    print_length(&greeting);  // borrow, don't move
    println!("{greeting}");   // ✓ still ours
}
```

    What a reference looks like in memory:

```
    Stack                        Heap
    greeting: [ptr, 13, 13] ──► "Hello, world!"
                  ▲
    s: [ptr] ─────┘  (s points to greeting's stack slot,
                       which points to the heap data)
```

    The borrowing rules (enforced at compile time):

```
    At any given time, you can have EITHER:
        ├── Any number of immutable references (&T)
        │       let r1 = &greeting;
        │       let r2 = &greeting;
        │       let r3 = &greeting;
        │       // All fine. Reading doesn't conflict.
        │
        └── Exactly ONE mutable reference (&mut T)
                let r = &mut greeting;
                // No other references allowed while
                // this mutable borrow is active.

    You CANNOT have both at the same time.
    This is the rule that prevents data races at compile time.
```

    Why one mutable XOR many immutable?
        ├── Multiple readers, no writers → safe.
        │   Everyone sees consistent data.
        │
        ├── One writer, no readers → safe.
        │   The writer can modify without anyone
        │   observing an intermediate, inconsistent state.
        │
        └── One writer + any readers → UNSAFE.
            A reader might see half-written data.
            In concurrent code, this is a data race.
            In single-threaded code, this can invalidate
            iterators, dangling references, etc.

    Practical example — the borrow checker saving you:

```rust
let mut data = vec![1, 2, 3];
let first = &data[0];     // immutable borrow of data
data.push(4);              // ✗ COMPILE ERROR
// push might reallocate, invalidating `first`.
// The borrow checker catches this at compile time.
// In C++, this compiles and may crash at runtime.
println!("{first}");
```

### 12.5. Lifetimes — How Long a Reference Is Valid

    Every reference has a lifetime — the span of code
    during which the reference is valid. Usually, the
    compiler infers lifetimes automatically ("lifetime
    elision"). Sometimes you must annotate them.

```rust
// Compiler infers: the returned &str lives as long
// as the input &str.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
```

    When you must annotate:

```rust
// Two inputs — which one does the output borrow from?
// The compiler can't guess. You must tell it.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a means: "the returned reference lives at least
//  as long as the shorter of x and y."
```

    What lifetimes prevent:

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s   // ✗ COMPILE ERROR
}        // s is dropped here, but we're returning
         // a reference to it — that would be a
         // dangling pointer. Rust stops us.
```

    The lifetime hierarchy:

```
    'static  ── lives for the entire program
       │         (string literals, leaked memory)
       ▼
    'a       ── lives for some scope 'a
       │         (function parameters, local borrows)
       ▼
    anonymous── the compiler figures it out
                 (most common case)
```

### 12.6. Mental model for move, copy, borrow

```
    Copy (stack types):
      a ──► [42]
      b ──► [42]    Two independent copies. Both usable.

    Move (heap types):
      s1 ──► [REVOKED]
      s2 ──► [ptr] ──► "Rust"    Only s2 is valid.

    Immutable borrow:
      owner ──► [ptr] ──► "Rust"
                  ▲
      &ref  ─────┘       Can read, not modify.

    Mutable borrow:
      owner ──► [ptr] ──► "Rust"   (owner temporarily frozen)
                  ▲
      &mut ref ──┘       Can read AND modify. Exclusive.
```

---

## 13. Variables vs. Databases (Programs Have Amnesia)

    A quick but important distinction that connects
    directly to the "current value only" idea.

    When your program is running:
        Variables remember the current value in RAM.

    When your program STOPS:
        Those variables vanish. Gone. Forgotten.
        Even the current value disappears.

    If you need information to survive across runs,
    it needs to live somewhere else:
        ├── A database
        ├── A file on disk
        └── A network service

    Example:
        let mut cups_sold = 120;
        cups_sold = 150;
        // Updated in memory only.
        // The database/file still says 120 until
        // you explicitly save the new value.

    The full memory hierarchy:

```
┌──────────────────┬─────────────────────────────────────────┐
│ Layer            │ What it remembers                       │
├──────────────────┼─────────────────────────────────────────┤
│ CPU Register     │ The current calculation. Nanoseconds.   │
│                  │ Vanishes when the register is reused.   │
├──────────────────┼─────────────────────────────────────────┤
│ Stack variable   │ The current value. No history.          │
│                  │ Vanishes when the scope ends.           │
├──────────────────┼─────────────────────────────────────────┤
│ Heap allocation  │ The current value. No history.          │
│ (String, Vec)    │ Vanishes when the owner is dropped.     │
├──────────────────┼─────────────────────────────────────────┤
│ Collection       │ All values you explicitly stored.       │
│ (Vec, HashMap)   │ Vanishes when the program ends.         │
├──────────────────┼─────────────────────────────────────────┤
│ Database / File  │ Values that survive program restart.    │
│                  │ Persists until explicitly deleted.      │
├──────────────────┼─────────────────────────────────────────┤
│ Distributed DB / │ Values that survive machine failure.    │
│ Replicated store │ Persists across multiple machines.      │
└──────────────────┴─────────────────────────────────────────┘
```

    Mental model:
        Registers  = your immediate thought.
        Stack      = scratch paper (thrown away after the task).
        Heap       = a notebook (carried until you close it).
        File / DB  = a library book (outlasts you).

---

## 14. Putting It All Together

```rust
const MAX_ATTEMPTS: u32 = 5;           // const: carved in stone

fn main() {
    let username = "ferris";            // immutable, inferred &str
    let mut attempts = 0;               // mutable, inferred i32

    let username = username.len();      // shadowed: now usize!

    attempts += 1;                      // mutation: fine, we said mut

    let result: f64 = "3.14"            // explicit type: parse needs
        .parse()                        // to know the target
        .expect("parse failed");

    let _debug_flag = true;             // intentionally unused

    let mut attempt_log = vec![1];      // Vec tracks history

    // Borrowing in action:
    let log_ref = &attempt_log;         // immutable borrow
    println!("Log so far: {:?}", log_ref);

    // log_ref is no longer used after this point,
    // so we can mutably borrow again:
    attempt_log.push(2);

    // Ownership in action:
    let message = String::from("Hello");
    let moved_message = message;        // ownership moves
    // println!("{message}");           // ✗ would fail — moved
    println!("{moved_message}");        // ✓ new owner

    println!(
        "User #{username} made {attempts} of \
         {MAX_ATTEMPTS} attempts. Result: {result}"
    );
}
```

    What is happening, line by line:
        MAX_ATTEMPTS
            └── const, explicit type, SCREAMING_CASE,
                compile-time. Inlined — no memory address.
        username (first)
            └── Immutable &str. Points to static string
                in the binary's read-only data segment.
        attempts
            └── Mutable i32 on the stack. 4 bytes.
                Updated in place. Holds only current count.
        username (second)
            └── Shadowed to usize via .len(). The old &str
                binding is eclipsed. A new stack slot (or
                register) holds the usize.
        result
            └── Explicit f64 because .parse() cannot guess.
                8 bytes on the stack.
        _debug_flag
            └── Underscore prefix suppresses unused warning.
                Likely optimized away entirely.
        attempt_log
            └── Vec<i32>: 24 bytes on stack (ptr, len, cap),
                elements on heap. Preserves history.
        log_ref
            └── Immutable borrow of attempt_log. 8-byte
                pointer on stack. Does not own the Vec.
        attempt_log.push(2)
            └── Mutable access. Only possible because
                log_ref's borrow has ended (NLL — non-lexical
                lifetimes detect this automatically).
        message → moved_message
            └── String moves. 24 stack bytes memcpy'd.
                Heap data untouched. message is invalidated.

---

## 15. Naming Variables Like a Pro

    This code works:
        let a = 120;
        let b = 25;
        let c = 10;
        let d = a * b - a * c;

    This code COMMUNICATES:
        let cups_sold = 120;
        let selling_price = 25;
        let cost_per_cup = 10;
        let daily_profit =
            cups_sold * selling_price - cups_sold * cost_per_cup;

    The compiler doesn't care which you write.
    Future-you at 2am DEEPLY cares.

    Rust's conventions:
        ├── snake_case for variables and functions
        │       let cups_sold = 120;
        │       fn calculate_profit() { }
        │
        ├── SCREAMING_SNAKE_CASE for constants
        │       const MAX_RETRIES: u32 = 5;
        │
        └── PascalCase for types (structs, enums, traits)
                struct Customer { }
                enum PaymentMethod { }

    Good names describe PURPOSE, not implementation:
        ✓ user_count        ✗ n
        ✓ is_logged_in      ✗ flag
        ✓ retry_delay_ms    ✗ x

---

## 16. Thinking Like a Developer: Picking Variables From a Problem

    When someone hands you a requirement, resist the urge
    to start typing `let` immediately. First, interrogate
    the problem.

    Walk through these questions:

        1. What information does the system need?
            → Candidate variables.

        2. What does each piece represent?
            → Good descriptive names.

        3. What KIND of value is it?
            → Pick the right type (int, float, string, bool…).

        4. Does it need to change while the program runs?
            → `mut` yes/no.

        5. Do I need to remember old values, or just the latest?
            → Single variable (latest) vs. collection (history).

        6. Will I need to change its TYPE later?
            → Shadowing vs. mut.

        7. Where will I use it? Just here, or everywhere?
            → Local variable vs. constant vs. struct field.

        8. Is it one value, or a collection, or structured?
            → Variable, Vec, struct, enum.

        9. Who owns it? Will it be copied or moved?
            → Copy types vs. owned types (ownership).

       10. Must it survive after the program exits?
            → Database/file, not just a variable.

       11. Where does it live in memory?
            → Stack (fixed size, short-lived) vs.
              Heap (dynamic size, shared ownership).

       12. Will multiple parts of the code need to read it?
            → References and borrowing.

       13. Will multiple threads need access?
            → Arc<Mutex<T>> or channels.

    Tea shop mapping (real example):

```
┌────────────────────┬───────────────────────────────────────────┐
│ Information        │ Rust representation                       │
├────────────────────┼───────────────────────────────────────────┤
│ Cups sold today    │ let mut cups_sold: u32  (stack, current)  │
│ Weekly sales log   │ let mut week: Vec<u32>  (stack+heap, hist)│
│ Selling price      │ let price_cents: u64    (stack, immutable)│
│ Cost per cup       │ let cost_cents: u64     (stack, immutable)│
│ Customer name      │ let name: String        (stack+heap, owned│
│ Payment method     │ enum PaymentMethod      (stack, 1 byte)   │
│ Daily profit       │ let profit: i64         (stack, computed) │
│ Shop name          │ const SHOP_NAME: &str   (inlined)         │
│ Max cups per order │ const MAX_PER_ORDER: u32 (inlined)        │
│ Shared config      │ Arc<Config>             (heap, thread-safe│
│ Historical records │ Database / File         (disk, persistent)│
└────────────────────┴───────────────────────────────────────────┘
```

---

## Quick-Recall Cheat Sheet

    let x = 5;
        └── Immutable by default. Cannot reassign.
            Stack-allocated. Possibly register-optimized.

    let mut x = 5;
        └── Opt-in mutability. Same type only.
        └── Holds only the CURRENT value. No history.

    let x = x + 1;
        └── Shadowing. New binding. Can change type.
            Old binding still exists but is invisible.

    const X: u32 = 5;
        └── Compile-time. Always typed. Never mutable.
            Inlined — no memory address.

    static X: u32 = 5;
        └── Fixed memory address. Lives entire program.
            Use for FFI, global state.

    Type inference
        └── Compiler figures out the type. Still static.
            Zero runtime cost.

    : Type annotation
        └── You tell the compiler. Required when ambiguous.

    ::<Type> turbofish
        └── Annotate on the function call, not the variable.

    Scope
        └── Variable lives from declaration to end of block.
            Drop runs at the closing brace. Deterministic.

    _prefix / _
        └── Silence unused warnings. _ discards AND drops
            immediately. _name keeps the value alive.

    Copy vs. Move
        └── Stack types (i32, f64, bool): copy on assignment.
            Heap types (String, Vec): move (old name dies).
            Clone: explicit deep copy. You pay for it.

    & (immutable borrow) vs. &mut (mutable borrow)
        └── Many &T OR one &mut T. Never both. Compile-time.

    Lifetimes ('a)
        └── How long a reference is valid. Usually inferred.
            Annotate when the compiler can't guess.

    =   vs.  ==
        └── Assign vs. compare. Never confuse the two.

    Single variable vs. Collection
        └── Variable: current value only. Snapshot.
            Collection: all values you stored. Journal.

    Stack vs. Heap
        └── Stack: fast, fixed-size, LIFO, ~8 MB.
            Heap: flexible, dynamic, scattered, ~GBs.

    Variable vs. Database
        └── Variable: lives in RAM, dies with the program.
            Database: lives on disk, survives restarts.

    Box<T>, Rc<T>, Arc<T>
        └── Box: single heap owner.
            Rc: shared ownership, single thread.
            Arc: shared ownership, multi-thread.

---

## The Questions to Ask Yourself

    1. What value am I working with?
        └── The actual data.

    2. What name am I giving it?
        └── The variable binding.

    3. What type does it have?
        └── Inferred or annotated. Locked at compile time.

    4. Will I need to reassign this binding?
        └── No → `let` (default, immutable).
        └── Yes → `let mut`.

    5. Am I updating the same binding or creating a new one?
        └── Same binding, same type → assignment with `mut`.
        └── New binding, possibly new type → shadow with `let`.

    6. Do I need to remember old values?
        └── No → a single variable is fine.
        └── Yes → use a collection, not a single variable.

    7. Where does this live in memory?
        └── Fixed-size, short-lived → stack.
        └── Dynamic-size or long-lived → heap.
        └── Permanent → const (inlined) or static (fixed address).

    8. Who owns this data?
        └── One owner → move semantics.
        └── Shared reading → immutable references (&T).
        └── Exclusive writing → mutable reference (&mut T).
        └── Shared ownership → Rc<T> or Arc<T>.

    9. How long must this reference stay valid?
        └── Within a function → compiler infers lifetime.
        └── Returned from function → may need 'a annotation.
        └── Forever → 'static.

   10. Must it survive after the program exits?
        └── Database, file, or network service.

---

**The golden rule:** Rust defaults to the safest option and makes you explicitly ask for power. Every `mut`, every `unsafe`, every `clone()` in your code is a conscious decision, documented right there in the source, visible to every future reader — including yourself at 2 AM debugging production.

**The deeper truth about variables:** A variable holds the current value and nothing else. It does not remember what it used to be. When you reassign it, the old value is gone from that variable's perspective. If you need history, use a collection. If you need persistence across program runs, use a database. A variable is a snapshot of the present — a single frame, not a movie.

**The memory truth:** Every variable is an abstraction over bytes. Those bytes live somewhere — in a register, on the stack, on the heap, or inlined as a constant. Understanding where they live, how they're laid out, how they're allocated and freed, and who owns them is what separates someone who writes Rust from someone who understands it. The ownership model, the borrow checker, the lifetime system — they are all consequences of taking memory seriously.

**The deepest goal:** Don't think in syntax. Think in information and resources. First understand WHAT your program needs to remember, how long it needs to remember it, where that memory lives, who owns it, and whether it needs the current value or the full history. Then translate that understanding into Rust's vocabulary — `let`, `mut`, `const`, types, references, lifetimes, collections, scope, ownership. Syntax is cheap. Clear thinking about data and memory is everything.

*Bookmark this. Come back when you forget. The compiler will remind you of the rest.*