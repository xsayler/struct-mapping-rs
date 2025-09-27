//! Basic usage example of the Mapping macro

use chrono::{DateTime, Utc};
use mapping_macro::Mapping;
use uuid::Uuid;

/// A user model that can be created from CreateUserDto and converted to UserDto
#[derive(Mapping, Debug, Clone)]
#[from(CreateUserDto)]
#[into(UserDto)]
pub struct UserModel {
    /// ID is generated automatically when creating from CreateUserDto
    #[from(CreateUserDto | Uuid::new_v4())]
    id: Uuid,

    /// Name is mapped directly from the source
    name: String,

    /// Email is mapped directly from the source
    email: Option<String>,

    /// Created timestamp is set when creating from CreateUserDto
    #[from(CreateUserDto | Some(chrono::Utc::now()))]
    #[into_skip(UserDto)] // Skip this field when converting to UserDto
    created_at: Option<DateTime<Utc>>,

    /// Version starts at 1 when creating from CreateUserDto
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
}

/// DTO for user data without internal fields
#[derive(Debug, Clone)]
pub struct UserDto {
    id: Uuid,
    name: String,
    email: Option<String>,
    version: i32,
}

/// DTO for creating a new user
#[derive(Debug, Clone)]
pub struct CreateUserDto {
    name: String,
    email: Option<String>,
}

fn main() {
    println!("=== Basic Mapping Example ===\n");

    // Create a user creation request
    let create_request = CreateUserDto {
        name: "John Doe".to_string(),
        email: Some("john@example.com".to_string()),
    };

    println!("1. Creating user from CreateUserDto:");
    println!("   Name: {}", create_request.name);
    println!("   Email: {:?}", create_request.email);

    // Convert CreateUserDto to UserModel
    let user_model = UserModel::from(create_request);

    println!("\n2. Generated UserModel:");
    println!("   ID: {} (auto-generated)", user_model.id);
    println!("   Name: {}", user_model.name);
    println!("   Email: {:?}", user_model.email);
    println!(
        "   Created At: {:?} (auto-generated)",
        user_model.created_at
    );
    println!("   Version: {} (default)", user_model.version);

    // Convert UserModel to UserDto for API response
    let user_response: UserDto = user_model.into();

    println!("\n3. Converted to UserDto for response:");
    println!("   ID: {}", user_response.id);
    println!("   Name: {}", user_response.name);
    println!("   Email: {:?}", user_response.email);
    println!("   Version: {}", user_response.version);
    println!("   (created_at is skipped in UserDto)");

    println!("\n=== Example completed successfully! ===");
}
