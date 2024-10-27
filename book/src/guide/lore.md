# Lore

Flux is a tool to add refinement types to Rust. Flux is developed at UC San Diego by Nico Lehmann, Adam Geller, Niki Vazou, and Ranjit Jhala.

[You can read the Flux paper here](https://ranjitjhala.github.io/static/flux-pldi23.pdf).

## Why Flux?

There have been previous systems ([Liquid Haskell](https://dl.acm.org/doi/10.1145/2628136.2628161), [F*](https://fstar-lang.org/papers/mumon/)) which added refinement types to functional languages. Flux extends Rust with a refinement type system for imperative code. 
Why Rust?
According to the Rust homepage
> Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time. 

## A brief history: Flux

In 1991, T. Freeman and F. Pfenning introduced refinement types in "Refinement types for ML". Refinement types let you add logical contraints to typed terms. A spry grad student, Ranjit Jhala, was inspired by the potential for program verification of large-scale imperative systems. Jhala has been working on scalable program verification since the turn of the millenium. At last, his vision is being realized. In 2008, Jhala presented his vision of refinement types, called Liquid Types, to the world.  Liquid Types have woven a path through TypeScript, Haskell, and now Rust. Flux hopes to expand lightweight verification to a popular, imperative systems language.

## Why is it called Flux?

> **flux** (/flʌks/) **1.** a flowing or flow. **2.** a substance used to refine metals. *v.* **3.** to melt; make fluid

| **INGREDIENTS**                | **CONTAINS** |
|-------------------------------|--------------|
| <mark>REFINEMENT TYPES</mark> | TYPES        |
| OWNERSHIP TYPES               |              |
| FERRIC OXYHYDROXIDE           |              |
| HYDRATED IRON OXIDE           |              |
| KING CRAB MEAT                |              |




    a flowing or flow:[countable]a flux of traffic.
    continuous change or movement:[uncountable]Our plans are in a state of flux.

Well typed programs can go wrong
 - insert liquid haskell blog