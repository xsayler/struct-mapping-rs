//! Advanced usage example showing multiple source and target types

use chrono::{DateTime, Utc};
use mapping_macro::Mapping;
use uuid::Uuid;

/// Order model that can be created from multiple sources
#[derive(Mapping, Debug, Clone)]
#[from(CreateOrderDto)]
#[from(UpdateOrderDto)]
#[into(OrderDto)]
#[into(OrderSummaryDto)]
pub struct OrderModel {
    /// ID generation varies by source
    #[from(CreateOrderDto | Uuid::new_v4())]
    #[from(UpdateOrderDto | value.id)]
    id: Uuid,

    /// Customer ID is mapped directly
    customer_id: Uuid,

    /// Total amount with different sources
    #[from(CreateOrderDto | value.amount)]
    #[from(UpdateOrderDto | value.new_amount.unwrap_or(0.0))]
    total_amount: f64,

    /// Status with defaults
    #[from(CreateOrderDto | "pending".to_string())]
    #[from(UpdateOrderDto | value.status)]
    #[into_skip(OrderSummaryDto)]
    status: String,

    /// Created timestamp
    #[from(CreateOrderDto | Some(chrono::Utc::now()))]
    #[from(UpdateOrderDto | value.updated_at)]
    #[into_skip(OrderSummaryDto)]
    created_at: Option<DateTime<Utc>>,

    /// Version tracking
    #[from(CreateOrderDto | 1 as i32)]
    #[from(UpdateOrderDto | value.version + 1)]
    #[into_skip(OrderSummaryDto)]
    version: i32,
}

/// Complete order DTO
#[derive(Debug, Clone)]
pub struct OrderDto {
    id: Uuid,
    customer_id: Uuid,
    total_amount: f64,
    status: String,
    created_at: Option<DateTime<Utc>>,
    version: i32,
}

/// Summary order DTO with minimal fields
#[derive(Debug, Clone)]
pub struct OrderSummaryDto {
    id: Uuid,
    customer_id: Uuid,
    total_amount: f64,
}

/// DTO for creating new orders
#[derive(Debug, Clone)]
pub struct CreateOrderDto {
    customer_id: Uuid,
    amount: f64,
}

/// DTO for updating existing orders
#[derive(Debug, Clone)]
pub struct UpdateOrderDto {
    id: Uuid,
    customer_id: Uuid,
    new_amount: Option<f64>,
    status: String,
    updated_at: Option<DateTime<Utc>>,
    version: i32,
}

fn main() {
    println!("=== Advanced Mapping Example ===\n");

    let customer_id = Uuid::new_v4();

    // Example 1: Create order from CreateOrderDto
    println!("1. Creating order from CreateOrderDto:");
    let create_dto = CreateOrderDto {
        customer_id,
        amount: 99.99,
    };

    let order_from_create = OrderModel::from(create_dto);
    println!("   Order ID: {} (auto-generated)", order_from_create.id);
    println!("   Customer: {}", order_from_create.customer_id);
    println!("   Amount: ${:.2}", order_from_create.total_amount);
    println!("   Status: {} (default)", order_from_create.status);
    println!("   Version: {} (initial)", order_from_create.version);

    // Example 2: Update order from UpdateOrderDto
    println!("\n2. Updating order from UpdateOrderDto:");
    let update_dto = UpdateOrderDto {
        id: order_from_create.id,
        customer_id,
        new_amount: Some(129.99),
        status: "confirmed".to_string(),
        updated_at: Some(chrono::Utc::now()),
        version: 1,
    };

    let order_from_update = OrderModel::from(update_dto);
    println!("   Order ID: {} (preserved)", order_from_update.id);
    println!(
        "   Amount: ${:.2} (updated)",
        order_from_update.total_amount
    );
    println!("   Status: {} (updated)", order_from_update.status);
    println!("   Version: {} (incremented)", order_from_update.version);

    // Example 3: Update with None amount (uses default)
    println!("\n3. Updating with None amount:");
    let update_dto_none = UpdateOrderDto {
        id: Uuid::new_v4(),
        customer_id,
        new_amount: None, // Will default to 0.0
        status: "cancelled".to_string(),
        updated_at: Some(chrono::Utc::now()),
        version: 2,
    };

    let order_with_default = OrderModel::from(update_dto_none);
    println!(
        "   Amount: ${:.2} (defaulted)",
        order_with_default.total_amount
    );
    println!("   Status: {}", order_with_default.status);

    // Example 4: Convert to different target types
    println!("\n4. Converting to different DTOs:");

    // Full DTO
    let full_dto: OrderDto = order_from_create.clone().into();
    println!("   OrderDto (complete):");
    println!("     Amount: ${:.2}", full_dto.total_amount);
    println!("     Status: {}", full_dto.status);
    println!("     Version: {}", full_dto.version);

    // Summary DTO (fields skipped)
    let summary_dto: OrderSummaryDto = order_from_create.into();
    println!("   OrderSummaryDto (minimal - status, created_at, version skipped):");
    println!("     Customer: {}", summary_dto.customer_id);
    println!("     Amount: ${:.2}", summary_dto.total_amount);

    println!("\n=== Advanced example completed! ===");
    println!("\nFeatures demonstrated:");
    println!("- Multiple From implementations with different field mappings");
    println!("- Multiple Into implementations with selective field inclusion");
    println!("- Default value assignment with unwrap_or");
    println!("- Version increment logic");
    println!("- Field preservation vs generation");
    println!("- Timestamp handling");
    println!("- UUID generation and reuse");
}
