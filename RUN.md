To run classical shor's algorithm, simply run 
>cargo install rand
>cargo install num
>cargo install libmath
for the necessary dependencies, then navigate to the folder named src/classicalshor, input the number you want to factor into main.rs where it says
"let num = a" where a is the number you input. Then just run
>cargo run
and the output will be a factor of the inputted number other than itself. Note if the result continues to be 1, then the number must be prime (as it has no factors other than 1 and itself)