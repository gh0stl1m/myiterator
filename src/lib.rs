
struct MyIteratorWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIteratorWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        if self.slice.is_empty() {
            return None;
        }

        let element = self.slice.get(0);
        self.slice = &self.slice[1..];

        return Some(element.unwrap());
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let collection = vec![1,2,3,4];
        let wrapper = MyIteratorWrapper{
            slice: &collection[..]
        };

        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);

        }
    }
}
