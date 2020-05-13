pub type Event = dyn Fn() -> bool;

pub struct LayerStack {
    pub layers: Vec<Layer>,
}

impl LayerStack {
    pub fn push_layer() {}
    pub fn push_overlay() {}
    pub fn pop_layer() {}
    pub fn pop_overlay() {}
}

pub struct Layer {
    pub name: String,
    pub events: Vec<Box<Event>>,
}

impl Layer {
    pub fn attach(&self) {}
    pub fn detach(&self) {}
    pub fn poll_front(&self) -> Vec<bool> {
	self.events
	    .iter()
	    .map_while(|event| if !event() {
		Some(false)
	    } else {
		None
	    })
	    .collect()
    }
    pub fn poll_final(&self) -> Vec<bool> {
	self.events
	    .iter()
	    .rev()
	    .map_while(|event| if !event() {
		Some(false)
	    } else {
		None
	    })
	    .collect()
    }
}
