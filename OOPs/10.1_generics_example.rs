use itertools::Itertools;

trait Prefix<T> {
    //&[T] --> a reference to a slice of type T
    fn has_prefix(&self, prefix: &[T]) -> bool;
}
//implementation of prefix trait for slice of type T where type T implements PartialEq
impl<T> Prefix<T> for &[T]
where
    T: PartialEq,
{   
    //&self represents the particular type we are implementing this trait for
    //hence &self here represents [T]
    //the prefix could have been theoritically anything; but here, it is &[T]
    fn has_prefix(&self, prefix: &[T]) -> bool {
        self.iter()
            .positions(|v| *v == prefix[0])
            .find(|&index| {
                let range = index..(index + prefix.len());
                self[range] == *prefix
            })
            .is_some()
    }
}

fn main() {
    let integers = vec![1,2,3,4,5,6,7,8,9,10];
    let integer_result = integers.as_slice().has_prefix(&[3,4,5]);
    dbg!(integer_result); 

    let floats = vec![1.0, 4.0, 5.0, 7.0];
    let float_result = floats.as_slice().has_prefix(&[5.2, 7.0]);
    dbg!(float_result);
}