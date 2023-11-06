use starknet::core::types::FieldElement;

pub trait FieldElementVecExt {
    fn extend_with_len(&mut self, other: &[FieldElement]);
}

impl FieldElementVecExt for Vec<FieldElement> {
    fn extend_with_len(&mut self, other: &[FieldElement]) {
        self.push(FieldElement::from(other.len()));
        self.extend(other.clone());
    }
}

pub trait ToFieldElementVec {
    fn to_felt_vec(&self) -> Vec<FieldElement>;
}

impl<T> ToFieldElementVec for Vec<T>
where
    T: Clone,
    FieldElement: From<T>,
{
    fn to_felt_vec(&self) -> Vec<FieldElement> {
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
    fn test_to_felt_vec() {
        let input = vec![1_u8, 2_u8, 3_u8];
        let expected = vec![
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(3_u8),
        ];
        assert_eq!(input.to_felt_vec(), expected);
    }

    #[test]
    fn test_field_element_vec_ext() {
        let mut vec = vec![FieldElement::from(1_u8), FieldElement::from(2_u8)];
        let other = vec![FieldElement::from(3_u8), FieldElement::from(4_u8)];
        let expected = vec![
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(2_u8), // Length of `other` vec
            FieldElement::from(3_u8),
            FieldElement::from(4_u8),
        ];
        vec.extend_with_len(&other);
        assert_eq!(vec, expected);
    }

    #[test]
    fn test_extend_with_len_empty_vec() {
        let mut vec = vec![FieldElement::from(1_u8), FieldElement::from(2_u8)];
        let other: Vec<FieldElement> = vec![];
        let expected = vec![
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(0_u8), // Length of an empty `other` vec
        ];
        vec.extend_with_len(&other);
        assert_eq!(vec, expected);
    }
}
