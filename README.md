# qubit-rep

## Fast representation of single qubit.
With some gates implemented like: Hadamard, Pauli-X, Pauli-Y, Pauli-Z and Phase shift

## Usage
Clone the repo and run with cargo
```bash
  git clone https://github.com/lukkyye/qubit-rep.git
  cargo run
```
### Example
<img src="/example/example1.png">

### Using as lib
Initialize a qubit undetermined state:
```rs
  let qbit1: Qubit<PolarComplex<f32>> = Qubit::init();
```
Note that, you can especify the float and type as `f32` or `f64` (recommended `f32`)
Also, two complex types are implemented (PolarComplex<T> and CartesianComplex<T>) where T: Float

#### Hadamard gate
```rs
  let mut qbit1: Qubit<PolarComplex<f64>> = Qubit::init();
  qbit1.hadamard()
```
>Note that qbit1 is mutable, because hadamard gate transform qubit state into another one
#### Pauli gates (X, Y, Z)
```rs
  let mut qbit1: Qubit<CartesianComplex<f64>> = Qubit::init();
  qbit1.px(); qbit1.py(); qbit1.pz();
```
#### Print
```rs
  let qbit1: Qubit<PolarComplex<f32>> = Qubit::init();
  qbit1.print();
```
> Example output: " |φ⟩=0.9716602e^(-2.9490366i)|0⟩+0.2363819e^(2.7047591i)|1⟩ "
