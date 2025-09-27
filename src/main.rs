use chrono::{DateTime, Utc};
use mapping_macro::Mapping;
use uuid::Uuid;

#[derive(Mapping, Debug, Clone)]
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
    #[from(CreateUserDto | Option::None)]
    #[into_skip(UserDto)]
    deleted_at: Option<DateTime<Utc>>,
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
}

#[derive(Debug, Clone)]
pub struct UserDto {
    id: Uuid,
    name: String,
    email: Option<String>,
    version: i32,
}

#[derive(Debug, Clone)]
pub struct CreateUserDto {
    name: String,
    email: Option<String>,
}

// Additional test structures for multiple mappings
#[derive(Mapping, Debug, Clone)]
#[from(CreateProductDto)]
#[from(UpdateProductDto)]
#[into(ProductDto)]
#[into(ProductSummaryDto)]
pub struct ProductModel {
    #[from(CreateProductDto | Uuid::new_v4())]
    #[from(UpdateProductDto | value.id)]
    id: Uuid,
    name: String,

    #[from(CreateProductDto | value.price)]
    #[from(UpdateProductDto | value.price.unwrap_or(0.0))]
    price: f64,

    #[from(CreateProductDto | Some(chrono::Utc::now()))]
    #[from(UpdateProductDto | value.updated_at)]
    #[into_skip(ProductSummaryDto)]
    updated_at: Option<DateTime<Utc>>,

    #[from(CreateProductDto | 1 as i32)]
    #[from(UpdateProductDto | value.version + 1)]
    #[into_skip(ProductSummaryDto)]
    version: i32,
}

#[derive(Debug, Clone)]
pub struct ProductDto {
    id: Uuid,
    name: String,
    price: f64,
    updated_at: Option<DateTime<Utc>>,
    version: i32,
}

#[derive(Debug, Clone)]
pub struct ProductSummaryDto {
    id: Uuid,
    name: String,
    price: f64,
}

#[derive(Debug, Clone)]
pub struct CreateProductDto {
    name: String,
    price: f64,
}

#[derive(Debug, Clone)]
pub struct UpdateProductDto {
    id: Uuid,
    name: String,
    price: Option<f64>,
    updated_at: Option<DateTime<Utc>>,
    version: i32,
}

fn main() {
    println!("=== Testing Mapping macro ===\n");

    // Test 1: Basic User mapping
    println!("1. Testing User mappings:");
    let create_user_dto = CreateUserDto {
        name: "John Doe".to_string(),
        email: Some("john@example.com".to_string()),
    };

    let user_model = UserModel::from(create_user_dto);
    println!("✓ Created UserModel from CreateUserDto:");
    println!("  ID: {}", user_model.id);
    println!("  Name: {}", user_model.name);
    println!("  Email: {:?}", user_model.email);
    println!("  Version: {}", user_model.version);
    println!("  Created at: {:?}", user_model.created_at);

    let user_dto: UserDto = user_model.into();
    println!("✓ Converted UserModel to UserDto:");
    println!("  ID: {}", user_dto.id);
    println!("  Name: {}", user_dto.name);
    println!("  Email: {:?}", user_dto.email);
    println!("  Version: {}", user_dto.version);
    println!();

    // Test 2: User with None email
    println!("2. Testing User with None email:");
    let create_user_dto2 = CreateUserDto {
        name: "Jane Smith".to_string(),
        email: None,
    };

    let user_model2 = UserModel::from(create_user_dto2);
    println!("✓ Created UserModel with None email:");
    println!("  Name: {}", user_model2.name);
    println!("  Email: {:?}", user_model2.email);

    let user_dto2: UserDto = user_model2.into();
    println!("✓ Converted to UserDto:");
    println!("  Name: {}", user_dto2.name);
    println!("  Email: {:?}", user_dto2.email);
    println!();

    // Test 3: Product mapping from CreateProductDto
    println!("3. Testing Product mappings from CreateProductDto:");
    let create_product_dto = CreateProductDto {
        name: "Laptop".to_string(),
        price: 999.99,
    };

    let product_model = ProductModel::from(create_product_dto);
    println!("✓ Created ProductModel from CreateProductDto:");
    println!("  ID: {}", product_model.id);
    println!("  Name: {}", product_model.name);
    println!("  Price: ${:.2}", product_model.price);
    println!("  Version: {}", product_model.version);

    let product_dto: ProductDto = product_model.clone().into();
    println!("✓ Converted ProductModel to ProductDto:");
    println!("  ID: {}", product_dto.id);
    println!("  Name: {}", product_dto.name);
    println!("  Price: ${:.2}", product_dto.price);
    println!("  Version: {}", product_dto.version);

    let product_summary: ProductSummaryDto = product_model.into();
    println!("✓ Converted ProductModel to ProductSummaryDto:");
    println!("  ID: {}", product_summary.id);
    println!("  Name: {}", product_summary.name);
    println!("  Price: ${:.2}", product_summary.price);
    println!();

    // Test 4: Product mapping from UpdateProductDto
    println!("4. Testing Product mappings from UpdateProductDto:");
    let update_product_dto = UpdateProductDto {
        id: Uuid::new_v4(),
        name: "Gaming Laptop".to_string(),
        price: Some(1299.99),
        updated_at: Some(chrono::Utc::now()),
        version: 2,
    };

    let product_model2 = ProductModel::from(update_product_dto.clone());
    println!("✓ Created ProductModel from UpdateProductDto:");
    println!("  ID: {}", product_model2.id);
    println!("  Name: {}", product_model2.name);
    println!("  Price: ${:.2}", product_model2.price);
    println!(
        "  Version: {} (incremented from {})",
        product_model2.version, update_product_dto.version
    );

    // Test 5: UpdateProductDto with None price (using default)
    println!("\n5. Testing UpdateProductDto with None price:");
    let update_product_dto_no_price = UpdateProductDto {
        id: Uuid::new_v4(),
        name: "Budget Laptop".to_string(),
        price: None, // This should default to 0.0
        updated_at: Some(chrono::Utc::now()),
        version: 1,
    };

    let product_model3 = ProductModel::from(update_product_dto_no_price);
    println!("✓ Created ProductModel with default price:");
    println!("  Name: {}", product_model3.name);
    println!("  Price: ${:.2} (defaulted)", product_model3.price);
    println!("  Version: {}", product_model3.version);

    println!("\n=== All tests completed successfully! ===");
}
