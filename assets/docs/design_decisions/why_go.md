# Why is Cadenza written in Go?

The choice of Go as the programming language for Cadonza came through in multiple montha of
experience, but it boils down to a few major factors.

## 1. Go is simple.

Go is a dead-simple programming language. Anything you write will still most likely be easily
understandable by contributors, and future me. That's very important to me as a solo dev. It means
that I won't end up maintaining a C++ monster like Sibelius, and I'll actually spend my time making
features that people want.

It also results in forks being easier to make, which is another win for FOSS.

## 2. Go is fast to develop in.

The biggest driver for my choice of Go. Building upon the previous section, using Go means that I
can ship features about as fast as C++ dev teams.

This allows me to make Cadenza actually be a competitor to MuseScore, Sibelius and Dorico. And I
find this non-negotiable.

## 3. Go is still about the same speed of C++ and Rust.

While people like to tout Rust as being fast, Go is still plenty, even with a garbage collector.
Go, Rust and C++ are all compiled to machine code anyway. But even then, a GUI frontend for
LilyPond doesn't have to run at full CPU power, and there's no point in optimising the GC out with
Rust.

Furthermore, while I could use an interpreted language, Go is still somehow the best mix of good
features. Again, it's simple, but it's also statically typed, and it's fast due to being compiled,
yet it has nearly instant compile times.

---

I actually wanted to switch to Rust in the beginning, as I wanted the enums, but during the first
few months of learning Rust and making Cadenza in Rust, I found out that it was a big mistake. Go is
simple and fast to develop in. That's the key reason in the end.
