Solutions to https://adventofcode.com/ for 2022.

Let's be honest, Rust isn't the best choice (yet? areweadventyet?) for small
problems. There are some ergonomics papercuts (e.g. reading the input), while
zero cost abstractions (e.g. iterators that do not allocate) are not needed
for such small inputs.

But, at the end of the day, the point of Advent of Code is to have fun. So,
Rust it is.
