# Mapping Macro

A Rust procedural macro for automatically generating `From` and `Into` trait implementations with flexible field mapping and transformation capabilities.

## Overview

The `Mapping` macro allows you to automatically generate trait implementations for converting between different struct types, with support for:

- **Custom field transformations** - Use expressions to compute field values
- **Field skipping** for specific target types  
- **Multiple source and target types** - One struct can convert from/to many types
- **Default value assignment** - Set defaults when fields don't exist in source
- **Expression-based field mapping** - Full Rust expressions supported

This is particularly useful for domain model conversion, API layer mapping, and cross-crate compatibility where types can't implement each other's traits directly.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mapping-macro = { path = "./mapping-macro" }
uuid = { version = "1.0", features = ["v4"] }  # If using UUID examples
chrono = { version = "0.4", features = ["serde"] }  # If using DateTime examples
```

## Quick Start

```rust
use mapping_macro::Mapping;
use uuid::Uuid;

#[derive(Mapping)]
#[from(CreateUserDto)]
#[into(UserDto)]
pub struct UserModel {
    #[from(CreateUserDto | Uuid::new_v4())]
    id: Uuid,
    name: String,
    email: Option<String>,
    #[from(CreateUserDto | Some(chrono::Utc::now()))]
    #[into_skip(UserDto)]
    created_at: Option<DateTime<Utc>>,
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
}

pub struct UserDto {
    id: Uuid,
    name: String,
    email: Option<String>,
    version: i32,
}

pub struct CreateUserDto {
    name: String,
    email: Option<String>,
}

// Usage
let create_dto = CreateUserDto {
    name: "John Doe".to_string(),
    email: Some("john@example.com".to_string()),
};

let user_model = UserModel::from(create_dto);  // Auto-generated
let user_dto: UserDto = user_model.into();     // Auto-generated
```

## Attributes Reference

### Struct-level Attributes

- `#[from(SourceType)]` - Generate a `From<SourceType>` implementation
- `#[into(TargetType)]` - Generate an `Into<TargetType>` implementation

Multiple types are supported:

```rust
#[derive(Mapping)]
#[from(CreateDto)]
#[from(UpdateDto)]
#[into(ResponseDto)]
#[into(SummaryDto)]
pub struct Model {
    // fields...
}
```

### Field-level Attributes

#### Custom Field Mapping

Use `#[from(SourceType | expression)]` to provide custom values:

```rust
#[derive(Mapping)]
#[from(CreateUserDto)]
pub struct UserModel {
    #[from(CreateUserDto | Uuid::new_v4())]
    id: Uuid,
    
    #[from(CreateUserDto | Some(chrono::Utc::now()))]
    created_at: Option<DateTime<Utc>>,
    
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
    
    // Regular field mapping (uses value.field_name)
    name: String,
}
```

#### Field Skipping

Use `#[into_skip(TargetType)]` or `#[from_skip(SourceType)]` to skip fields for specific types:

```rust
#[derive(Mapping)]
#[from(CreateUserDto)]
#[into(UserDto)]
#[into(UserSummaryDto)]
pub struct UserModel {
    id: Uuid,
    name: String,
    
    #[into_skip(UserSummaryDto)]  // Skip when converting to summary
    internal_data: String,
    
    #[from_skip(CreateUserDto)]   // Skip when converting from create DTO
    calculated_field: i32,
}
```

## Advanced Examples

### Multiple Source Types with Different Logic

```rust
#[derive(Mapping)]
#[from(CreateOrderDto)]
#[from(UpdateOrderDto)]
#[into(OrderDto)]
pub struct OrderModel {
    #[from(CreateOrderDto | Uuid::new_v4())]
    #[from(UpdateOrderDto | value.id)]
    id: Uuid,
    
    #[from(CreateOrderDto | value.amount)]
    #[from(UpdateOrderDto | value.new_amount.unwrap_or(0.0))]
    total_amount: f64,
    
    #[from(CreateOrderDto | 1 as i32)]
    #[from(UpdateOrderDto | value.version + 1)]
    version: i32,
}
```

### Multiple Target Types with Selective Fields

