#![allow(dead_code)]

/// A fixed size array of 8 elements. If more than 8 elements are added, a heal-allocated array of
/// any size will be switched to instead. (aka fixed-size up to 8 elements).
#[derive(Debug, Clone)]
pub enum DynamicArray8<T: Copy + PartialEq> {
    // Fixed size array of up to 8 elements, along with a count.
    Fixed([T; 8], u8),

    // A heap-allocated array of any number of elements.
    Dynamic(Vec<T>),
}

impl<T: Copy + PartialEq> DynamicArray8<T> {
    pub fn new() -> Self {
        Self::Fixed(unsafe { std::mem::uninitialized() }, 0)
    }

    pub fn with(element: T) -> Self {
        let mut arr = Self::new();
        arr.push(element);
        arr
    }

    pub fn push(&mut self, element: T) {
        if let Self::Fixed(array, count) = self {
            if *count < 8 {
                array[*count as usize] = element;
                *count += 1;
            } else {
                // Switch to a dynamic array
                let mut vector: Vec<_> = array.iter().cloned().collect();
                vector.push(element);
                *self = Self::Dynamic(vector);
            }
        } else if let Self::Dynamic(vec) = self {
            vec.push(element);
        }
    }

    pub fn remove_element(&mut self, element: &T) {
        if let Self::Fixed(array, count) = self {
            if let Some(index) = array[..*count as usize].iter().position(|e| e == element) {
                // Swap-remove with last element.
                *count -= 1;
                array.swap(index, *count as usize);
            }
        } else if let Self::Dynamic(vec) = self {
            if let Some(index) = vec.iter().position(|e| e == element) {
                vec.remove(index);
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Fixed(_, count) => *count as usize,
            Self::Dynamic(vec) => vec.len(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        match self {
            Self::Fixed(array, count) => array[..*count as usize].iter(),
            Self::Dynamic(vec) => vec.as_slice().iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_len_iter() {
        let mut dynamic_array = DynamicArray8::new();

        // Will cause an overflow after the first 8
        for i in 0..16 {
            dynamic_array.push(i);
            assert_eq!(dynamic_array.len(), i + 1);
            assert_eq!(
                dynamic_array.iter().cloned().collect::<Vec<_>>(),
                (0..i + 1).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn remove_len_iter() {
        let mut dynamic_array = DynamicArray8::new();

        // Will not overflow
        let count = 6_usize;
        for i in 0..count {
            dynamic_array.push(i);
        }

        assert_eq!(dynamic_array.len(), count);

        for i in (0..count).rev() {
            dynamic_array.remove_element(&i);
            assert_eq!(dynamic_array.len(), i);
            assert_eq!(
                dynamic_array.iter().cloned().collect::<Vec<_>>(),
                (0..i).collect::<Vec<_>>()
            );
        }

        assert_eq!(dynamic_array.len(), 0);

        // Will overflow
        let count = 16_usize;
        for i in 0..count {
            dynamic_array.push(i);
        }

        assert_eq!(dynamic_array.len(), count);

        for i in (0..count).rev() {
            dynamic_array.remove_element(&i);
            assert_eq!(dynamic_array.len(), i);
            assert_eq!(
                dynamic_array.iter().cloned().collect::<Vec<_>>(),
                (0..i).collect::<Vec<_>>()
            );
        }

        assert_eq!(dynamic_array.len(), 0);
    }
}
