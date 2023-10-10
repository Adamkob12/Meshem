#[derive(Debug, PartialEq)]
struct Run {
    value: bool,
    length: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) struct RleVec {
    data: Vec<Run>,
}

impl RleVec {
    pub fn new() -> Self {
        RleVec { data: Vec::new() }
    }

    pub fn push(&mut self, value: bool, length: usize) {
        match self.data.last_mut() {
            Some(last_run) if last_run.value == value => {
                last_run.length += length;
            }
            _ => {
                self.data.push(Run { value, length });
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        let mut run_start = 0;
        for run in &self.data {
            if index < run_start + run.length {
                return Some(run.value);
            }
            run_start += run.length;
        }
        None
    }

    pub fn change(&mut self, index: usize, value: bool) -> Result<(), &'static str> {
        let mut run_start = 0;
        for run in &mut self.data {
            if index < run_start + run.length {
                if run.length == 1 {
                    run.value = value;
                } else {
                    let offset = index - run_start;
                    if offset == 0 {
                        run.length -= 1;
                        self.data.insert(index, Run { value, length: 1 });
                    } else if offset == run.length - 1 {
                        run.length -= 1;
                        self.data.insert(index + 1, Run { value, length: 1 });
                    } else {
                        let new_run = Run {
                            value,
                            length: run.length - offset - 1,
                        };
                        run.length = offset;
                        self.data.insert(index + 1, Run { value, length: 1 });
                        self.data.insert(index + 2, new_run);
                    }
                }
                return Ok(());
            }
            run_start += run.length;
        }
        Err("Index out of bounds")
    }
}
