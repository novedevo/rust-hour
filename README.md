# Rust Hour


As is tradition in the Rust community, this is a rewrite of a project originally in a different language. As is also tradition, this rewrite resulted in a 10x performance improvement instantly, without any optimizations. After performance optimization, this program can solve all 35 test cases, and output the solved boards, in as little as 0.04 seconds on a Ryzen 5 3600. This is a result of various coding choices, but it is in no small part thanks to the aggressive optimization of the Rust compiler as well.

As with the Java version, our graph traversal algorithm was actually unimportant. Depth-first search performs better than breadth-first search, and A* with a basic heuristic lands in the middle. However, this is the difference between checking ~100k board states and ~70k states: not actually all that much.

One of the most relevant performance decisions was the way we kept track of which states we had visited. If this was not done at all, the program gets stuck in loops and takes a long time to terminate. We add tens of thousands of board states to it, and need to check if it contains many times that. The obvious choice is a HashMap, as it supports (theoretically) O(1) insertion and contains() checking. This worked, but upon profiling our executable with valgrind's Callgrind, we discovered that the majority of our program's runtime was being consumed by hashing operations. Our initial response was to switch to a BTreeMap, as those theoretically offer consistently better performance, but that requires ordering, and making that many comparisons actually slowed the program down tenfold. Upon further reading, I found that Rust's default hashing algorithm is SipHash, which is cryptographically secure and resistant to multiple forms of hash attacks, but isn't as fast as alternatives. Since cryptography was of no concern to this program, we looked for a better one, and found it in aHash. This crate (Rust's term for an importable third-party module, like you might find on Python's `pip`) purported to be the fastest algorithm available, and it maintained some resistance to attacks such as hashDOS. These claims had validity, 

Honestly, the performance barely matters. Practically nothing I did made the runtime untenable. I made a null hash function, it "only" increased the runtime to a few seconds. Disabling the `visited` map entirely was similar. My first implementation sorted a vector _every time_ the hash() function was called. I didn't notice; it was still under half a second. Running without any `rustc` optimizations still terminates within a few seconds.

It was to the degree that I was concerned my algorithm was cheating. So, I checked through all the solution files. They seemed to be in order, were all valid board states, had the same number and arrangement of pieces as the puzzle boards, and agreed (minus a few differential moves) with the Java solutions. The most recent move was also clear: there was space directly behind the X car that it had evidently just moved out of. This tracks with expected behaviour. That wasn't enough, so I stepped through the solution-finding process for an _entire board_. This took forever. I didn't see a single illegal move.

Looking online, some other people have done this before. This person solved every possible 6x6 Rush Hour board in minutes. That's billions of boards. He also exported some very nice-looking diagrams of the "clusters", or the completely explored graphs, consisting of every state you can get to from one given state. Each cluster had hundreds to thousands of states, but that's not very many to traverse.

I checked the number of stored states contained in visited for each board. Depending on the algorithm and some internal structure, this varied from a few hundred to a few thousand. Again, not very many for a fast computer to traverse.

One performance optimization I made is to only store the char array representation of the board in the hashmap, reducing calls to Board.clone() by ~100k per run. This helped significantly.

The fastest I have seen is using plain DFS with a Vec operating as a stack, without calculating any heuristics whatsoever, using unsafe rust and the mutate-clone-unmutate move generator, variable-length moves. This did 100 iterations of all 35 test boards within 1.15 seconds.

When using map() instead of unsafe rust, the time is in the 1.40 range, and up to 1.55.
Using A* and calculating heuristics, the time is nearly doubled, in the mid 2-second range. Checking this with valgrind, I noticed that roughly the same number of nodes are added to the visited hashset, but more than double the number of calls to get_moves() and .contains(). This tracks with an inferior algorithm. 
Using breadth-first search is the worst of them all. With a VecDeque as our queue, using push_back and pop_front commands, it took even longer than A*. This is an obvious non-starter.


