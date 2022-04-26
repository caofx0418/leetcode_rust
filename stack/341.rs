struct NestedIterator {
    vec: Vec<i32>,
    index: usize,
}

fn dfs(nestedList: Vec<NestedInteger>, vec: &mut Vec<i32>) {
    for item in nestedList {
        match item {
            NestedInteger::Int(u) => vec.push(u),
            NestedInteger::List(v) => {
                dfs(v, vec);
            }
        }
    }
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut vec = Vec::new();
        dfs(nestedList, &mut vec);
        return NestedIterator {
            vec,
            index: 0,
        };
    }

    fn next(&mut self) -> i32 {
        let res = self.vec[self.index];
        self.index += 1;
        res
    }

    fn has_next(&self) -> bool {
        self.index < self.vec.len()
    }
}