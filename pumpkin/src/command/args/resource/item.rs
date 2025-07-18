use async_trait::async_trait;
use pumpkin_data::item::Item;
use pumpkin_protocol::java::client::play::{ArgumentType, CommandSuggestion, SuggestionProviders};
use pumpkin_util::text::TextComponent;

use crate::command::{
    CommandSender,
    args::{
        Arg, ArgumentConsumer, ConsumedArgs, DefaultNameArgConsumer, FindArg,
        GetClientSideArgParser,
    },
    dispatcher::CommandError,
    tree::RawArgs,
};
use crate::server::Server;

pub struct ItemArgumentConsumer;

impl GetClientSideArgParser for ItemArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType {
        ArgumentType::Resource { identifier: "item" }
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for ItemArgumentConsumer {
    async fn consume<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        // todo: get an actual item
        Some(Arg::Item(args.pop()?))
    }

    async fn suggest<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion>>, CommandError> {
        Ok(None)
    }
}

impl DefaultNameArgConsumer for ItemArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "item"
    }
}

impl<'a> FindArg<'a> for ItemArgumentConsumer {
    type Data = (&'a str, &'static Item);

    fn find_arg(args: &'a ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Item(name)) => {
                Item::from_registry_key(name.strip_prefix("minecraft:").unwrap_or(name))
                    .map_or_else(
                        || {
                            if name.starts_with("minecraft:") {
                                Err(CommandError::CommandFailed(Box::new(
                                    TextComponent::translate(
                                        "argument.item.id.invalid",
                                        [TextComponent::text((*name).to_string())],
                                    ),
                                )))
                            } else {
                                Err(CommandError::CommandFailed(Box::new(
                                    TextComponent::translate(
                                        "argument.item.id.invalid",
                                        [TextComponent::text("minecraft:".to_string() + *name)],
                                    ),
                                )))
                            }
                        },
                        |item| Ok((*name, item)),
                    )
            }
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}
