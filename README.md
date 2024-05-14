# Filio
### A Background file Organizer , which lets u to organize your data the way you want
### currently only suppots Unix based systems

> \[!IMPORTANT]
>
> Still in Development

<hr>

### How To Setup?
#### For now , You Have To Compile The Source Code Using
```shell
cargo build --release
```

#### Copy the binary into /bin if u want it globally
```
sudo cp ./target/release/filio /bin
``` 

<hr>

### How to run the program ?
```shell
filio start /path/to/json
```

### what json file?
#### filio uses json to keep track of which dirs to lookout for u can find example [here](./examples/) or
```json
"test" : {
    "input" : "path/to/dir",
    "output" : "path/to/dir",
    "extension" : "pdf",
    "action" : "mov",
    "prefix" : "exam"
}

```
#### this essentialy means that whenever a file changes in the dir 'input' and if that file has .pdf extension
#### mov that file into 'output' dir with the prefix of 'prefix' , although prefix is optional and u can even #### not write it and will be fine , for more example see [here](./examples/) or