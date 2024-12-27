use std::marker::PhantomData;
use meshbuzz_base::Hardware;

pub struct Timelet<H: Hardware>{
    _hardware: PhantomData<H>,
}

impl<H: Hardware> Timelet<H> {
    pub fn new() -> Self {
        Timelet {
            _hardware: PhantomData,
        }
    }
    pub fn run(&self) {
        H::log_message("Timelet is running");
    }
}