use std::fmt;

pub type WorryLevel = u64;
pub type MonkeyIndex = usize;
pub type InspectionFunction = Box<dyn Fn(WorryLevel) -> WorryLevel>;
pub type TestFunction = Box<dyn Fn(WorryLevel) -> bool>;

pub struct TestBehavior {
    check: TestFunction,
    pass_dest: MonkeyIndex,
    fail_dest: MonkeyIndex,
}

impl TestBehavior {
    pub fn new(check: TestFunction, pass_dest: MonkeyIndex, fail_dest: MonkeyIndex) -> Self {
        Self {
            check,
            pass_dest,
            fail_dest,
        }
    }
}

pub struct Monkey {
    items: Vec<WorryLevel>,
    inspect: InspectionFunction,
    test: TestBehavior,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Monkey with Items: {:?}", self.items)
    }
}

impl Monkey {
    pub fn inspect_item(&self, worry: WorryLevel) -> WorryLevel {
        (*self.inspect)(worry)
    }

    pub fn test_anxiety(&self, item: WorryLevel) -> MonkeyIndex {
        // returns index of destination monkey.
        if (*self.test.check)(item) {
            self.test.pass_dest
        } else {
            self.test.fail_dest
        }
    }

    pub fn throw_current_item_to(&mut self, other: &mut Monkey) {
        if let Some(item) = self.items.pop() {
            other.items.insert(0, item);
        } else {
            panic!("this monkey has no items!");
        }
    }
}

pub struct MonkeyFactory {
    items: Vec<WorryLevel>,
    inspect: Option<InspectionFunction>,
    test: Option<TestBehavior>,
}

impl MonkeyFactory {
    pub fn initialize() -> Self {
        Self {
            items: Vec::new(),
            inspect: None,
            test: None,
        }
    }

    pub fn with_items(mut self, items: &[WorryLevel]) -> Self {
        self.items = items.to_vec();
        self
    }

    pub fn with_inspection(mut self, inspection_function: InspectionFunction) -> Self {
        self.inspect = Some(inspection_function);
        self
    }

    pub fn with_test_behavior(mut self, behavior: TestBehavior) -> Self {
        self.test = Some(behavior);
        self
    }

    pub fn spawn_monkey(self) -> Option<Monkey> {
        let inspection_function = self.inspect?;
        let test_function = self.test?;
        Some(Monkey {
            items: self.items.clone(),
            inspect: inspection_function,
            test: test_function,
        })
    }
}
