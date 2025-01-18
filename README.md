# qubit-rep

## Simple representation of a Qubit.


```bash
$ cargo add rand
$ cargo run
```
### Examples:
Initialize a Qubit in a random state.
```rust
  let newqubit: Qubit = Qubit::init(Forms::Bin); //Select your favorite representation for a Complex number Forms::{Bin, Exp, Coords}
```
Also, you can set the values by your own:
```rust
  let z1: Complex = Complex::new(a, b, Forms::Bin); 
  let z2: Complex = Complex::new(c, d, Forms::Bin); //Be careful when you are selecting these values, make sure that |z1|+|z2|=1.
  let qubit1: Qubit = Qubit::new(z1, z2);
```
>to help you to choose the right values, you can use the `norm()` method from Complex

Make it collapse:
```rust
  newqubit.collapse(); //Output: |ϕ⟩= 0+0i|0⟩ + 1+0i|1⟩ or |ϕ⟩= 1+0i|0⟩ + 0+0i|1⟩
```
Take a basic state and put it on superposition with Hadamard gate:
```rust
  let mut qubit2: Qubit = Qubit::new(Complex::new(1.0, 0.0, Forms::Exp), Complex::new(0.0, 0.0, Forms::Exp));
  qubit2.hadamard() // Now; P(|0⟩)=50% and P(|1⟩)=50%
```
