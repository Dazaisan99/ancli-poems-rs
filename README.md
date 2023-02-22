## **Warning : This crate is not usable**
### Please use [this addon](https://ankiweb.net/shared/info/2084557901) instead

# Ancli-poems-rs

Ancli-poems is a simple command line tool made with rust to help create anki cards from poems.

### Example
![Example](example.png)

<details><summary><strong>Installation</strong></summary> 
<p> 

### Dependencies
#### *You will only need cargo and git for the installation process*

### Process

Git clone the repository and go into it

```console
$ git clone https://github.com/dazaisan99/ancli-poems-rs

$ cd ancli-poems-rs
```

Build the crate

```console
$ cargo build --release
```

Finally, move the executable in ```target/release``` to a directory in your path (e.g. /usr/bin/ for linux)
</p> 
</details>

## Usage

```console
$ ancli-poems-rs -p/--path <PATH> -n/--name <NAME> [-h/--help]
```

PATH is the path to the file you want to create cards with

NAME is the name you want the deck created to have

A file with the name "*name*.apkg" will be created in the current directory

Next, open Anki, click *File > Import* and select the file.

The file has now been imported. Have fun !
