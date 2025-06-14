use database::show;
use crate::database::cloud::{backup, fetch, has_internet_access, push, Database};
use crate::database::{self, delete, insert, modify};

use clap::{Parser, Subcommand, Args};
use rusqlite::Result;

#[derive(Parser)]
#[command(name = "arino")]
#[command(about = "placeholder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
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
}

#[derive(Args)]
struct NewIngredientArgs {
    #[arg(long)]
    name: String,

    #[arg(long)]
    category: String,
}

#[derive(Args)]
struct AddPriceArgs {
    #[arg(long)]
    ingredient: String,

    #[arg(long)]
    price: f32,
}

#[derive(Args)]
struct NewDishArgs {
    #[arg(long)]
    name: String,
}

#[derive(Args)]
struct AddRecipeArgs {
    #[arg(long)]
    dish: String,

    #[arg(long)]
    ingredient: String,

    #[arg(long)]
    amount: f32,
}

#[derive(Args)]
struct ListAllDishesArgs {}

#[derive(Args)]
struct ListAllIngredientsArgs {}

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
    async fn execute(&self) -> Result<()> {
        match self {
            Command::NewIngredient(_args) => insert::ingredient().await,
            Command::AddPrice(_args) => insert::price().await,
            Command::NewDish(_args) => insert::dish().await,
            Command::AddRecipe(_args) => insert::recipe(None).await,
            Command::ListAllDishes(_args) => show::all_dish_names(),
            Command::ListAllIngredients(_args) => show::all_ingredients(),
            Command::IHave(_args) => show::dish_by_ingredients::get_dishes(),
            Command::RecipeOf(_args) => show::recipe_by_dish_name(),
            Command::DeleteIngredientFromRecipe(_args) => delete::ingredient_from_recipe().await,
            Command::DeleteDish(_args) => delete::dish().await,
            Command::DeleteIngredient(_args) => delete::ingredient().await,
            Command::Pull(_args) => {
                if has_internet_access().await {
                    fetch(Database::Main).await.expect("Error fetching database");
                } else {
                    eprintln!("Internet access is required to fetch database from cloud");
                }
                Ok(())
            },
            Command::Push(_args) => {
                if has_internet_access().await {
                    push().await.expect("Error syncing database");
                } else {
                    eprintln!("Internet access is required to sync database to cloud");
                }
                Ok(())
            },
            Command::Backup(_args) => {
                if has_internet_access().await {
                    backup().await.expect("Error backing up database");
                } else {
                    eprintln!("Internet access is required to backup database to cloud");
                }
                Ok(())
            },
            Command::UpdateIngredient(_args) => {
                modify::ingredient().await
            }
            Command::UpdateDishName(_args) => {
                modify::dish_name().await
            }
        }
    }
}

struct CommandDef {
    aliases: Vec<&'static str>,
    args: Vec<&'static str>,
}

