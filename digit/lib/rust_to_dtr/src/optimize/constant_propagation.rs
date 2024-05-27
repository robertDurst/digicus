use crate::Instruction;
use std::collections::HashMap;

pub fn apply(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut instruction_hash_table: HashMap<String, Vec<String>> = HashMap::new();
    let mut optimized_instructions: Vec<Instruction> = Vec::new();

    instructions.clone().into_iter().for_each(|instruction| {
        if instruction.name == "assign" {
            instruction_hash_table.insert(instruction.assign.clone(), instruction.input.clone());
        }

        let mut new_inputs: Vec<String> = Vec::new();
        instruction.input.clone().into_iter().for_each(|input| {
            if instruction_hash_table.contains_key(&input) {
                new_inputs.extend(instruction_hash_table.get(&input).unwrap().clone());
            } else {
                new_inputs.push(input);
            }
        });

        let optimized_instruction = Instruction::new(
            instruction.name.clone(),
            new_inputs,
            instruction.assign.clone(),
        );

        optimized_instructions.push(optimized_instruction);
    });

    optimized_instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_propagation_only_propagates_if_assign() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["a", "2"], "b"),
            create_instruction("add", vec!["b", "3"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["1", "2"], "b"),
            create_instruction("add", vec!["b", "3"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_latest_value() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "a"),
            create_instruction("add", vec!["a", "2"], "b"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "a"),
            create_instruction("add", vec!["5", "2"], "b"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["1", "5"], "c"),
            create_instruction("add", vec!["c", "4"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_multiple_values_with_multiple_assigns() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("assign", vec!["7"], "b"),
            create_instruction("add", vec!["a", "b"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("assign", vec!["5"], "b"),
            create_instruction("add", vec!["1", "5"], "c"),
            create_instruction("assign", vec!["7"], "b"),
            create_instruction("add", vec!["1", "7"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    #[test]
    fn constant_propagation_propagates_same_value_multiple_times() {
        let unoptimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["a", "b"], "c"),
            create_instruction("add", vec!["c", "a"], "d"),
        ];

        let expected_optimized_instructions = vec![
            create_instruction("assign", vec!["1"], "a"),
            create_instruction("add", vec!["1", "b"], "c"),
            create_instruction("add", vec!["c", "1"], "d"),
        ];

        assert_eq!(
            apply(unoptimized_instructions),
            expected_optimized_instructions
        );
    }

    fn create_instruction(name: &str, input: Vec<&str>, assign: &str) -> Instruction {
        Instruction::new(
            name.to_string(),
            input.into_iter().map(|s| s.to_string()).collect(),
            assign.to_string(),
        )
    }
}
