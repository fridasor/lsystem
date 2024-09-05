# LSystem
Massively untrimmed code for constructing and viewing Lindenmayer systems (L-systems). You construct the L-system in an interface (made with `iced`) and press a button to render the structure. In the interface you can tweak parameters (the rules, axiom, angle, num of iterations). There are also templates to choose from (including the Hilbert curve, Koch curve, the Sierpinski triangle).

Heavily inspired by the [L-system renderer](http://piratefsh.github.io/p5js-art/public/lsystems/). Hinges solely (modulo the file `plotting.rs` for plotting the structures with `plotters`) on [iced](https://github.com/iced-rs/iced) and the examples in that repo.

## To do
* Prettify the interface
* Trim the code
* Add comments
* Add button for exporting plot