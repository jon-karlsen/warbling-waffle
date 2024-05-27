use std::sync::Mutex;


pub struct AppState {
    pub todo_list: Mutex<Vec<String>>
}


impl AppState {
    pub fn new( todo_list: Mutex<Vec<String>> ) -> Self {
        Self { todo_list }
    }
}
