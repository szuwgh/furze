use crate::encoder::Encoder;
use std::io::Write;
struct UnCompiledNodes {
    stack: Vec<UnCompiledNode>,
}

impl UnCompiledNodes {
    fn new() -> UnCompiledNodes {
        let stack: Vec<UnCompiledNode> = Vec::with_capacity(64);
        Self { stack: stack }
    }

    fn print(&self) {
        for v in self.stack.iter() {
            println!("");
            v.print();
            // println!("last_in:{}, last_out:{}", v.last_in as char, v.last_out);
        }
    }

    fn find_common_prefix(&mut self, key: &[u8], mut out: u64) -> (usize, u64) {
        let mut i: usize = 0;
        while i < key.len() {
            if self.stack[i].last_in() == key[i] {
                let common_pre = Self::output_prefix(self.stack[i].last_out(), out);
                out = Self::output_sub(out, common_pre);
                self.stack[i].set_last_out(common_pre);
                i += 1;
            } else {
                break;
            }
        }
        (i, out)
    }

    fn push_empty(&mut self) {
        self.stack.push(UnCompiledNode::new());
    }

    fn pop_empty(&mut self) {
        self.stack.pop();
    }

    fn pop_freeze(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    fn add_suffix(&mut self, key: &[u8], out: u64) {
        if key.len() == 0 {
            return;
        }
        let last = self.stack.len() - 1;
        self.stack[last].push_arc(Arc::new(key[0], out));
        //self.stack[last].last_out = out;
        for v in &key[1..] {
            let mut next = UnCompiledNode::new();
            next.push_arc(Arc::new(*v, 0));
            // next.last_in = *v;
            self.stack.push(next);
        }
        self.push_empty();
    }

    fn output_prefix(l: u64, r: u64) -> u64 {
        if l < r {
            return l;
        }
        r
    }

    fn output_sub(l: u64, r: u64) -> u64 {
        l - r
    }

    fn output_cat(l: u64, r: u64) -> u64 {
        l + r
    }
}

struct UnCompiledNode {
    // last_in: u8,
    // last_out: u64,
    num_arc: usize, //边的数量
    arcs: Vec<Arc>,
}

impl UnCompiledNode {
    fn new() -> UnCompiledNode {
        Self {
            //  last_in: 0,
            //   last_out: 0,
            num_arc: 0,
            arcs: Vec::new(),
        }
    }

    fn print(&self) {
        for v in self.arcs.iter() {
            print!("arc: in: {}, out:{} ", v._in as char, v.out);
        }
    }

    fn push_arc(&mut self, arc: Arc) {
        self.arcs.push(arc);
        self.num_arc = self.arcs.len();
    }

    fn last_in(&self) -> u8 {
        if self.num_arc == 0 {
            return 0;
        }
        self.arcs[self.num_arc - 1]._in
    }

    fn last_out(&self) -> u64 {
        if self.num_arc == 0 {
            return 0;
        }
        self.arcs[self.num_arc - 1].out
    }

    fn set_last_out(&mut self, out: u64) {
        self.arcs[self.num_arc - 1].out = out
    }

    fn set_in_out(&mut self, _in: u8, out: u64) {
        if self.num_arc == 0 {
            self.push_arc(Arc::new(_in, out));
            return;
        }
        self.arcs[self.num_arc - 1]._in = _in;
        self.arcs[self.num_arc - 1].out = out;
    }
}

struct BuilderNode {}

struct Arc {
    _in: u8,
    out: u64,
    is_final: bool,
}

impl Arc {
    fn new(_in: u8, out: u64) -> Arc {
        Self {
            _in: _in,
            out: out,
            is_final: false,
        }
    }
}

struct Builder<W: Write> {
    unfinished: UnCompiledNodes,
    encoder: Encoder<W>,
}

impl<W: Write> Builder<W> {
    fn new(w: W) -> Builder<W> {
        let mut unfinished = UnCompiledNodes::new();
        unfinished.push_empty();
        Self {
            unfinished: unfinished,
            encoder: Encoder::new(w),
        }
    }

    fn print(&self) {
        self.unfinished.print();
    }

    fn add(&mut self, key: &[u8], val: u64) {
        let (prefix_len, out) = self.unfinished.find_common_prefix(key, val);
        // self.freeze_tail(prefix_len);
        // for i in 0..prefix_len {
        //     self.unfinished.stack[i].add_arc();
        // }
        self.unfinished.add_suffix(&key[prefix_len..], out)
    }

    fn freeze_tail(&mut self, prefix_len: usize) {
        self.unfinished.pop_empty();
        while prefix_len < self.unfinished.stack.len() {
            if let Some(unfinish_node) = self.unfinished.pop_freeze() {
                self.compile_node(unfinish_node);
            } else {
                break;
            }
        }
        self.unfinished.push_empty();
    }

    fn compile_node(&self, node: UnCompiledNode) {
        for a in node.arcs.iter() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut b = Builder::new(vec![]);
        b.add("cat".as_bytes(), 2);
        b.add("deep".as_bytes(), 5);
        b.add("do".as_bytes(), 3);
        b.print()
    }
}
