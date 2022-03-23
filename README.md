# Quantum Shor Algorithm

This repository implements the Quantum Shor Algorithm in Rust! 

The idea comes from the current course I have taken (PHYS 498 computational physics) where we implement the Quantum Shor Algorithm in scripting languages (The professor uses python, while I use typescript). The general logic is already figured out and implemented in typescript, and in the repository of ([Zhaiyuqing2003/Quantum_Shor_Algorithm_typescript (github.com)](https://github.com/Zhaiyuqing2003/Quantum_Shor_Algorithm_typescript)). The notes that professor given are in this website: https://courses.physics.illinois.edu/phys498cmp/sp2022/QC/Overview.html.

The idea of reimplementing in Rust is to greatly improve the simulation speed. (Quantum Computer simulation having a `O(2^n)` time complexity, so running it on scripting language is really slow). Although the JS have `Worker` and `WebGL` to use multithreading and GPU to improve speed, implementing in Rust will certainly provide better speed.

Currently Proposed To-do list

1. Implement Complex Number and Quantum State Struct (The Quantum State use `Map<u32, ComplexNumber>` to store the state).
2. Implement Elementary Quantum Gate `P`, `H` and `CNOT` as state-transform function. 
3. Implement the Quantum Fourier Transformation (QFT) and Phase Estimation circuit
4. Implement classical Shor's algorithm
5. Implement quantum Shor's algorithm

