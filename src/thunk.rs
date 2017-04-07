pub trait Thunk {
    type Item;
    
    fn eval(self) -> Self::Item;
    
    fn map<U, F: FnOnce(Self::Item) -> U>(self, f: F) -> Map<Self, F> 
        where Self: Sized,
    {
        Map {
            thunk: self,
            transformation: f,
        }
    }
    
    fn combine<T: Thunk, U, F: FnOnce(Self::Item, T::Item) -> U>(self, t: T, f: F) -> Combine<Self, T, F>
        where Self: Sized,
    {
        Combine {
            t1: self,
            t2: t,
            combinator: f
        }
    }
}

pub struct Combine<T, U, F> {
    t1: T,
    t2: U,
    combinator: F,
}
    
impl<T, U, V, F> Thunk for Combine<T, U, F>
where T: Thunk,
      U: Thunk,
      F: FnOnce(T::Item, U::Item) -> V,
{
    type Item = V;
    
    fn eval(self) -> Self::Item {
        (self.combinator)(self.t1.eval(), self.t2.eval())
    }
}

pub struct ThunkOnce<T, F>
where F: FnOnce() -> T {
    prod: F,
}

impl<T, F> ThunkOnce<T, F>
where F: FnOnce() -> T {
    pub fn new(f: F) -> Self {
        ThunkOnce {
            prod: f,
        }
    }
}

impl<T, F> Thunk for ThunkOnce<T, F>
where F: FnOnce() -> T {
    type Item = T;
    
    fn eval(self) -> Self::Item {
        (self.prod)()
    }
}

pub struct Map<T, F> {
    transformation: F,
    thunk: T,
}

impl<T, U, F> Thunk for Map<T, F>
where T: Thunk,
      F: FnOnce(T::Item) -> U {
    type Item = U;
    
    fn eval(self) -> Self::Item {
        (self.transformation)(self.thunk.eval())
    }
}
