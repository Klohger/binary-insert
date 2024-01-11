#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

pub trait BinaryInsert<T: Ord> {
    fn binary_insert(&mut self, val: T) -> Result<(), std::collections::TryReserveError>;
}
pub trait BinaryInsertBy<T> {
    fn binary_insert_by<F: Fn(&T, &T) -> std::cmp::Ordering>(
        &mut self,
        val: T,
        cmp: F,
    ) -> Result<(), std::collections::TryReserveError>;
}
#[cfg(nightly)]
impl<T: Ord, A: std::alloc::Allocator> BinaryInsert<T> for Vec<T, A> {
    fn binary_insert(&mut self, val: T) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search(&val) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(nightly)]
impl<T, A: std::alloc::Allocator> BinaryInsertBy<T> for Vec<T, A> {
    fn binary_insert_by<F: Fn(&T, &T) -> std::cmp::Ordering>(
        &mut self,
        val: T,
        cmp: F,
    ) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search_by({
            let val = &val;
            move |rhs| cmp(&val, rhs)
        }) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(nightly)]
impl<T: Ord, A: std::alloc::Allocator> BinaryInsert<T> for std::collections::VecDeque<T, A> {
    fn binary_insert(&mut self, val: T) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search(&val) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(nightly)]
impl<T, A: std::alloc::Allocator> BinaryInsertBy<T> for std::collections::VecDeque<T, A> {
    fn binary_insert_by<F: Fn(&T, &T) -> std::cmp::Ordering>(
        &mut self,
        val: T,
        cmp: F,
    ) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search_by({
            let val = &val;
            move |rhs| cmp(&val, rhs)
        }) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(not(nightly))]
impl<T: Ord> BinaryInsert<T> for Vec<T> {
    fn binary_insert(&mut self, val: T) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search(&val) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(not(nightly))]
impl<T> BinaryInsertBy<T> for Vec<T> {
    fn binary_insert_by<F: Fn(&T, &T) -> std::cmp::Ordering>(
        &mut self,
        val: T,
        cmp: F,
    ) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search_by({
            let val = &val;
            move |rhs| cmp(&val, rhs)
        }) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(not(nightly))]
impl<T: Ord> BinaryInsert<T> for std::collections::VecDeque<T> {
    fn binary_insert(&mut self, val: T) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search(&val) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
#[cfg(not(nightly))]
impl<T> BinaryInsertBy<T> for std::collections::VecDeque<T> {
    fn binary_insert_by<F: Fn(&T, &T) -> std::cmp::Ordering>(
        &mut self,
        val: T,
        cmp: F,
    ) -> Result<(), std::collections::TryReserveError> {
        let i = match self.binary_search_by({
            let val = &val;
            move |rhs| cmp(&val, rhs)
        }) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.try_reserve(1)?;
        self.insert(i, val);
        Ok(())
    }
}
