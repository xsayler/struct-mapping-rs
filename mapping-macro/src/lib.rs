//! # Mapping Macro
//!
//! A procedural macro for automatically generating `From` and `Into` trait implementations
//! with flexible field mapping and transformation capabilities.
//!
//! ## Features
//!
//! - Automatic trait implementation generation
//! - Custom field transformations with expressions
//! - Field skipping for specific types
//! - Multiple source and target type support
//! - Default value assignment
//!
//! ## Usage
//!
//! ```rust
//! use mapping_macro::Mapping;
//!
//! #[derive(Mapping)]
//! #[from(CreateUserDto)]
//! #[into(UserDto)]
//! pub struct UserModel {
//!     #[from(CreateUserDto | 42)]
//!     id: i32,
//!     name: String,
//!     #[from(CreateUserDto | 1 as i32)]
//!     version: i32,
//! }
//!
//! pub struct UserDto {
//!     id: i32,
//!     name: String,
//!     version: i32,
//! }
//!
//! pub struct CreateUserDto {
//!     name: String,
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Path};

/// Derives `From` and `Into` implementations for struct types with custom field mapping.
///
/// # Attributes
///
/// ## Struct-level attributes:
/// - `#[from(SourceType)]` - Generate a `From<SourceType>` implementation
/// - `#[into(TargetType)]` - Generate an `Into<TargetType>` implementation
///
/// ## Field-level attributes:
/// - `#[from(SourceType | expression)]` - Use custom expression for field mapping
/// - `#[into_skip(TargetType)]` - Skip field when converting to TargetType
/// - `#[from_skip(SourceType)]` - Skip field when converting from SourceType
///
/// # Examples
///
/// ## Basic usage:
/// ```
/// # use mapping_macro::Mapping;
/// #[derive(Mapping)]
/// #[from(CreateDto)]
/// #[into(ResponseDto)]
/// struct Model {
///     #[from(CreateDto | 42)]
///     id: i32,
///     name: String,
/// }
/// # struct CreateDto { name: String }
/// # struct ResponseDto { id: i32, name: String }
/// ```
///
/// ## Multiple types with field skipping:
/// ```
/// # use mapping_macro::Mapping;
/// #[derive(Mapping)]
/// #[from(CreateDto)]
/// #[into(FullDto)]
/// #[into(SummaryDto)]
/// struct Model {
///     id: i32,
///     name: String,
///     #[into_skip(SummaryDto)]
///     internal_field: String,
/// }
/// # struct CreateDto { id: i32, name: String, internal_field: String }
/// # struct FullDto { id: i32, name: String, internal_field: String }
/// # struct SummaryDto { id: i32, name: String }
/// ```
#[proc_macro_derive(Mapping, attributes(from, into, from_skip, into_skip))]
pub fn derive_mapping(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let mut from_impls = Vec::new();
    let mut into_impls = Vec::new();

    // Parse struct-level attributes
    let mut from_types = Vec::new();
    let mut into_types = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("from") {
            if let Ok(meta_list) = attr.meta.require_list() {
                for token in meta_list.tokens.clone() {
                    if let Ok(path) = syn::parse2::<Path>(token.into()) {
                        from_types.push(path);
                    }
                }
            }
        } else if attr.path().is_ident("into") {
            if let Ok(meta_list) = attr.meta.require_list() {
                for token in meta_list.tokens.clone() {
                    if let Ok(path) = syn::parse2::<Path>(token.into()) {
                        into_types.push(path);
                    }
                }
            }
        }
    }

    // Parse fields
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Mapping macro only supports structs with named fields"),
        },
        _ => panic!("Mapping macro only supports structs"),
    };

    // Generate From implementations
    for from_type in &from_types {
        let mut field_assignments = Vec::new();

        for field in fields {
            let field_name = field.ident.as_ref().unwrap();

            // Check if field should be skipped for this from_type
            let mut skip_field = false;
            for attr in &field.attrs {
                if attr.path().is_ident("from_skip") {
                    if let Ok(meta_list) = attr.meta.require_list() {
                        for token in meta_list.tokens.clone() {
                            if let Ok(path) = syn::parse2::<Path>(token.into()) {
                                if paths_equal(&path, from_type) {
                                    skip_field = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if skip_field {
                continue;
            }

            // Check for custom field mapping
            let mut custom_expr = None;
            for attr in &field.attrs {
                if attr.path().is_ident("from") {
                    if let Ok(meta_list) = attr.meta.require_list() {
                        let tokens_str = meta_list.tokens.to_string();
                        let from_type_str = path_to_string(from_type);

                        if tokens_str.contains(&from_type_str) {
                            // Parse the expression after the pipe
                            let parts: Vec<&str> = tokens_str.split('|').collect();
                            if parts.len() == 2 {
                                let expr_str = parts[1].trim();
                                if let Ok(expr) = syn::parse_str::<Expr>(expr_str) {
                                    custom_expr = Some(expr);
                                }
                            }
                        }
                    }
                }
            }

            let assignment = if let Some(expr) = custom_expr {
                quote! { #field_name: #expr }
            } else {
                quote! { #field_name: value.#field_name }
            };

            field_assignments.push(assignment);
        }

        let from_impl = quote! {
            impl From<#from_type> for #struct_name {
                fn from(value: #from_type) -> Self {
                    Self {
                        #(#field_assignments,)*
                    }
                }
            }
        };

        from_impls.push(from_impl);
    }

    // Generate Into implementations
    for into_type in &into_types {
        let mut field_assignments = Vec::new();

        for field in fields {
            let field_name = field.ident.as_ref().unwrap();

            // Check if field should be skipped for this into_type
            let mut skip_field = false;
            for attr in &field.attrs {
                if attr.path().is_ident("into_skip") {
                    if let Ok(meta_list) = attr.meta.require_list() {
                        for token in meta_list.tokens.clone() {
                            if let Ok(path) = syn::parse2::<Path>(token.into()) {
                                if paths_equal(&path, into_type) {
                                    skip_field = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if !skip_field {
                let assignment = quote! { #field_name: self.#field_name };
                field_assignments.push(assignment);
            }
        }

        let into_impl = quote! {
            impl Into<#into_type> for #struct_name {
                fn into(self) -> #into_type {
                    #into_type {
                        #(#field_assignments,)*
                    }
                }
            }
        };

        into_impls.push(into_impl);
    }

    let expanded = quote! {
        #(#from_impls)*
        #(#into_impls)*
    };

    TokenStream::from(expanded)
}

fn paths_equal(path1: &Path, path2: &Path) -> bool {
    path_to_string(path1) == path_to_string(path2)
}

fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_paths_equal() {
        let path1: Path = parse_quote!(MyStruct);
        let path2: Path = parse_quote!(MyStruct);
        let path3: Path = parse_quote!(OtherStruct);

        assert!(paths_equal(&path1, &path2));
        assert!(!paths_equal(&path1, &path3));
    }

    #[test]
    fn test_path_to_string() {
        let path1: Path = parse_quote!(MyStruct);
        let path2: Path = parse_quote!(my_module::MyStruct);

        assert_eq!(path_to_string(&path1), "MyStruct");
        assert_eq!(path_to_string(&path2), "my_module::MyStruct");
    }
}
