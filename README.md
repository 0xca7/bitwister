# bitwister

![alt](doc/logo.png)

A very simple calculator for bit operations.
this calculator only understands binary operations like `+, -, AND, OR` and unary operations like `NOT`. More complex expressions are not supported.

---

### Usage

Build with `cd bitwister/` and `cargo build --release`.

Run the calculator with `./bitwister` which will give you a prompt: `[bt]>`.

All numeric inputs have the form: `[number]u[width]`, with the possible inputs: `[number]u8, [number]u16, [number]u32, [number]u64`. The calculator understands decimals and hex, if you want hex, add "0x" in front of the `[number]`, for example: `0xf00du16`.

**Example**: Adding two 8-bit integers.

```
[bt]> 1u8 + 1u8
```
**Example**: NOT 0xdeadbeef

```
[bt]> ! 0xdeadbeefu32
```

Operations that can overflow will show if an overflow occured.

**Register Display**: Let's say you're working with a microcontroller and bits are written to a register. It's possible you see something like this:

```
reg = 0xcafe
```

Now you consult a datasheet that tells you what the bits inside the register do, so you go through the hex number to check which registers are set. This can be cumbersome, for example if you work with 64-bit registers. This is why there is register mode `r [number]`:

```
[bc]> r 0xcafeu16
15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 
1  1  0  0  1  0  1 0 1 1 1 1 1 1 1 0 
[bc]> 0xcafe b1100101011111110 51966
```
This shows the invocation of register mode with the switch `r` and the corresponding register output in addition to the output of the number in hex, binary and decimal.

### List of Operations

- `+` add two numbers
- `-` subtract two numbers
- `*` multiply two numbers
- `&` logical AND of two numbers
- `|` logical OR of two numbers
- `^` logical XOR of two numbers
- `<<` shift left
- `>>` shift right
- `<<<` rotate left
- `>>>` rotate right
- `~` negate
- `!` logical NOT
- `r` register mode