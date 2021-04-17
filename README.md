# du-rs
## Performance compared to du
```bash
'du -hsl ~/' ran
    1.10 ± 0.01 times faster than 'target/release/du-rs-sync -hsl ~/'
    3.69 ± 0.26 times faster than 'target/release/du-rs-async-par -hsl ~/'
   14.42 ± 0.86 times faster than 'target/release/du-rs-async-seq -hsl ~/'
```