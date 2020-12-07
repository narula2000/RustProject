# RustProject 
This was a part of OPL-ICCS311 Project, it task was to implement a parallelize version of an algorithm; Which I fail to do.
## The Goal
The goal was to implement an A* algorithm which is was parallelize. Initially I was interested on this algorithm was for it to slove a Rubik Cube. But Due to my lack of knowledge on Rust OOP.
## My Attempt
I started off by reading majority of paper on A* alogrithm with mixes of the parallel version. I have concluded that I will have to test my sequencial version with the crate version and implement the parallel version and compare to both. I assume that using rayon and it's into_par() function would work but due to theirs two map to keep track with specific type rayon wasn't allowing it. So I tried converting and implement my know ParallelIterator for rayon to use so it would it didn't get that far till I gave up. So I tried using multithread programming which I will lock each map when I remove or add new node to it and release after cost is compute. I tried to implement it but due to lack of time(Very bad time mangement on my part), I wasn't able to accomplish. But I did complete the comparision from the crate pathfinding version and mine
## Result
the data is in a_star/resources/report/report.zip you can unzip it can see how was the data is distributed I did a knight moves on a infinitly expanding chess board where the position to achieve was from 2,2 till 100,100 the result show that my implementation and theirs are close but I have more swing on the higher postion.

## Resources
* https://people.csail.mit.edu/rholladay/docs/parallel_search_report.pdf
* https://www.ijecs.in/index.php/ijecs/article/download/2774/2563/
* https://arxiv.org/abs/1708.05296
* https://spcl.inf.ethz.ch/Teaching/2013-dphpc/final/5.pdf
* https://docs.rs/pathfinding/2.0.4/pathfinding/directed/astar/fn.astar.html
* https://bheisler.github.io/criterion.rs/book/index.html
