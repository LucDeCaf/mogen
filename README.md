# Mogen
A chess move generation library written in Rust.

## Motivation

I already have a chess move generation library called Chress. This project aims to be a reimagining of that project with a few key differences.

### Scope

My old chess move generation library, Chress, grew larger and larger in scope, eventually becoming a move generator,
UCI compliant engine, and testing framework all in one. This project is smaller in scope and will just be the
move generation aspects of Chress (plus some changes).

### Architecture

Chress used a make-unmake approach to move generation, which is the modern standard to managing board state; however, it is also rather complicated, and the unmake function
alone took around 3-4 weeks to create and debug. Mogen will use a copy-make approach, which is slightly slower but also much easier to implement and maintain.

### External dependencies

Chress tried to contain as few external dependencies as possible, eventually only containing the `rand` crate provided by the Rust Foundation. Mogen will forego these
restrictions and allow itself to use external crates (serde, tokio, clap) as I see fit.
