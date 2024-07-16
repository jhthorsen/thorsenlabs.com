---
title: I'm getting rusty
status: draft
---

I've been using Perl and Javascript for about 24 years, and haven't really used any other programming languages. This makes 2024 a very exciting year for me, since I have now started using both Go and Rust for various types of projects.

## Go

Go feels like just a faster version of Python though. It's a very simple language, which is both it's strength and it's weakness. I've experienced so many runtime panics, where the code tries to access nil-slices, or nil-pointers. In addition, the error handling in Go is... quite annoying. I had this dicussion with a friend some years ago, where the argument was that you just get the editor to add the if-err-then-return-err-code, but even if I automate that, I still have to *read* those lines code later on.

I'm not a huge fan of Go.


## Rust

Rust on the other hand is just amazing. The highlighs for me are:

* Memory safety is maybe the main selling point, but I think it's a better selling point to people with a C/C++ background.
* You can add "?" at the end of a statemnt to bubble the error up the call tree. I can't believe this isn't added to more languages. It just makes the code so much easier to read.
* I find it amazing that you are forced to handle errors.
* I thought it would be annying to have structs that describes the data you are workign with to the point, but in reality it hasn't been a limitation yet.


## Resources

