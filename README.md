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