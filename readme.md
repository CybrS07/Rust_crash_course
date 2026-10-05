Externsion Of Rust file. create a file add the 'rs' as the file externsion. 
to run the file, use the following command to first compile it.
```bash
rustc < filenaname >
```

This will compile it and when you want to run it enter the following command
```bash
./< filename >
```

Can also just fix it via adding a cargo env as using the following command
for new folder use
```bash
cargo new < foldername >
```
But incase of you want to make it inside an already existed folder use 
```bash
cargo init
```
And to run those file use the following command. This will automatically compile and show the output
```bash
cargo run
```