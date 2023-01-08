use pest::Parser;

use space10::{CharacterId, CharacterIdSizeType, Game};

#[derive(Parser)]
#[grammar = "src/web/parser/command/command.pest"]
struct CommandParser;

#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    GetCharacter(CharacterId),
    GetPlayer,
    ListCommands,
    ListCharacters,
    ListSettledZones,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    Parse(pest::error::Error<Rule>),
    Generic(String),
}

impl Command {
    pub fn parse(game: &Game, input: &str) -> Result<Self, Error> {
        let pairs = CommandParser::parse(Rule::run_command, input);

        match pairs {
            Ok(mut pairs) => {
                let command_node = pairs
                    .next()
                    .expect("expecting one command node (plus end-of-input)");

                match command_node.as_rule() {
                    Rule::get_character => {
                        let arg = command_node.into_inner().next().unwrap();
                        assert_eq!(arg.as_rule(), Rule::integer);
                        let arg_span = arg.as_span();
                        let id: Result<CharacterIdSizeType, _> = arg_span.as_str().parse();

                        match id {
                            Ok(id) => {
                                let copied_id = id;
                                let parsed_id = game.parse_character_id(copied_id);

                                if let Some(id) = parsed_id {
                                    Ok(Command::GetCharacter(id))
                                } else {
                                    Err(Error::Generic("character does not exist".into()))
                                }
                            }
                            Err(e) => Err(Error::Generic(e.to_string())),
                        }
                    }
                    Rule::get_player => Ok(Command::GetPlayer),
                    Rule::list_commands => Ok(Command::ListCommands),
                    Rule::list_characters => Ok(Command::ListCharacters),
                    Rule::list_settled_zones => Ok(Command::ListSettledZones),
                    _ => {
                        unreachable!();
                    }
                }
            }
            Err(e) => Err(Error::Parse(e)),
        }
    }
}
