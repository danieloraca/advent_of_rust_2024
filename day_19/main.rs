use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<State> {
    state: PhantomData<State>,
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Sleigh { state: PhantomData }
    }
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}
