use starknet::core::types::FieldElement;

pub trait IntoFieldElementVec<T: Clone> {
    fn as_field_element_vec(&self) -> Vec<FieldElement>;
}

impl<T> IntoFieldElementVec<T> for Vec<T>
where
    T: Clone,
    FieldElement: From<T>,
{
    fn as_field_element_vec(&self) -> Vec<FieldElement> {
        self.iter()
            .map(|item| FieldElement::from(item.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::FieldElement;

    #[test]
    fn test_as_field_element_vec() {
        let input = vec![1_u8, 2_u8, 3_u8];
        let expected = vec![
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(3_u8),
        ];
        assert_eq!(input.as_field_element_vec(), expected);
    }
}
