
struct MyIteratorWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIteratorWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        let (element, rest) = self.slice.split_first()?;
        self.slice = rest;

        return Some(element);
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
            println!("Elem: {}", *elem);
            assert_eq!(*elem, collection[index]);
        }
    }
}
