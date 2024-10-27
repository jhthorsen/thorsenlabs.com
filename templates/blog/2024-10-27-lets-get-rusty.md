---
title: I'm getting rusty
---

I've been using [Perl](https://www.perl.org) and
[Javascript](https://developer.mozilla.org/en-US/) for about 24 years, and
haven't really used any other programming languages - at least not for creating
applications from the ground up. This makes 2024 a very exciting year for me,
since I have now started using both [Go](https://go.dev) and
[Rust](http://rust-lang.org/) for various types of projects.

## Go

Go feels like just a faster version of [Python](https://www.python.org) though.
It's a very simple language, which is both it's strength and it's weakness:
It's so quick to get started with, but the type system is very basic and
results in a lot of runtime panics, unless you are careful.

The error handling is just sh*t. I don't understand why you would want to
sprinkle three lines of code after every function call, just to make sure your
application does not blow up:

```go
res, err := my_awesome_function()
if err != nil {
    return err
}
```

I had this dicussion with a friend some years ago, where the argument was that
you just get the editor to add the if-err-then-return-err-code, but even if I
automate that, I still have to *read* those lines code later on.

I'm currently not a huge fan of Go, but it is growing on me.

## Rust

Rust is just amazing. I feel like I've wasted so much time testing
data-structures in Perl, as in "Did I really get the structure that I
expected?". This is not something I have to spend time doing in Rust, since the
types are what they are. It's also a lot more comfortable to work with than Go,
since you have [Option](https://doc.rust-lang.org/std/option/) and
[enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).

Error handling is also just wonderful. Have a look at this, compared to the Go
example above:

```rust
res = my_awesome_function()?
```

Just by adding a simple question mark at the end, you can bubble up the error,
that is if your return type allow the error type... It's still really nice
though, and saves me for a lot of typing and reading. The best thing is however
that you can't *forget* to check for errors! In Go, you can just drop the
if-statement and your application will for sure blow up at some point.

Coming from Perl, the speed is just mind-boggling: When I first rewrote my
[webpage](http://github.com/jhthorsen/thorsen.pm) in Rust, I thought I had
introduced some magic caching headers, since the page loaded instantaneous.
I do feel bad thinking about all the extra electricity servers have to use
to render for example a Wordpress site.

I can't wait to write some more Rust.

## Zig?

Haven't gotten around to Zig yet. I'm a bit scared of jumping on the "hype
train", but it looks like it's a serious language, which many people like, so
maybe it would be worth a look? I do however wonder which languages will
disappear, becuase there certainly is a lot of languages now...

## Resources

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Rust exercises](https://github.com/rust-lang/rustlings/)
* [Command line Rust](https://github.com/kyclark/command-line-rust)
* [From Perl to Rust](https://oylenshpeegul.gitlab.io/from-perl-to-rust/)
* [Go by Example](https://gobyexample.com/)
* [A tour of Go](https://go.dev/tour)
