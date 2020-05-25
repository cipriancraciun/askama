# [patched fork] Askama


**This is a patched fork!**  What follows bellow after the line, is from the original Askama readme.

In order to use this patched fork you can do the following:
* just replace `askama` with `askama_patched` in your `Cargo.toml` and Rust sources;
* alternatively, as a drop-in replacement without changing any of your Rust sources just rename the dependency in `Cargo.toml` as:
~~~~
[dependencies]
askama = { package = "askama_patched", version = "*" }
~~~~


----


[![Documentation](https://docs.rs/askama/badge.svg)](https://docs.rs/askama/)
[![Latest version](https://img.shields.io/crates/v/askama.svg)](https://crates.io/crates/askama)
[![Build Status](https://github.com/djc/askama/workflows/CI/badge.svg)](https://github.com/djc/askama/actions?query=workflow%3ACI)
[![Chat](https://badges.gitter.im/gitterHQ/gitter.svg)](https://gitter.im/djc/askama)

Askama implements a template rendering engine based on Jinja.
It generates Rust code from your templates at compile time
based on a user-defined `struct` to hold the template's context.
See below for an example, or read [the book][docs].

**"I use Askama for actix's TechEmpower benchmarks."** --
[Nikolay Kim][fafhrd91], creator of actix-web

**"Pretty exciting. I would love to use this already."** --
[Armin Ronacher][mitsuhiko], creator of Jinja

All feedback welcome. Feel free to file bugs, requests for documentation and
any other feedback to the [issue tracker][issues] or [tweet me][twitter].
Many thanks to [David Tolnay][dtolnay] for his support in improving Askama.

Askama was created by and is maintained by Dirkjan Ochtman. If you are in a
position to support ongoing maintenance and further development or use it
in a for-profit context, please consider supporting my open source work on
[Patreon][patreon].

### Feature highlights

* Construct templates using a familiar, easy-to-use syntax
* Template code is compiled into your crate for [optimal performance][benchmarks]
* Benefit from the safety provided by Rust's type system
* Optional built-in support for Actix, Gotham, Iron, Rocket and warp web frameworks
* Debugging features to assist you in template development
* Templates must be valid UTF-8 and produce UTF-8 when rendered
* Works on stable Rust

### Supported in templates

* Template inheritance
* Loops, if/else statements and include support
* Macro support
* Variables (no mutability allowed)
* Some built-in filters, and the ability to use your own
* Whitespace suppressing with '-' markers
* Opt-out HTML escaping
* Syntax customization

[docs]: https://djc.github.io/askama/
[fafhrd91]: https://github.com/fafhrd91
[mitsuhiko]: http://lucumr.pocoo.org/
[issues]: https://github.com/djc/askama/issues
[twitter]: https://twitter.com/djco/
[dtolnay]: https://github.com/dtolnay
[patreon]: https://www.patreon.com/dochtman
[benchmarks]: https://github.com/djc/template-benchmarks-rs


How to get started
------------------

First, add the following to your crate's `Cargo.toml`:

```toml
# in section [dependencies]
askama = "0.8"

```

Now create a directory called `templates` in your crate root.
In it, create a file called `hello.html`, containing the following:

```
Hello, {{ name }}!
```

In any Rust file inside your crate, add the following:

```rust
use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

fn main() {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.render().unwrap()); // then render it.
}
```

You should now be able to compile and run this code.

Review the [test cases] for more examples.

[test cases]: https://github.com/djc/askama/tree/master/testing
