# Quantum Shor Algorithm

This repository implements the Quantum Shor Algorithm in Rust! 

The Shor Algorithm, simply speaking is a algorithm to factor numbers into primes: like `15 -> 3 * 5`. The Quantum Shor Algorithm could factor number in polynomial times (in a real quantum computer), which makes it possible to break the current encryption method like SHA256

The idea comes from the current course I have taken (PHYS 498 computational physics) where we implement the Quantum Shor Algorithm in scripting languages (The professor uses python, while I use typescript). The general logic is already figured out and implemented in typescript, and in the repository of ([Zhaiyuqing2003/Quantum_Shor_Algorithm_typescript (github.com)](https://github.com/Zhaiyuqing2003/Quantum_Shor_Algorithm_typescript)). The notes that professor given are in this website: https://courses.physics.illinois.edu/phys498cmp/sp2022/QC/Overview.html.

The idea of reimplementing in Rust is to greatly improve the simulation speed. (Quantum Computer simulation having a `O(2^n)` time complexity, so running it on scripting language is really slow). Although the JS have `Worker` and `WebGL` to use multithreading and GPU to improve speed, implementing in Rust will certainly provide better speed.

The quantum part of the process is most important, it involves a **Phase Estimation**   that execute in **Quantum Circuit**, the time complexity of the process is in polynomial time on a real quantum computer (though in simulation it's not). The Phase Estimation accomplished the **Period Finding** process where classical counterparts take in exponential time. To do the **Phase Estimation**, we need to implement **Quantum Fourier Transformation** which built upon some elementary quantum gate (check the wiki: [Quantum logic gate - Wikipedia](https://en.wikipedia.org/wiki/Quantum_logic_gate?msclkid=61b2356aaaef11ec8bfe954f8824ed0f)). The quantum gate could be maintained as either a **matrix** or a **state-function**. The latter is usually used because its significant simulation speed advantage. The quantum circuit need to maintain a **Quantum State**, which is the map of qubit configuration and the associate the Complex Number (it could be thought as some form of magnitude / likelihood of the certain configuration will result when the state get measured). A simple example is the qubit $|001 \rangle$ with complex number $0.6 + 0.8i$.

Currently Proposed To-do list

1. Implement Complex Number and Quantum State Struct (The Quantum State use `Map<u32, ComplexNumber>` to store the state).
2. Implement Elementary Quantum Gate `P`, `H` and `CNOT` as state-transform function. 
3. Implement the Quantum Fourier Transformation (QFT) and Phase Estimation circuit
4. Implement classical Shor's algorithm
5. Implement quantum Shor's algorithm

