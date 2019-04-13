# nib

## Summary

`nib` is a renderer created for research purposes. It is designed to be as
performant as possible without compromising the ability for a user to hack
around with the source code, especially with regards to implementing different
sampling strategies. It is written in Rust and uses Rayon for parallelization,
which calculates the value of each pixel as a unit of work.
