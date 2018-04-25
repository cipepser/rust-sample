use std::ops::Deref;

struct DarefExample<T> {
    value: T,
}

impl<T> Deref for DarefExample<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.value
    }
}

fn main() {
    let x = DarefExample { value: 'a' };
    assert_eq!('a', *x);
}