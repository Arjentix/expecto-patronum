# Expecto patronum

## Disclaimer

I, Daniil Polyakov, hereby waive all rights to the code and resources distributed with this document that use elements from the Harry Potter universe and ASCII art. I understand that this code and resources are not sanctioned by, affiliated with, or endorsed by the Harry Potter rights holders or the creators of the ASCII art and I make no claims to the contrary. I acknowledge that the use of any intellectual property from the Harry Potter universe and ASCII art in my code and resources is for entertainment purposes only and falls under the doctrine of fair use. I make no claim to any rights in the Harry Potter universe or ASCII art and do not imply any endorsement or approval from the Harry Potter rights holders or the creators of the ASCII art. The waiver of rights applies to the Harry Potter elements and ASCII art in my code and resources and does not extend to the code itself. The code is distributed under **MIT** license.

## Description

This crate allows you to replace all `expect(…)` with `expecto_patronum(…)` which works in the same way but also prints a beautiful patronus with your panic message. Patronus animal depends on the message you provide.

## Examples

Simple usage example:

```rust
use expecto_patronum::prelude::*;

let x = Some("Harry Potter");
assert_eq!(x.expecto_patronum("No dementors here"), "Harry Potter");
```

```rust
use expecto_patronum::prelude::*;

let x: Option<&str> = None;
x.expecto_patronum("No dementors here"); // panics with `No dementors here` and casts a patronus
```

Output of the last example:

```
thread 'main' panicked at '
 _,
<\ `.
  `. `~'^----.._            _
   `. ,    _,  `.`-.       ' )
   , ),'-~'(   / ` .`-.___,-'
  ( /;      `'\,    `
  _/'       _//       `.
 ' "       ' "       ' `
No dementors here
'
```

## Try this out

You can run

```bash
cargo run --example summon
```

And enter any line you want to get an example of generated animal.

## Resources

- Patronus list: [https://harrypotter.fandom.com/wiki/Patronus_Charm](https://harrypotter.fandom.com/wiki/Patronus_Charm)
- Main source of ASCII art: [http://www.ascii-art.de/](http://www.ascii-art.de/)
