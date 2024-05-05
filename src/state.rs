use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tokio_i3ipc::reply::Node;
use tokio_i3ipc::reply::NodeType::Dockarea;

#[derive(Debug)]
pub enum SwitchMode {
    Forward,
    Backward,
}

pub struct State {
    focused_window_id: Arc<Mutex<Option<usize>>>,
    prev_window_id: Arc<Mutex<Option<usize>>>,
    next_window_id: Arc<Mutex<Option<usize>>>,
    first_window_id: Arc<Mutex<Option<usize>>>,
    last_window_id: Arc<Mutex<Option<usize>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            focused_window_id: Arc::new(Mutex::new(None)),
            prev_window_id: Arc::new(Mutex::new(None)),
            next_window_id: Arc::new(Mutex::new(None)),
            first_window_id: Arc::new(Mutex::new(None)),
            last_window_id: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_focused_window_id(&self) -> usize {
        if let Some(focused_window_id) = self
            .focused_window_id
            .lock()
            .expect("fail to acquire lock in get_focused_window_id")
            .as_ref()
        {
            *focused_window_id
        } else {
            0
        }
    }

    pub fn set_focused_window_id(&self, window_id: usize) {
        self.focused_window_id
            .lock()
            .expect("fail to acquire lock in set_focused_window_id")
            .replace(window_id);
    }

    pub fn get_first_window_id(&self) -> usize {
        if let Some(first_window_id) = self
            .first_window_id
            .lock()
            .expect("fail to acquire lock in get_first_window_id")
            .as_ref()
        {
            *first_window_id
        } else {
            0
        }
    }

    pub fn set_first_window_id(&self, window_id: usize) {
        self.first_window_id
            .lock()
            .expect("fail to acquire lock in set_first_window_id")
            .replace(window_id);
    }

    pub fn get_last_window_id(&self) -> usize {
        if let Some(last_window_id) = self
            .last_window_id
            .lock()
            .expect("fail to acquire lock in get_last_window_id")
            .as_ref()
        {
            *last_window_id
        } else {
            0
        }
    }

    pub fn set_last_window_id(&self, window_id: usize) {
        self.last_window_id
            .lock()
            .expect("fail to acquire lock in set_last_window_id")
            .replace(window_id);
    }

    pub fn get_prev_window_id(&self) -> usize {
        if let Some(prev_window_id) = self
            .prev_window_id
            .lock()
            .expect("fail to acquire lock in get_prev_window_id")
            .as_ref()
        {
            *prev_window_id
        } else {
            0
        }
    }

    pub fn set_prev_window_id(&self, window_id: usize) {
        self.prev_window_id
            .lock()
            .expect("fail to acquire lock in set_prev_window_id")
            .replace(window_id);
    }

    pub fn get_next_window_id(&self) -> usize {
        if let Some(next_window_id) = self
            .next_window_id
            .lock()
            .expect("fail to acquire lock in get_next_window_id")
            .as_ref()
        {
            *next_window_id
        } else {
            0
        }
    }

    pub fn set_next_window_id(&self, window_id: usize) {
        self.next_window_id
            .lock()
            .expect("fail to acquire lock in set_next_window_id")
            .replace(window_id);
    }
}

unsafe impl Sync for State {}

lazy_static! {
    pub static ref STATE: State = State::new();
}

pub fn collect_window_ids(node: &Node) {
    if node.node_type == Dockarea {
        return;
    }

    if node.window_properties.is_some() {
        if STATE.get_first_window_id() == 0 {
            STATE.set_first_window_id(node.id);
        }

        if node.focused {
            STATE.set_focused_window_id(node.id);
        } else {
            if STATE.get_focused_window_id() == 0 {
                STATE.set_prev_window_id(node.id);
            } else if STATE.get_next_window_id() == 0 {
                STATE.set_next_window_id(node.id);
            }
        }

        STATE.set_last_window_id(node.id);
    }
    for inner_node in &node.nodes {
        collect_window_ids(inner_node);
    }

    let mut sorted_floating_nodes = vec![];
    for inner_floating_node in &node.floating_nodes {
        sorted_floating_nodes.push((inner_floating_node.id, inner_floating_node));
    }
    sorted_floating_nodes.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, inner_floating_node) in sorted_floating_nodes {
        collect_window_ids(inner_floating_node);
    }
}
