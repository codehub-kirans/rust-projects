//! Use the factory method to create data structures in Rust
//! The factory method is a creational design pattern that creates a value that implements an interface from the input argument.
//! Imagine we have a trait implementing a product, and we want to return a different implementation of this trait based on the function argument.
//! It is where the factory method is helpful. In Rust, we can solve this by using trait objects and dynamically returning an object according to the input argument.
//! If we know the input argument at compile time, we could solve this without dynamic dispatch and trait objects using static dispatch.
pub trait ProductTrait {
    fn operation(&self) -> String;
}

struct Product1;
struct Product2;

#[allow(dead_code)]
enum ProductType {
    Product1,
    Product2,
}

impl ProductTrait for Product1 {
    fn operation(&self) -> String {
        "Invoking Product1's operation".into()
    }
}

impl ProductTrait for Product2 {
    fn operation(&self) -> String {
        "Invoking Product2's operation".into()
    }
}

#[allow(dead_code)]
fn product_factory(product_type: ProductType) -> Box<dyn ProductTrait> {
    match product_type {
        ProductType::Product1 => Box::new(Product1),
        ProductType::Product2 => Box::new(Product2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        assert_eq!(
            product_factory(ProductType::Product1).operation(),
            "Invoking Product1's operation"
        );
        assert_eq!(
            product_factory(ProductType::Product2).operation(),
            "Invoking Product2's operation"
        );
    }
}
