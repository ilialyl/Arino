# Arino
A self-use **CLI** application capable of storing entries of dish recipes and their ingredients using SQLite, and sync the database using Dropbox API. It is written in Rust.

## Functionalities
### Store a database of
- Dishes.
- Ingredients split into groups (vegetables, fruits, dairy, meats, and condiments), and their prices and lifespans.
### Manually update the database
- Add, update, remove new dishes, recipes, and ingredients (its price and how long it lasts).
### Tell you what dishes in the database can possibly be made with the ingredients you have.
- For example, you can input eggs and it would show the dishes you can make with only eggs (though you can enter multiple ingredients) such as boiled eggs (if the dish exists in the database).
### Cloud Sync
- Push, fetch, and backup to Dropbox **(requiring a private key)**.

## To-Do List
- Store data about what ingredients you currently have
- Display ingredient usage priorities based on lifespans

## Usage Instruction
### Linux (Bash)
1. Download `arino` from release
2. cd to the directory Arino resides and run
```
# Generate Arino Bash autocompletion

./arino completion -p bash > arino.bash
```
```
# Source from it, this only lasts until you close your terminal.
# This allows you to autocomplete commands while typing by double-pressing tab

source arino.bash
```
3. type `./arino` to see the list of commands

### Windows (PowerShell) - Build from source
0. Install Rust -> https://www.rust-lang.org/tools/install
1. Clone this repository
2. cd into the Arino directory
3. run this snippet
```
# Build from source
cargo build --release
```
```
# Copy the executable to current directory
cp target/release/arino.exe .
```
```
# Generate Arino PowerShell autocompletion
.\arino.exe completion -p powershell > arino.ps1
```
```
# Source from it, this only lasts until you close your terminal.
# This allows you to autocomplete commands while typing by double-pressing tab
. .\arino.ps1
```
4. type `.\arino.exe` to see the list of commands
