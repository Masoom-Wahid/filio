# Filio
### A Background file Organizer , which lets u to organize your data the way you want
### currently only suppots Unix based systems

<hr>

### How To Setup?
```shell
curl -L https://github.com/Masoom-Wahid/filio/releases/download/v1/install.sh | sudo sh
```

<hr>

### How to run the program ?
```shell
filio enable
```

### for more help on commands run
```shell
filio help
```


### How to edit the filio.json file
#### You can open it in your fav editor , for ex in vscode
```shell
filio cwd | xargs code
```

#### filio uses json to keep track of which dirs to lookout for u can find example [here](./examples/) or
```json
"test" : {
    "input" : "path/to/dir",
    "output" : "path/to/dir",
    "extension" : "pdf",
    "name":  "ch1,ch2",
    "action" : "mov",
    "prefix" : "exam"
}

```
#### this essentialy means that whenever a file changes in the dir 'input' and if that file has .pdf extension and has pdf substr
#### mov that file into 'output' dir with the prefix of 'exam' , although prefix is optional and u can even  not write it and will be fine , for more example see [here](./examples/)