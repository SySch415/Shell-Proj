
// src/command_history.rs

pub struct CommandHistory {
    commands: Vec<String>,
    current_index: Option<usize>,
}

// implement command history struct
impl CommandHistory {
    
    // constructor
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_index: None,
        }
    }

    // add command to history vector
    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
        self.current_index = None;
    }

    // navigate up command history
    pub fn navigate_up(&mut self) -> Option<&String> {

        // check if there is a current index, if so, decrease index
        if let Some(index) = self.current_index {
            if index > 0 {
                self.current_index = Some(index - 1);
            }
          // if command history is fresh and command history is not empty, move index to begining of history  
        } else if !self.commands.is_empty() {
            self.current_index = Some(self.commands.len() - 1);
        }

        // return command
        self.current_index.map(move |i| &self.commands[i])
    }

    // navigate down command history
    pub fn navigate_down(&mut self) -> Option<&String> {
        
        // check if there is current index and its less than the last command in hsitory vector; if so, increase index
        if let Some(index) = self.current_index {
            if index < self.commands.len() - 1 {
                self.current_index = Some(index + 1);
            } else {
                self.current_index = None;
            }
        }

        // return command
        self.current_index.map(move |i| &self.commands[i])
    }
}