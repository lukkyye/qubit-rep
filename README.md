# qubit-rep

## Fast representation of single qubit.
With some gates implemented like: Hadamard, Pauli-X, Pauli-Y, Pauli-Z and Phase shift

### Usage:

```bash
$ cargo add rand
$ cargo run
```
### Features:
- **Qubit Representation**: The qubit is represented as a normalized 2D complex vector: (|ψ⟩ = α|0⟩ + β|1⟩).
- **Quantum Gates**: Implements common quantum gates:
  - **Hadamard Gate (H)**
  - **Pauli-X Gate (X)**
  - **Pauli-Y Gate (Y)**
  - **Pauli-Z Gate (Z)**
  - **Phase Shift Gate**

### Examples:
Initialize a Qubit in random state.
```rust
  let newqubit: Qubit = Qubit::init(Forms::Bin); //Select your favorite representation for a Complex number Forms::{Bin, Exp, Coords}
```
Also, you can set the values by your own (not recommended):
```rust
  let z1: Complex = Complex::new(a, b, Forms::Bin); 
  let z2: Complex = Complex::new(c, d, Forms::Bin);
  let qubit1: Qubit = Qubit::new(z1, z2);
```
>Make sure that |z1|²+|z2|²=1

Make it collapse:
```rust
  let collapsed_newqubit: Qubit = newqubit.collapse(); //Output: |ϕ⟩= 0+0i|0⟩ + 1+0i|1⟩ or |ϕ⟩= 1+0i|0⟩ + 0+0i|1⟩
```
Take a basis state and put it on superposition with Hadamard gate:
```rust
  let mut qubit2: Qubit = Qubit::init(Forms::Bin);
  let collapsed_qubit2: Qubit = qubit2.collapse();
  collapsed_qubit2.hadamard() // Now; P(|0⟩)=50% and P(|1⟩)=50%
```
You can measure probabilities before collapsing:
```rust
let qubit3: Qubit = Qubit::init();
qubit3.measure() // Output looks like: P(|0⟩)= 0.65050066, P(|1⟩)= 0.34949934
```
Pauli-X, Pauli-Y, Pauli-Z gates also included:
```rust
let qubit4: Qubit = Qubit::init();
qubit4.px();
qubit4.py();
qubit4.pz();
```

