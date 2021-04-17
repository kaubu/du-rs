# du-rs
## Commands
Do `du-rs --help` to get a list of commands.  
Here are the commands for version 0.3.0:
```bash
me@mypc:~/Downloads$ ./du-rs-sync-linux64 --help
du-rs 0.3.0

kautionb <kaution3@pm.me>

USAGE:
    du-rs-sync-linux64 [FLAGS] [dir]

ARGS:
    <dir>    Directory to start from (default = current directory)

FLAGS:
    -h, --human-readable    
        --help              Prints help information
    -l, --count-links       Count sizes many times if hard links
    -s, --summarize         Produce a summary for the directory
    -V, --version           Prints version information
```
## Performance compared to du
```bash
'du -hsl ~/' ran
    1.10 ± 0.01 times faster than 'target/release/du-rs-sync -hsl ~/'
    3.69 ± 0.26 times faster than 'target/release/du-rs-async-par -hsl ~/'
   14.42 ± 0.86 times faster than 'target/release/du-rs-async-seq -hsl ~/'
```