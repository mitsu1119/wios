use crate::{
    linked_list::{LinkedList, LinkedListItem},
    process::Process,
};

pub struct Scheduler {
    process_list: LinkedList<Process>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            process_list: LinkedList::new(),
        }
    }

    pub fn register(&mut self, process: &mut LinkedListItem<Process>) {
        self.process_list.push(process)
    }

    pub fn remove_current_process(&mut self) -> *mut LinkedListItem<Process> {
        let current_process = self.process_list.pop_item().unwrap();
        current_process as *mut LinkedListItem<Process>
    }

    fn exec_next(&mut self) {
        let current_process = self.remove_current_process();
        self.register(unsafe { current_process.as_mut() }.unwrap());
    }

    pub fn exec(&mut self) {
        loop {
            if let Some(ps) = self.process_list.look() {
                ps.exec();
            } else {
                unimplemented!();
            }
            self.exec_next();
        }
    }
}
