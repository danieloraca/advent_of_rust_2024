use std::marker::PhantomData;

// State structs
pub struct Empty;
pub struct Ready;
pub struct Flying;

// Define a SleighState trait to enable state-specific behavior
pub trait SleighState {
    fn status() -> &'static str;
}

// Implement SleighState for each state
impl SleighState for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}

impl SleighState for Ready {
    fn status() -> &'static str {
        "Ready"
    }
}

impl SleighState for Flying {
    fn status() -> &'static str {
        "Flying"
    }
}

// The Sleigh struct with a generic state parameter
pub struct Sleigh<T: SleighState> {
    state: PhantomData<T>,
}

// Implement methods for the Empty state
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

// Implement methods for the Ready state
impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

// Implement methods for the Flying state
impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

// Provide a universal `status` method for all Sleigh states
impl<T: SleighState> Sleigh<T> {
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

// Example usage
pub fn main() {
    let sleigh = Sleigh::<Empty>::new();
    assert_eq!(sleigh.status(), "Empty");

    let ready_sleigh = sleigh.load();
    assert_eq!(ready_sleigh.status(), "Ready");

    let flying_sleigh = ready_sleigh.take_off();
    assert_eq!(flying_sleigh.status(), "Flying");

    let ready_sleigh_again = flying_sleigh.land();
    assert_eq!(ready_sleigh_again.status(), "Ready");

    let empty_sleigh = ready_sleigh_again.unload();
    assert_eq!(empty_sleigh.status(), "Empty");
}
