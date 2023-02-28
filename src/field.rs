use serde::{Serialize, Deserialize};

// FIXME: 固定長、可変長を区別するためにenumにするのはあり？
#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub bitsize: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSpec {
    pub fields: Vec<Field>,
}