//! Tests for the mapping macro functionality

use chrono::{DateTime, Utc};
use mapping_macro::Mapping;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Mapping, Debug, PartialEq, Clone)]
    #[from(CreateUserDto)]
    #[into(UserDto)]
    pub struct UserModel {
        #[from(CreateUserDto | Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())]
        id: Uuid,
        name: String,
        email: Option<String>,
        #[from(CreateUserDto | Some(42))]
        #[into_skip(UserDto)]
        age: Option<i32>,
        #[from(CreateUserDto | 1 as i32)]
        version: i32,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct UserDto {
        id: Uuid,
        name: String,
        email: Option<String>,
        version: i32,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CreateUserDto {
        name: String,
        email: Option<String>,
    }

    #[derive(Mapping, Debug, PartialEq, Clone)]
    #[from(CreateProductDto)]
    #[from(UpdateProductDto)]
    #[into(ProductDto)]
    #[into(ProductSummaryDto)]
    pub struct ProductModel {
        #[from(CreateProductDto | Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap())]
        #[from(UpdateProductDto | value.id)]
        id: Uuid,
        name: String,
        #[from(CreateProductDto | value.price)]
        #[from(UpdateProductDto | value.price.unwrap_or(999.99))]
        price: f64,
        #[from(CreateProductDto | 1 as i32)]
        #[from(UpdateProductDto | value.version + 1)]
        #[into_skip(ProductSummaryDto)]
        version: i32,
        #[from(CreateProductDto | Some("new".to_string()))]
        #[from(UpdateProductDto | Some("updated".to_string()))]
        #[into_skip(ProductSummaryDto)]
        status: Option<String>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ProductDto {
        id: Uuid,
        name: String,
        price: f64,
        version: i32,
        status: Option<String>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ProductSummaryDto {
        id: Uuid,
        name: String,
        price: f64,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CreateProductDto {
        name: String,
        price: f64,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct UpdateProductDto {
        id: Uuid,
        name: String,
        price: Option<f64>,
        version: i32,
    }

    #[test]
    fn test_user_from_create_dto() {
        let create_dto = CreateUserDto {
            name: "Alice".to_string(),
            email: Some("alice@example.com".to_string()),
        };

        let user_model = UserModel::from(create_dto);

        assert_eq!(
            user_model.id,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
        );
        assert_eq!(user_model.name, "Alice");
        assert_eq!(user_model.email, Some("alice@example.com".to_string()));
        assert_eq!(user_model.age, Some(42));
        assert_eq!(user_model.version, 1);
    }

    #[test]
    fn test_user_from_create_dto_with_none_email() {
        let create_dto = CreateUserDto {
            name: "Bob".to_string(),
            email: None,
        };

        let user_model = UserModel::from(create_dto);

        assert_eq!(user_model.name, "Bob");
        assert_eq!(user_model.email, None);
        assert_eq!(user_model.age, Some(42));
        assert_eq!(user_model.version, 1);
    }

    #[test]
    fn test_user_into_dto() {
        let user_model = UserModel {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            name: "Charlie".to_string(),
            email: Some("charlie@example.com".to_string()),
            age: Some(30),
            version: 2,
        };

        let user_dto: UserDto = user_model.into();

        assert_eq!(
            user_dto.id,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
        );
        assert_eq!(user_dto.name, "Charlie");
        assert_eq!(user_dto.email, Some("charlie@example.com".to_string()));
        assert_eq!(user_dto.version, 2);
        // Note: age field is skipped in UserDto
    }

    #[test]
    fn test_product_from_create_dto() {
        let create_dto = CreateProductDto {
            name: "Laptop".to_string(),
            price: 1299.99,
        };

        let product_model = ProductModel::from(create_dto);

        assert_eq!(
            product_model.id,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap()
        );
        assert_eq!(product_model.name, "Laptop");
        assert_eq!(product_model.price, 1299.99);
        assert_eq!(product_model.version, 1);
        assert_eq!(product_model.status, Some("new".to_string()));
    }

    #[test]
    fn test_product_from_update_dto() {
        let update_dto = UpdateProductDto {
            id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            name: "Gaming Laptop".to_string(),
            price: Some(1599.99),
            version: 2,
        };

        let product_model = ProductModel::from(update_dto);

        assert_eq!(
            product_model.id,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
        );
        assert_eq!(product_model.name, "Gaming Laptop");
        assert_eq!(product_model.price, 1599.99);
        assert_eq!(product_model.version, 3); // version + 1
        assert_eq!(product_model.status, Some("updated".to_string()));
    }

    #[test]
    fn test_product_from_update_dto_with_none_price() {
        let update_dto = UpdateProductDto {
            id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            name: "Budget Laptop".to_string(),
            price: None, // Should default to 999.99
            version: 1,
        };

        let product_model = ProductModel::from(update_dto);

        assert_eq!(product_model.name, "Budget Laptop");
        assert_eq!(product_model.price, 999.99); // Default value
        assert_eq!(product_model.version, 2); // version + 1
    }

    #[test]
    fn test_product_into_dto() {
        let product_model = ProductModel {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            name: "Smartphone".to_string(),
            price: 699.99,
            version: 3,
            status: Some("active".to_string()),
        };

        let product_dto: ProductDto = product_model.into();

        assert_eq!(
            product_dto.id,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap()
        );
        assert_eq!(product_dto.name, "Smartphone");
        assert_eq!(product_dto.price, 699.99);
        assert_eq!(product_dto.version, 3);
        assert_eq!(product_dto.status, Some("active".to_string()));
    }

    #[test]
    fn test_product_into_summary_dto() {
        let product_model = ProductModel {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            name: "Tablet".to_string(),
            price: 399.99,
            version: 5,
            status: Some("active".to_string()),
        };

        let summary_dto: ProductSummaryDto = product_model.into();

        assert_eq!(
            summary_dto.id,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap()
        );
        assert_eq!(summary_dto.name, "Tablet");
        assert_eq!(summary_dto.price, 399.99);
        // Note: version and status fields are skipped in ProductSummaryDto
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_create_dto = CreateUserDto {
            name: "Eve".to_string(),
            email: Some("eve@example.com".to_string()),
        };

        // CreateUserDto -> UserModel -> UserDto
        let user_model = UserModel::from(original_create_dto.clone());
        let user_dto: UserDto = user_model.into();

        assert_eq!(user_dto.name, original_create_dto.name);
        assert_eq!(user_dto.email, original_create_dto.email);
        assert_eq!(user_dto.version, 1);
    }

    #[test]
    fn test_multiple_conversions_same_struct() {
        let create_dto = CreateProductDto {
            name: "Monitor".to_string(),
            price: 299.99,
        };

        let product_model = ProductModel::from(create_dto);

        // Convert to both target types
        let full_dto: ProductDto = product_model.clone().into();
        let summary_dto: ProductSummaryDto = product_model.into();

        // Both should have the same basic fields
        assert_eq!(full_dto.name, summary_dto.name);
        assert_eq!(full_dto.price, summary_dto.price);
        assert_eq!(full_dto.id, summary_dto.id);

        // But full_dto has additional fields
        assert_eq!(full_dto.version, 1);
        assert_eq!(full_dto.status, Some("new".to_string()));
    }
}
