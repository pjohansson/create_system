//! Edit a `DataBase`.

use database::DataBase;
use error::Result;
use ui::utils::{CommandList, CommandParser};

#[derive(Clone, Copy, Debug)]
enum Command {
    AddResidue,
    RemoveResidue,
    AddSubstrate,
    RemoveSubstrate,
    WriteToDisk,
    QuitAndSave,
    QuitWithoutSaving,
}

pub fn user_menu(database: &mut DataBase) -> Result<()> {
    let command_list: CommandList<Command> = vec![
        ("ra", Command::AddResidue, "Add a residue definition"),
        ("rr", Command::RemoveResidue, "Remove a residue definition"),
        ("sa", Command::AddSubstrate, "Add a substrate definition"),
        ("sr", Command::RemoveSubstrate, "Remove a substrate definition"),
        ("w", Command::WriteToDisk, "Write database to disk"),
        ("f", Command::QuitAndSave, "Finish editing database"),
        ("a", Command::QuitWithoutSaving, "Abort editing and discard changes"),
    ];
    let commands = CommandParser::from_list(command_list);

    commands.print_menu();

    unimplemented!();
}
