//! Advanced usage example showing multiple source and target types

use chrono::{DateTime, Utc};
use mapping_macro::Mapping;
use uuid::Uuid;

/// Product model that can be created from multiple DTOs and converted to multiple DTOs
#[derive(Mapping, Debug, Clone)]
#[from(CreateProductDto)]
#[from(UpdateProductDto)]
#[from(ImportProductDto)]
#[into(ProductDto)]
#[into(ProductSummaryDto)]
#[into(ProductListItemDto)]
pub struct ProductModel {
    /// ID handling varies by source type
    #[from(CreateProductDto | Uuid::new_v4())]
    #[from(UpdateProductDto | value.id)]
    #[from(ImportProductDto | Uuid::parse_str(&value.external_id).unwrap_or_else(|_| Uuid::new_v4()))]
    id: Uuid,

    /// Name is mapped directly from all sources
    name: String,

    /// Description handling with defaults
    #[from(CreateProductDto | value.description)]
    #[from(UpdateProductDto | value.description.unwrap_or_else(|| "No description".to_string()))]
    #[from(ImportProductDto | Some(value.description))]
    #[into_skip(ProductSummaryDto)]
    #[into_skip(ProductListItemDto)]
    description: Option<String>,

    /// Price with different logic per source
    #[from(CreateProductDto | value.price)]
    #[from(UpdateProductDto | value.new_price.unwrap_or(0.0))]
    #[from(ImportProductDto | value.price_cents as f64 / 100.0)]
    price: f64,

    /// Category with defaults and transformations
    #[from(CreateProductDto | value.category_id)]
    #[from(UpdateProductDto | value.category_id.unwrap_or(1))]
    #[from(ImportProductDto | match value.category_name.as_str() { "electronics" => 1, "books" => 2, _ => 3 })]
    #[into_skip(ProductSummaryDto)]
    category_id: i32,

    /// Timestamps with different logic
    #[from(CreateProductDto | Some(chrono::Utc::now()))]
    #[from(UpdateProductDto | Some(chrono::Utc::now()))]
    #[from(ImportProductDto | value.created_at)]
    #[into_skip(ProductSummaryDto)]
    #[into_skip(ProductListItemDto)]
    updated_at: Option<DateTime<Utc>>,

    /// Version increment logic
    #[from(CreateProductDto | 1 as i32)]
    #[from(UpdateProductDto | value.current_version + 1)]
    #[from(ImportProductDto | 1 as i32)]
    #[into_skip(ProductSummaryDto)]
    #[into_skip(ProductListItemDto)]
    version: i32,

    /// Status with different defaults
    #[from(CreateProductDto | "draft".to_string())]
    #[from(UpdateProductDto | value.status.unwrap_or_else(|| "updated".to_string()))]
    #[from(ImportProductDto | "imported".to_string())]
    #[into_skip(ProductListItemDto)]
    status: String,

    /// Stock management
    #[from(CreateProductDto | 0 as i32)]
    #[from(UpdateProductDto | value.stock_adjustment)]
    #[from(ImportProductDto | value.initial_stock)]
    #[into_skip(ProductSummaryDto)]
    stock_quantity: i32,
}

/// Complete product DTO with all fields
#[derive(Debug, Clone)]
pub struct ProductDto {
    id: Uuid,
    name: String,
    description: Option<String>,
    price: f64,
    category_id: i32,
    updated_at: Option<DateTime<Utc>>,
    version: i32,
    status: String,
    stock_quantity: i32,
}

/// Minimal product summary for listings
#[derive(Debug, Clone)]
pub struct ProductSummaryDto {
    id: Uuid,
    name: String,
    price: f64,
    status: String,
}

/// Even more minimal for list items
#[derive(Debug, Clone)]
pub struct ProductListItemDto {
    id: Uuid,
    name: String,
    price: f64,
}

/// DTO for creating new products
#[derive(Debug, Clone)]
pub struct CreateProductDto {
    name: String,
    description: Option<String>,
    price: f64,
    category_id: i32,
}

/// DTO for updating existing products
#[derive(Debug, Clone)]
pub struct UpdateProductDto {
    id: Uuid,
    name: String,
    description: Option<String>,
    new_price: Option<f64>,
    category_id: Option<i32>,
    current_version: i32,
    status: Option<String>,
    stock_adjustment: i32,
}

