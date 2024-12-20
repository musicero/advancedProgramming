# Lab 06

I did this using rust

Money handling is easy, I just used f64!

Another even more robust way to do it is using int, and multiplying the value by 100. This way we can avoid floating point errors all together. u64 would be good enough for a long time, but we could also use u128 for future proofing in case of extreme inflation.

To run this, simply:
```bash
cargo run
```
