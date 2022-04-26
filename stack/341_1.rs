struct NestedIterator {
    stack: Vec<NestedInteger>,
    next: Option<i32>,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut s = Self {
            stack: nestedList.into_iter().rev().collect::<Vec<_>>(),
            next: None,
        };
        s.advance_next();
        s
    }

    fn advance_next(&mut self) {
        while let Some(last) = self.stack.pop() {
            match last {
                NestedInteger::Int(u) => {
                    self.next = Some(u);
                    return;
                }
                NestedInteger::List(v) => {
                    self.stack.extend(v.into_iter().rev());
                }
            }
        }
        self.next = None
    }

    fn next(&mut self) -> i32 {
        let ret = self.next.unwrap();
        self.advance_next();
        ret
    }

    fn has_next(&self) -> bool {
        self.next.is_some()
    }
}