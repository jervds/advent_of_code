pub struct Depth {
    pub(crate) previous_depth: i32,
    pub(crate) current_depth: i32,
    pub(crate) depth_increase_count: usize,
    pub(crate) first: bool,
}

impl Depth {
    fn maybe_increase_count(self) -> Self {
        return if self.current_depth > self.previous_depth {
            Self {
                depth_increase_count: self.depth_increase_count + 1,
                ..self
            }
        } else {
            self
        };
    }

    pub(crate) fn evaluate(self, new_depth: i32) -> Self {
        if self.first == true {
            Self {
                current_depth: new_depth,
                first: false,
                ..self
            }
        } else {
            Self {
                previous_depth: self.current_depth,
                current_depth: new_depth,
                ..self
            }
            .maybe_increase_count()
        }
    }
}
