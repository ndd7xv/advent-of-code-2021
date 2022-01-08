# advent-of-code-2021

I do not guarantee my code to be the fastest, the cleanest, or even the most idiomatic. It is an exploration of the [Advent of Code](https://adventofcode.com), and an effort to improve my usage of Rust. What I write here is a reflection of my experience, from which I hope to improve my skills coding and problem solving B) I'm not looking at solutions until I've completed a problem, and even then I intend to do so in chunks, so that I can compare more general trends to what others are doing alongside the oppurtunity to pick apart specific questions.

With the (delayed) completion of the AoC 2021, I can definitely say it gets harder in the last week. I initially planned to write a lot more than what I'm writing now, but I realized I'd probably get more benefit reviewing the experience if I kept it succint.

**I experienced..**
- How nice iterators feel. While Rust's standard library can sometimes feel minimal, I never had a problem with iterator capabilities (windows, map, filter, etc.)
- Using generics and implementing traits! It wasn't necessary, but I'd like ot think it made code more idiomatic and generalizable
- Fights with the borrow checker, which ultimately ended with a better respect for the rules. Kind of seems like a right of passage
- `cargo clippy` and `cargo fmt` 👌
- The minimal effort it takes to write a test
- A sense of dread when I decided to `clone()` something I new would be large
- Self questioning after spending multiple hours trying to do day23p2 by hand
- Inferiority when seeing the performance and time it took people to solve questions

**I did not experience..**
- Lifetimes, though there were some questions where I thought they might be useful. Makes me wonder how necessary it is to write operational code, and how necessary it is to write *fast* code
- `unsafe` - not necessary in the same way life times weren't
- External libraries. I completed everything with std
- Thorough performance testing - though this may be something I'd like to do in the future with a library or flamegraph. I just `time`d all my binaries
- Macros, docs, custom use of `Cargo.toml`, and many more...