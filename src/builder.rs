struct Builder {
    stack: Vec<UnCompiledNode>,
}

struct UnCompiledNode {
    last_in: u8,
    last_out: u64,
}

struct Transition {
    out: u64,
    _in: u8,
}

impl Builder {
    fn add(&self, key: &[u8], val: u64) {}

    fn find_common_prefix(&self, key: &[u8]) -> u32 {
        let mut i: usize = 0;
        while i < key.len() {
            if self.stack[i].last_in == key[i] {
                i += 1;
            }
        }
        i as u32
    }

    fn add_suffix() {}

    fn pop_freeze() {}

    fn freeze_tail(prefix_len: u32) {}

    fn compile_node() {}

    fn output_prefix() -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {}
}
