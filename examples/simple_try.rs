//! Simple TryFrom example for debugging

use anyhow::{anyhow, Result};
use mapping_macro::Mapping;
use std::convert::TryFrom;
use uuid::Uuid;

/// Simple user model with TryFrom
#[derive(Mapping, Debug, Clone)]
#[try_from(CreateUserDto)]
pub struct UserModel {
    #[try_from(CreateUserDto | Uuid::parse_str(&value.id_str)?)]
    id: Uuid,

    #[try_from(CreateUserDto | value.name)]
    name: String,

    #[try_from(CreateUserDto | value.age_str.parse::<i32>()?)]
    age: i32,
}

#[derive(Debug, Clone)]
pub struct CreateUserDto {
    id_str: String,
    name: String,
    age_str: String,
}

fn main() {
    println!("=== Simple TryFrom Test ===");

    // Test successful conversion
    let create_dto = CreateUserDto {
        id_str: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        name: "John Doe".to_string(),
        age_str: "30".to_string(),
    };

    match UserModel::try_from(create_dto) {
        Ok(user) => {
            println!("✓ Success: {:?}", user);
        }
        Err(e) => {
            println!("✗ Error: {}", e);
        }
    }

    // Test failed conversion (invalid UUID)
    let invalid_dto = CreateUserDto {
        id_str: "invalid-uuid".to_string(),
        name: "Jane Doe".to_string(),
        age_str: "25".to_string(),
    };

    match UserModel::try_from(invalid_dto) {
        Ok(user) => {
            println!("✓ Unexpected success: {:?}", user);
        }
        Err(e) => {
            println!("✓ Expected error: {}", e);
        }
    }
}
