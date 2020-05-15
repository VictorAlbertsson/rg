pub type Event = dyn Fn() -> bool;

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

pub struct LayerStack {
    pub layers: Vec<Layer>,
}

impl LayerStack {
    pub fn push_layer(&mut self, layer: Layer) {
	self.layers.push(layer);
    }
    pub fn push_overlay(&mut self, layer: Layer) {
	let mut result = vec![layer];
	result.append(&mut self.layers);
	self.layers = result;
    }
    pub fn pop_layer(&mut self) {
	self.layers = self.layers.drain(1..).collect();
    }
    pub fn pop_overlay(&mut self) {
	self.layers.pop();
    }
}
