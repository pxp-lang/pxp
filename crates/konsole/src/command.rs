use indexmap::IndexMap;

pub type CommandHandler = fn () -> ();

#[derive(Debug, Default, Clone)]
pub struct Command<T = CommandHandler> {
    pub name: String,
    pub description: Option<String>,
    pub arguments: IndexMap<String, ()>,
    pub handler: Option<T>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            arguments: IndexMap::default(),
            handler: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub(crate) fn help(&self) {
        
    }

    pub fn handle(mut self, handler: CommandHandler) -> Self {
        self.handler = Some(handler);
        self
    }
}