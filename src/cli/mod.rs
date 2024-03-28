mod assets_cli;
mod datastore_cli;
mod experience_cli;
mod group_cli;
mod messaging_cli;
mod ordered_datastore_cli;

use clap::{Parser, Subcommand};

use self::{
    assets_cli::Assets, datastore_cli::DataStore, experience_cli::Experience, group_cli::Group,
    messaging_cli::Messaging, ordered_datastore_cli::OrderedDataStore,
};

#[derive(Debug, Parser)]
#[clap(name = "rbxcloud", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Access the Roblox Assets API
    Assets(Assets),

    /// Access the Roblox Experience API
    Experience(Experience),

    /// Access the Roblox Messaging API
    Messaging(Messaging),

    /// Access the Roblox DataStore API
    Datastore(DataStore),

    /// Access the Roblox OrderedDataStore API
    OrderedDatastore(OrderedDataStore),

    /// Access the Roblox Group API
    Group(Group),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            Command::Assets(command) => command.run().await,
            Command::Experience(command) => command.run().await,
            Command::Messaging(command) => command.run().await,
            Command::Datastore(command) => command.run().await,
            Command::OrderedDatastore(command) => command.run().await,
            Command::Group(command) => command.run().await,
        }
    }
}
