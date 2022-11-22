# Hands-on #02: Segment tree

The provided crate produces both executables for the **min and max** problem and the **queries of operations** problem. To build and run the executables, open a terminal at the same location of this README file which has the cargo tool available, and run:

- `cargo run --bin min_and_max`
- `cargo run --bin queries_of_operations`

To run the tests, files can be passed using:

- `cargo run --bin NAME_OF_THE_PROBLEM < input_file.txt | diff - output_file.txt`

As an example:

- `cargo run --bin min_and_max < ./src/bin/min_and_max/tests/input0.txt | diff - ./src/bin/min_and_max/tests/output0.txt`

Additionally calling `cargo test` runs these same tests included in these folders, which are included into the test source files.

To *"install"* the generated executables to the current directory, run the command `cargo install --root . --path .` which will copy the generated executables in the `./bin/` folder.

# Hands-on Description
In this hands-on, we are going to solve two problems with segment trees.
For each problem, we have a pdf with the description of the problem and a set of tests to check the correctness of your implementations.

- Problem *Min and Max*: [Text](https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_01/text.pdf) and [TestSet.zip](https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_01/Testset.zip)
- Problem *Queries and Opearations*: [Text](https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_02/text.pdf) and [TestSet.zip](https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_02/TestSet.zip)

## Submission
Submit 
- a file ```lib.rs``` and a ```main.rs``` for each problem
- a file ```Handson_02_solution_YOUR_NAME.pdf``` to [rossano.venturini@gmail.com](mailto:rossano.venturini@gmail.com) 
by 23/11/2022. 

- The ```main.rs``` file takes its input from stdin and produces its output to the stdout. This way we can use ```./solution < input1.txt | diff - output1.txt``` to compare its output with the expected one on the first test case.
- A report ```Handson_02_solution_YOUR_NAME.pdf``` that briefly describes your solutions, your implementations, and an analysis of their time and space complexities. Add references to 
any relevant source you consulted to find your solutions or to develop their implementations. 

Before submitting your solutions, 
- make sure your implementation successfully passes all the tests.
- use ```cargo fmt``` to format your code. 
- use ```cargo clippy``` to check your code.
- use [Grammarly](https://grammarly.com/) to improve your English and avoid [tpyos](https://en.wiktionary.org/wiki/tpyo#English) :-). There is an [extension for vscode](https://marketplace.visualstudio.com/items?itemName=znck.grammarly).  

## Cheating
**Very important!** You are allowed to discuss possible solutions with other
students, **BUT** you have to implement all the solutions by yourself. 
Thus, sharing implementations with others is strictly **forbidden**.