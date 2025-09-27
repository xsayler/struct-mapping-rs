# Mapping Macro

A Rust procedural macro for automatically generating `From` and `Into` trait implementations with flexible field mapping and transformation capabilities.

## Overview

The `Mapping` macro allows you to automatically generate trait implementations for converting between different struct types, with support for:

- Custom field transformations
- Field skipping for specific target types
- Multiple source and target types
- Default value assignment
- Expression-based field mapping

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mapping-macro = { path = "./mapping-macro" }
```

## Basic Usage

```rust
use mapping_macro::Mapping;

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
```

This generates:

```rust
impl From<CreateUserDto> for UserModel {
    fn from(value: CreateUserDto) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            email: value.email,
            created_at: Some(chrono::Utc::now()),
            version: 1 as i32,
        }
    }
}

impl Into<UserDto> for UserModel {
    fn into(self) -> UserDto {
        UserDto {
            id: self.id,
            name: self.name,
            email: self.email,
            version: self.version,
        }
    }
}
```

## Attributes

### Struct-level Attributes

- `#[from(SourceType)]` - Generate a `From<SourceType>` implementation
- `#[into(TargetType)]` - Generate an `Into<TargetType>` implementation

You can specify multiple source and target types:

```rust
#[derive(Mapping)]
#[from(CreateProductDto)]
#[from(UpdateProductDto)]
#[into(ProductDto)]
#[into(ProductSummaryDto)]
pub struct ProductModel {
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
pub struct UserModel {
    id: Uuid,
    name: String,
    
    #[into_skip(UserDto)]  // Skip this field when converting to UserDto
    internal_data: String,
    
    #[from_skip(CreateUserDto)]  // Skip this field when converting from CreateUserDto
    calculated_field: i32,
}
```

## Advanced Examples

### Multiple Source Types with Different Mappings

```rust
#[derive(Mapping)]
#[from(CreateProductDto)]
#[from(UpdateProductDto)]
#[into(ProductDto)]
pub struct ProductModel {
    #[from(CreateProductDto | Uuid::new_v4())]
    #[from(UpdateProductDto | value.id)]
    id: Uuid,
    
    name: String,  // Maps directly from both source types
    
    #[from(CreateProductDto | value.price)]
    #[from(UpdateProductDto | value.price.unwrap_or(0.0))]
    price: f64,
    
    #[from(CreateProductDto | 1 as i32)]
    #[from(UpdateProductDto | value.version + 1)]
    version: i32,
}

pub struct CreateProductDto {
    name: String,
    price: f64,
}

pub struct UpdateProductDto {
    id: Uuid,
    name: String,
    price: Option<f64>,
    version: i32,
}

pub struct ProductDto {
    id: Uuid,
    name: String,
    price: f64,
    version: i32,
}
```

### Multiple Target Types with Selective Fields

```rust
#[derive(Mapping)]
#[from(CreateUserDto)]
#[into(UserDto)]
#[into(UserSummaryDto)]
pub struct UserModel {
    id: Uuid,
    name: String,
    email: Option<String>,
    
    #[into_skip(UserSummaryDto)]  // Skip detailed fields in summary
    created_at: Option<DateTime<Utc>>,
    
    #[into_skip(UserSummaryDto)]
    last_login: Option<DateTime<Utc>>,
    
    version: i32,
}

pub struct UserDto {
    id: Uuid,
    name: String,
    email: Option<String>,
    created_at: Option<DateTime<Utc>>,
    last_login: Option<DateTime<Utc>>,
    version: i32,
}

pub struct UserSummaryDto {
    id: Uuid,
    name: String,
    email: Option<String>,
    version: i32,
}
```

## Expression Syntax

In field mapping expressions, you can use:

- `value` - refers to the source struct instance
- Any valid Rust expression
- Function calls: `Uuid::new_v4()`, `Some(chrono::Utc::now())`
- Arithmetic operations: `value.version + 1`
- Method calls: `value.price.unwrap_or(0.0)`
- Literals: `1 as i32`, `"default".to_string()`

## Limitations

1. **Field Names**: By default, fields with the same name are mapped directly. Custom expressions override this behavior.

2. **Expression Parsing**: Complex expressions might need careful formatting. Simple expressions work best.

3. **Type Safety**: The macro doesn't validate that expressions return the correct type - this is caught at compile time.

4. **Struct Types Only**: The macro only works with structs that have named fields.

## Use Cases

This macro is particularly useful for:

- **Domain Model Conversion**: Converting between DTOs, entities, and domain models
- **API Layer Mapping**: Transforming request/response objects to internal representations  
- **Database Mapping**: Converting between database models and business objects
- **Cross-Crate Compatibility**: Mapping between types from different crates that can't implement each other's traits

## License

This project is licensed under the MIT License.