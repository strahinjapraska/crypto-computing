use crate::protocols::bedoza::expressions::ExpressionTypes::XORWithConstant;


#[derive(Debug, Clone)]
pub enum ExpressionTypes {
    XORWithConstant,
    ANDWithConstant,
    XORWithTwoWires,
    ANDWithTwoWires,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum VariableNames {
    DONOR_A,
    DONOR_B,
    DONOR_RH,
    RECIPIENT_A,
    RECIPIENT_B,
    RECIPIENT_RH,
    A_AND,
    B_AND,
    RH_AND,
    A_XOR,
    B_XOR,
    RH_XOR,
    A_NEG,
    B_NEG,
    RH_NEG,
    A_B_AND,
    A_B_RH_AND,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression_type: ExpressionTypes,
    pub output_variable_name: VariableNames,
    pub first_input_variable_name: VariableNames,
    // can have either a secont input or a constant, depending on the expression type
    pub second_input_variable_name: Option<VariableNames>,
    pub constant: Option<bool>,
}

// this is only for testing: (d.a + r.a) * 1 + 1 
pub fn get_local_running_expressions() -> [Expression; 3] {
  [
    Expression {
      expression_type: ExpressionTypes::XORWithTwoWires,
      output_variable_name: VariableNames::A_XOR,
      first_input_variable_name: VariableNames::DONOR_A,
      second_input_variable_name: Some(VariableNames::RECIPIENT_A),
      constant: None
    },
    Expression {
      expression_type: ExpressionTypes::ANDWithConstant,
      output_variable_name: VariableNames::A_XOR,
      first_input_variable_name: VariableNames::A_XOR,
      constant: Some(true),
      second_input_variable_name: None
    },
    Expression {
      expression_type: XORWithConstant,
      output_variable_name: VariableNames::A_NEG,
      first_input_variable_name: VariableNames::A_XOR,
      constant: Some(true),
      second_input_variable_name: None
    
    }
  ]
}

pub fn get_blood_compatibility_expressions() -> [Expression; 11] {
    [
        // THE ANDS
        Expression {
            expression_type: ExpressionTypes::ANDWithTwoWires,
            output_variable_name: VariableNames::A_AND,
            first_input_variable_name: VariableNames::DONOR_A,
            second_input_variable_name: Some(VariableNames::RECIPIENT_A),
            constant: None,
        },
        Expression {
            expression_type: ExpressionTypes::ANDWithTwoWires,
            output_variable_name: VariableNames::B_AND,
            first_input_variable_name: VariableNames::DONOR_B,
            second_input_variable_name: Some(VariableNames::RECIPIENT_B),
            constant: None,
        },
        Expression {
            expression_type: ExpressionTypes::ANDWithTwoWires,
            output_variable_name: VariableNames::RH_AND,
            first_input_variable_name: VariableNames::DONOR_RH,
            second_input_variable_name: Some(VariableNames::RECIPIENT_RH),
            constant: None,
        },
        // THE XORS
        Expression {
            expression_type: ExpressionTypes::XORWithTwoWires,
            output_variable_name: VariableNames::A_XOR,
            first_input_variable_name: VariableNames::DONOR_A,
            second_input_variable_name: Some(VariableNames::A_AND),
            constant: None,
        },
        Expression {
            expression_type: ExpressionTypes::XORWithTwoWires,
            output_variable_name: VariableNames::B_XOR,
            first_input_variable_name: VariableNames::DONOR_B,
            second_input_variable_name: Some(VariableNames::B_AND),
            constant: None,
        },
        Expression {
            expression_type: ExpressionTypes::XORWithTwoWires,
            output_variable_name: VariableNames::RH_XOR,
            first_input_variable_name: VariableNames::DONOR_RH,
            second_input_variable_name: Some(VariableNames::RH_AND),
            constant: None,
        },
        // THE NEGS
        Expression {
            expression_type: ExpressionTypes::XORWithConstant,
            output_variable_name: VariableNames::A_NEG,
            first_input_variable_name: VariableNames::A_XOR,
            second_input_variable_name: None,
            constant: Some(true),
        },
        Expression {
            expression_type: ExpressionTypes::XORWithConstant,
            output_variable_name: VariableNames::B_NEG,
            first_input_variable_name: VariableNames::B_XOR,
            second_input_variable_name: None,
            constant: Some(true),
        },
        Expression {
            expression_type: ExpressionTypes::XORWithConstant,
            output_variable_name: VariableNames::RH_NEG,
            first_input_variable_name: VariableNames::RH_XOR,
            second_input_variable_name: None,
            constant: Some(true),
        },
        // COMBINING ALL THREE OF THEM
        Expression {
            expression_type: ExpressionTypes::ANDWithTwoWires,
            output_variable_name: VariableNames::A_B_AND,
            first_input_variable_name: VariableNames::A_NEG,
            second_input_variable_name: Some(VariableNames::B_NEG),
            constant: None,
        },
        Expression {
            expression_type: ExpressionTypes::XORWithConstant,
            output_variable_name: VariableNames::A_B_RH_AND,
            first_input_variable_name: VariableNames::A_B_AND,
            second_input_variable_name: Some(VariableNames::RH_NEG),
            constant: None,
        },
    ]
}