```rust
#[derive(Mapping)]
#[from(CreateProductDto)]
#[into(ProductDto)]
#[into(ProductSummaryDto)]
pub struct ProductModel {
    id: Uuid,
    name: String,
    price: f64,
    
    #[into_skip(ProductSummaryDto)]  // Skip detailed fields in summary
    description: Option<String>,
    
    #[into_skip(ProductSummaryDto)]
    category_id: i32,
}
```

## Expression Syntax

In field mapping expressions, you have access to:

- `value` - The source struct instance
- Any valid Rust expression
- Function calls: `Uuid::new_v4()`, `Some(chrono::Utc::now())`
- Arithmetic operations: `value.version + 1`
- Method calls: `value.price.unwrap_or(0.0)`
- Literals: `1 as i32`, `"default".to_string()`
- Pattern matching: `match value.status { "active" => 1, _ => 0 }`

Examples:

```rust
#[derive(Mapping)]
#[from(SourceDto)]
pub struct Model {
    #[from(SourceDto | Uuid::new_v4())]
    id: Uuid,
    
    #[from(SourceDto | value.price.unwrap_or(99.99))]
    price: f64,
    
    #[from(SourceDto | value.name.to_uppercase())]
    name: String,
    
    #[from(SourceDto | match value.category.as_str() { "tech" => 1, "books" => 2, _ => 3 })]
    category_id: i32,
}
```

## Generated Code

For this input:

```rust
#[derive(Mapping)]
#[from(CreateUserDto)]
#[into(UserDto)]
pub struct UserModel {
    #[from(CreateUserDto | Uuid::new_v4())]
    id: Uuid,
    name: String,
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
}
```

The macro generates:

```rust
impl From<CreateUserDto> for UserModel {
    fn from(value: CreateUserDto) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            version: 1 as i32,
        }
    }
}

impl Into<UserDto> for UserModel {
    fn into(self) -> UserDto {
        UserDto {
            id: self.id,
            name: self.name,
            version: self.version,
        }
    }
}
```

## Use Cases

This macro is particularly useful for:

- **Domain Model Conversion** - Converting between DTOs, entities, and domain models
- **API Layer Mapping** - Transforming request/response objects to internal representations  
- **Database Mapping** - Converting between database models and business objects
- **Cross-Crate Compatibility** - Mapping between types from different crates that can't implement each other's traits
- **Microservice Communication** - Converting between service-specific data structures

## Real-world Example

```rust
// In your domain layer
#[derive(Mapping, Debug, Clone)]
#[from(api::CreateUserRequest)]
#[from(db::UserEntity)]
#[into(api::UserResponse)]
#[into(events::UserCreatedEvent)]
pub struct User {
    #[from(api::CreateUserRequest | Uuid::new_v4())]
    #[from(db::UserEntity | value.id)]
    id: Uuid,
    
    name: String,  // Maps directly from all sources
    email: String,
    
    #[from(api::CreateUserRequest | Some(chrono::Utc::now()))]
    #[from(db::UserEntity | value.created_at)]
    #[into_skip(events::UserCreatedEvent)]
    created_at: Option<DateTime<Utc>>,
    
    #[from(api::CreateUserRequest | 1 as i32)]
    #[from(db::UserEntity | value.version)]
    #[into_skip(events::UserCreatedEvent)]
    version: i32,
}
```

## Running the Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Advanced usage with multiple types
cargo run --example advanced_usage

# Run all tests
cargo test --all

# Run just the main demo
cargo run
```

## Limitations

1. **Field Names**: By default, fields with the same name are mapped directly. Custom expressions override this behavior.

2. **Expression Parsing**: Complex expressions might need careful formatting. Simple expressions work best.

3. **Type Safety**: The macro doesn't validate that expressions return the correct type - this is caught at compile time.

4. **Struct Types Only**: The macro only works with structs that have named fields.

5. **Proc Macro Context**: Some expressions might not work in all contexts due to Rust's procedural macro limitations.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## Testing

The project includes comprehensive tests:

- Unit tests in `src/lib.rs` test the core functionality
- Integration tests verify real-world usage patterns
- Examples serve as both documentation and integration tests
- Doctests ensure documentation examples work

```bash
# Run all tests
cargo test --all

# Run specific test suites
cargo test --lib                    # Library tests only
cargo test -p mapping-macro         # Macro tests only
cargo test --example basic_usage    # Example as test
```
