use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Thunk<A, F>
where A: Sized, F: FnOnce() -> A {
    provider: Option<F>,
    value: Option<A>,
}

impl<A, F> Thunk<A, F>
where A: Sized, F: FnOnce() -> A {
    pub fn new(f: F) -> Self {
        Thunk {
            provider: Some(f),
            value: None,
        }
    }
    
    pub fn eval(&mut self) -> &A {
        match self.provider.take() {
            Some(f) => self.value = Some(f()),
            None => (),
        };
        
        self.value.as_ref().unwrap()
    }
}

pub fn add<'a, A, F>(a: &'a Thunk<A, F>, b: Thunk<A, F>) -> Thunk<A::Output, impl FnOnce() -> A::Output>
where A: Sized + Add, F: FnOnce() -> A {
    let f = |&'mut 
    Thunk::new(|| {
        *a.eval() + *b.eval()
    })
}

