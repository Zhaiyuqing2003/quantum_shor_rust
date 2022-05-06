To run classical shor's algorithm, simply run 
>cargo install rand
>cargo install num
>cargo install libmath
for the necessary dependencies, then navigate to the folder named src/classicalshor, input the number you want to factor into main.rs where it says
"let num = a" where a is the number you input. Then just run
>cargo run
and the output will be a factor of the inputted number other than itself. Note if the result continues to be 1, then the number must be prime (as it has no factors other than 1 and itself)

To run the implemented classical shor's algorithm simply
>cargo run 
this will not cause overflow error as long as the input is *reasonable*

To run implemented quantum shor's algorithm simply
>cargo run
in the main.rs there is a function called quantum_shor(x), x is the number to factorize.
quantum_shor(15) could be done in reasonable time, while other might result in infinite loop or some runtime error.
this is normal simply because the number might be too big be to handle for simulation.
