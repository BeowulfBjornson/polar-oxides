# polar-oxides
A small experimentation with primes and polar coordinates

This is my [30 day project](https://dk30.day9.tv/projects/108445523291193344-1567550541907?t=1570981119338) for Day[9].
My goal was to get a little bit familiarized with Rust and practice what I learned after reading the Rust Book and watching
some online classes.

The inspiration for the idea behind this small project was [this video from 3Blue1Brown](https://www.youtube.com/watch?v=EK32jo7i5LQ).

It uses the [Coffee Engine](https://github.com/hecrj/coffee) to get some sort of performance out of trying to draw millions of
points and with some very naïve culling it actually manages to work with one hundred million points on my PC (i7 7700k @ 5Ghz
and RTX 2080).

## Usage

By default, it will draw 50k points, but you can change it by passing it a command-line argument:

```
$ ./polar-oxides 10000000 # will generate 10 million points
```

## Commands

* **W**: Zoom In
* **S**: Zoom Out
* **F**: Toggle Fullscreen
* **D**: Toggle drawing of non-primes
