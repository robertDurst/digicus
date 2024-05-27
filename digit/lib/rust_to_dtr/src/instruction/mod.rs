#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: String,
    pub input: Vec<String>,
    pub assign: String,
}

impl Instruction {
    pub fn new(name: String, input: Vec<String>, assign: String) -> Self {
        Self {
            name,
            input,
            assign,
        }
    }

    pub fn as_str(&self) -> String {
        if self.assign == "" {
            return format!(
                "{{ instruction: {}, input: ({:}) }}",
                self.name,
                self.input.join(", ")
            );
        }

        format!(
            "{{ instruction: {}, input: ({:}), assign: {} }}",
            self.name,
            self.input.join(", "),
            self.assign
        )
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.input == other.input && self.assign == other.assign
    }
}

impl Eq for Instruction {}
