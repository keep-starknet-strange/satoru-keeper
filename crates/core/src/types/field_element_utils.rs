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