/// DTO for importing products from external systems
#[derive(Debug, Clone)]
pub struct ImportProductDto {
    external_id: String,
    name: String,
    description: String,
    price_cents: i64,
    category_name: String,
    created_at: Option<DateTime<Utc>>,
    initial_stock: i32,
}

fn main() {
    println!("=== Advanced Mapping Example ===\n");

    // Example 1: Create from CreateProductDto
    println!("1. Creating product from CreateProductDto:");
    let create_dto = CreateProductDto {
        name: "Wireless Headphones".to_string(),
        description: Some("High-quality wireless headphones".to_string()),
        price: 149.99,
        category_id: 1,
    };

    let product_from_create = ProductModel::from(create_dto);
    println!("   Created product: {}", product_from_create.name);
    println!("   ID: {} (auto-generated)", product_from_create.id);
    println!("   Price: ${:.2}", product_from_create.price);
    println!("   Status: {} (default)", product_from_create.status);
    println!("   Version: {} (initial)", product_from_create.version);

    // Example 2: Update from UpdateProductDto
    println!("\n2. Updating product from UpdateProductDto:");
    let update_dto = UpdateProductDto {
        id: product_from_create.id,
        name: "Premium Wireless Headphones".to_string(),
        description: None, // Will use default
        new_price: Some(179.99),
        category_id: None, // Will use default
        current_version: 1,
        status: Some("active".to_string()),
        stock_adjustment: 50,
    };

    let product_from_update = ProductModel::from(update_dto);
    println!("   Updated product: {}", product_from_update.name);
    println!("   ID: {} (preserved)", product_from_update.id);
    println!("   Price: ${:.2} (updated)", product_from_update.price);
    println!(
        "   Description: {:?} (default)",
        product_from_update.description
    );
    println!("   Status: {} (updated)", product_from_update.status);
    println!("   Version: {} (incremented)", product_from_update.version);

    // Example 3: Import from ImportProductDto
    println!("\n3. Importing product from ImportProductDto:");
    let import_dto = ImportProductDto {
        external_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        name: "Programming Book".to_string(),
        description: "Learn Rust programming".to_string(),
        price_cents: 2999, // $29.99
        category_name: "books".to_string(),
        created_at: Some(chrono::Utc::now()),
        initial_stock: 25,
    };

    let product_from_import = ProductModel::from(import_dto);
    println!("   Imported product: {}", product_from_import.name);
    println!("   ID: {} (from external)", product_from_import.id);
    println!(
        "   Price: ${:.2} (converted from cents)",
        product_from_import.price
    );
    println!(
        "   Category ID: {} (mapped from name)",
        product_from_import.category_id
    );
    println!("   Status: {} (import default)", product_from_import.status);
    println!("   Stock: {} (initial)", product_from_import.stock_quantity);

    // Example 4: Convert to different target types
    println!("\n4. Converting to different DTOs:");

    // Full DTO
    let full_dto: ProductDto = product_from_create.clone().into();
    println!("   ProductDto fields: id, name, description, price, category_id, updated_at, version, status, stock_quantity");
    println!("     Name: {}", full_dto.name);
    println!("     Price: ${:.2}", full_dto.price);
    println!("     Version: {}", full_dto.version);

    // Summary DTO (some fields skipped)
    let summary_dto: ProductSummaryDto = product_from_create.clone().into();
    println!("   ProductSummaryDto fields: id, name, price, status");
    println!("     Name: {}", summary_dto.name);
    println!("     Price: ${:.2}", summary_dto.price);
    println!("     Status: {}", summary_dto.status);

    // List item DTO (minimal fields)
    let list_item_dto: ProductListItemDto = product_from_create.into();
    println!("   ProductListItemDto fields: id, name, price");
    println!("     Name: {}", list_item_dto.name);
    println!("     Price: ${:.2}", list_item_dto.price);

    println!("\n=== Advanced example completed successfully! ===");
    println!("\nThis example demonstrates:");
    println!("- Multiple source types with different field mappings");
    println!("- Multiple target types with selective field inclusion");
    println!("- Complex expressions and transformations");
    println!("- Default value handling");
    println!("- Pattern matching in field mappings");
    println!("- Version increment logic");
    println!("- Price format conversions");
}
