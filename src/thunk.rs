pub trait ThunkOwned {
    type Item;
    fn eval_once(self) -> Self::Item;

    fn map<O, U: FnOnce(Self::Item) -> O>(self, f: U) -> Map<Self, Self::Item, U, O>
    where Self: Sized {
        Map {
            thunk: self,
            transform: f,
        }
    }
}

pub trait Thunk : ThunkOwned {
    fn eval(&mut self) -> Self::Item;
}

pub trait ThunkRef : Thunk {
    fn eval_ref(&mut self) -> &Self::Item;
}

/*
pub trait ThunkMut : ThunkRef {
    fn mutate<F: FnOnce(Self::Item) -> Self::Item>(&mut self, f: F);
}
 */

pub struct ThunkOnce<T, F>
    where F: FnOnce() -> T {
    gen: F,
}

impl<T, F> ThunkOnce<T, F>
    where F: FnOnce() -> T {
    pub fn new(f: F) -> Self {
        ThunkOnce {
            gen: f,
        }
    }
}

impl<T, F> ThunkOwned for ThunkOnce<T, F>
    where F: FnOnce() -> T, T: Sized {
    type Item = T;

    fn eval_once(self) -> Self::Item {
        (self.gen)()
    }
}

pub struct ThunkCached<T, F>
    where F: FnOnce() -> T, T: Sized {
    gen: Option<F>,
    res: Option<T>,
}

impl<T, F> ThunkCached<T, F>
    where F: FnOnce() -> T, T: Sized {
    pub fn new(f: F) -> Self {
        ThunkCached {
            gen: Some(f),
            res: None,
        }
    }
}


impl<T, F> ThunkOwned for ThunkCached<T, F>
    where F: FnOnce() -> T, T: Sized + Copy {
    type Item = T;

    fn eval_once(mut self) -> Self::Item {
        self.eval()
    }
}

impl<T, F> Thunk for ThunkCached<T, F>
    where F: FnOnce() -> T, T: Sized + Copy {

    fn eval(&mut self) -> Self::Item {
        match self.gen.take() {
            Some(f) => self.res = Some(f()),
            None => (),
        };

        self.res.unwrap()
    }
}

impl<T, F> ThunkRef for ThunkCached<T, F>
    where F: FnOnce() -> T, T: Sized + Copy {

    fn eval_ref(&mut self) -> &Self::Item {
        match self.gen.take() {
            Some(f) => self.res = Some(f()),
            None => (),
        };

        self.res.as_ref().unwrap()
    }
}

pub struct Map<H, T, U, O>
    where H: ThunkOwned<Item = T>, U: FnOnce(H::Item) -> O {
    thunk: H,
    transform: U,
}

impl<H, T, U, O> ThunkOwned for Map<H, T, U, O>
    where H: ThunkOwned<Item = T>, U: FnOnce(H::Item) -> O {
    type Item = O;

    fn eval_once(self) -> Self::Item {
        (self.transform)(self.thunk.eval_once())
    }
}
