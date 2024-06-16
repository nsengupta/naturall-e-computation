### Background

A few weeks back, in the Rust Programming Language [channel](https://www.linkedin.com/groups/4973032/) on LinkedIn, 
Ethan Barry (LinkedIn: https://www.linkedin.com/in/ethanbarry/) did a post, celebrating a fascinating quirk of 
mathematics (one amongst uncountable such): an iterative computation  seemingly unrelated to `'e'` , converges 
towards it, as the number of iterations in the computation increases. Here is the link to the [post](https://www.linkedin.com/feed/update/urn:li:activity:7197299671013154817/) and subsequent 
discussion. For more the _Mathy_ amongst us, here is the link to [Wikipedia]( [e (mathematical constant) - Wikipedia]
(https://en.wikipedia.org/wiki/E_(mathematical_constant)#Stochastic_representations))) where this behaviour is described. 

Amongst those who left their comments in the thread, Sergei Blinov (LinkedIn: https://www.linkedin.com/in/awnion/) offered an alternative way to compute the same, but using functional constructs and `rayon`'s parallel iterator facility. 

As an off-the-head , I had been curious to benchmark these two implementations, using Rust's benchmarking tools. 
These are CPU-bound computations, based on a single `u64` parameter (no of iterations): a perfect case for measuring the speed.

### Approach

I have used three different tools (crates) namely

-    Bencher (old and unstable)

-    Criterion (the most popular)

-    Divan (the new kid on the block)

There are other tools, but I chose not to use them, for this limited exercise.

Because the way, each of these tools are run to benchmark target functions, I have separated them into three directories, each having its own directory structure and `Cargo.toml`. 

### Code structure

There are 3 subdirectories, namely:

-    naturall-e-bencher

-    naturall-e-divan

-    naturall-e-criterion-with-asynch

To run the benchmarks, one needs to move to the corresponding directory and follow the instructions in respective README.

### Outcome

I could not find a way to run grouped functions in _Divan_. Moreover, I have failed toto run a `AsyncExecutor` -based benchmark in __Divan_. May be an way exists, but I haven't explored further. Here's _Divan_ 's output:

![Sample output](/home/nirmalya/Workspace-Rust/naturall-e-computation/Comparison-between-Handrolled-Parallel-Iteration.png "cargo-bench-output.png").

_Criterion_ allows us to group functions for easier comparison and its HTML reports are quite comprehensive. Moreover, it facilitates running AsyncExecutor-based tasks, viz., Tokio's. So, a three-way comparison is possible. 

Unsurpisingly, Rayon's Parallel Iterator facility works the best amongst the three.  The computation is great fit for data parallelization. Interestingly, Tokio's tasks perform reasonably well, given that its tasks are primarily meant for asynchronous I/O tasks.

<img title="cargo-bench-output.png" src="file:///home/nirmalya/Workspace-Rust/naturall-e-computation/Comparison-Response-1000000-Iterations.png" alt="Sample output" width="145" data-align="center">.

### Discloure

I am not a Rust benchmarking-super-expert and this is not a full-fledged benchmarking exercise. For example, I have not captured how much memory is consumed. My intention has been more to explore the tool more than to analyse the behaviour of the functions. 
