use crate::database::cloud::{backup, fetch, has_internet_access, push, Database};
use crate::database::{self, delete, insert, modify};
use database::{show, Category};

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::Bash};
use rusqlite::Result;
use std::io;

#[derive(Parser)]
#[command(name = "arino")]
#[command(about = "placeholder", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// An enum of commands
#[derive(Subcommand)]
#[command(rename_all = "snake_case")]
pub enum Command {
    NewIngredient(NewIngredientArgs),
    AddPrice(AddPriceArgs),
    NewDish(NewDishArgs),
    AddRecipe(AddRecipeArgs),
    ListAllDishes(ListAllDishesArgs),
    ListAllIngredients(ListAllIngredientsArgs),
    IHave(IHaveArgs),
    RecipeOf(RecipeOfArgs),
    DeleteIngredientFromRecipe(DeleteIngredientFromRecipeArgs),
    DeleteDish(DeleteDisArgsh),
    DeleteIngredient(DeleteIngredientArgs),
    Pull(PullArgs),
    Push(PushArgs),
    Backup(BackupArgs),
    UpdateIngredient(UpdateIngredientArgs),
    UpdateDishName(UpdateDishNameArgs),
    Completion,
}

#[derive(Args)]
pub struct NewIngredientArgs {
    #[arg(short = 'n', long = "name")]
    pub name: String,

    #[arg(
        short = 'c',
        long = "category",
        help = "Category must be one of (vegetable, fruit, dairy, meat, condiment, grain)."
    )]
    pub category: Category,

    #[arg(short = 'l', long = "lifespan")]
    pub lifespan: String,
}

#[derive(Args)]
pub struct AddPriceArgs {
    #[arg(short = 'i', long = "ingredient")]
    pub ingredient: String,

    #[arg(
        short = 'p',
        long = "price",
        help = "Price can be in floating point numbers, without currency prefixes."
    )]
    pub price: f32,
}

#[derive(Args)]
pub struct NewDishArgs {
    #[arg(short = 'n', long = "name")]
    pub name: String,
}

#[derive(Args)]
pub struct AddRecipeArgs {
    #[arg(short = 'd', long = "dish", help = "Name of an existing dish.")]
    pub dish: String,

    #[arg(
        short = 'i',
        long = "ingredient",
        help = "Name of an existing ingredient."
    )]
    pub ingredient: Vec<String>,

    #[arg(
        short = 'q',
        long = "quantity",
        help = "Quantity of the ingredient in numbers in grams (g)."
    )]
    pub quantity: Vec<String>,
}

#[derive(Args)]
pub struct ListAllDishesArgs {}

#[derive(Args)]
pub struct ListAllIngredientsArgs {
    #[arg(
        short = 'c',
        long = "category",
        help = "One of vegetable, fruit, dairy, meat, condiment, grain (default is all)."
    )]
    pub category: Option<Category>,
}

#[derive(Args)]
struct IHaveArgs {
    #[arg(long)]
    ingredients: Vec<String>,
}

#[derive(Args)]
struct RecipeOfArgs {
    #[arg(long)]
    dish: String,
}

#[derive(Args)]
struct DeleteIngredientFromRecipeArgs {
    #[arg(long)]
    dish: String,

    #[arg(long)]
    ingredient: String,
}

#[derive(Args)]
struct DeleteDisArgsh {
    #[arg(long)]
    dish: String,
}

#[derive(Args)]
struct DeleteIngredientArgs {
    #[arg(long)]
    ingredient: String,
}

#[derive(Args)]
struct PullArgs {}

#[derive(Args)]
struct PushArgs {}

#[derive(Args)]
struct BackupArgs {}

#[derive(Args)]
struct UpdateIngredientArgs {
    #[arg(long)]
    ingredient: String,

    #[arg(long)]
    new_name: Option<String>,

    #[arg(long)]
    new_category: Option<String>,
}

#[derive(Args)]
struct UpdateDishNameArgs {
    #[arg(long)]
    dish: String,

    #[arg(long)]
    new_name: String,
}

impl Command {
    pub async fn execute(&self) -> Result<()> {
        match self {
            Command::NewIngredient(args) => insert::ingredient(args).await,
            Command::AddPrice(args) => insert::price(args).await,
            Command::NewDish(args) => insert::dish(args).await,
            Command::AddRecipe(args) => insert::recipe(args).await,
            Command::ListAllDishes(args) => show::all_dish_names(args),
            Command::ListAllIngredients(args) => show::all_ingredients(args),
            Command::IHave(_args) => show::dish_by_ingredients::get_dishes(),
            Command::RecipeOf(_args) => show::recipe_by_dish_name(),
            Command::DeleteIngredientFromRecipe(_args) => delete::ingredient_from_recipe().await,
            Command::DeleteDish(_args) => delete::dish().await,
            Command::DeleteIngredient(_args) => delete::ingredient().await,
            Command::Pull(_args) => {
                if has_internet_access().await {
                    fetch(Database::Main)
                        .await
                        .expect("Error fetching database");
                } else {
                    eprintln!("Internet access is required to fetch database from cloud");
                }
                Ok(())
            }
            Command::Push(_args) => {
                if has_internet_access().await {
                    push().await.expect("Error syncing database");
                } else {
                    eprintln!("Internet access is required to sync database to cloud");
                }
                Ok(())
            }
            Command::Backup(_args) => {
                if has_internet_access().await {
                    backup().await.expect("Error backing up database");
                } else {
                    eprintln!("Internet access is required to backup database to cloud");
                }
                Ok(())
            }
            Command::UpdateIngredient(_args) => modify::ingredient().await,
            Command::UpdateDishName(_args) => modify::dish_name().await,
            Command::Completion => {
                print_completions();
                Ok(())
            }
        }
    }
}

struct CommandDef {
    aliases: Vec<&'static str>,
    args: Vec<&'static str>,
}

fn print_completions() {
    let mut cmd = Cli::command();
    generate(Bash, &mut cmd, "target/release/arino", &mut io::stdout());
}
