// TODO: Investigate traits as possible API implementation
pub struct Core<'a> {
    pub message: &'a str,
}

impl<'a> Core<'a> {
    pub fn new() -> Self {
	Core {
	    message: "test",
	}
    }

    pub fn run(self) {
	loop {
	    todo!();
	}
    }
}

pub trait Process {
    fn start();
    fn update();
}

impl<'a> Default for Core<'a> {
    fn default() -> Self {
	todo!()
    }
}
