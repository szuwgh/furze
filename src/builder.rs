use crate::encoder::Encoder;
use anyhow::Result;
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
        }
    }

    fn find_common_prefix(&mut self, key: &[u8], mut out: i64) -> (usize, i64) {
        let mut i: usize = 0;
        while i < key.len() {
            if i >= self.stack.len() {
                break;
            }
            if self.stack[i].last_in() == key[i] {
                let common_pre = Self::output_prefix(self.stack[i].last_out(), out);
                let add_prefix = Self::output_sub(self.stack[i].last_out(), common_pre);
                out = Self::output_sub(out, common_pre);
                self.stack[i].set_last_out(common_pre);
                i += 1;
            } else {
                break;
            }
        }
        (i, out)
    }

    fn add_output_prefix() {}

    fn push_empty(&mut self, _final: bool) {
        self.stack.push(UnCompiledNode::new(_final));
    }

    fn pop_empty(&mut self) {
        self.stack.pop();
    }

    fn pop_freeze(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    fn add_prefix(&mut self, key: &[u8]) {}

    fn add_suffix(&mut self, key: &[u8], out: i64) {
        if key.len() == 0 {
            return;
        }
        let last = self.stack.len() - 1;
        self.stack[last].push_arc(Arc::new(key[0], out));
        for v in &key[1..] {
            let mut next = UnCompiledNode::new(false);
            next.push_arc(Arc::new(*v, 0));
            self.stack.push(next);
        }
        self.push_empty(true);
    }

    fn output_prefix(l: i64, r: i64) -> i64 {
        if l < r {
            return l;
        }
        r
    }

    fn output_sub(l: i64, r: i64) -> i64 {
        l - r
    }

    fn output_cat(l: i64, r: i64) -> i64 {
        l + r
    }
}

pub struct UnCompiledNode {
    pub num_arc: usize, //边的数量
    pub arcs: Vec<Arc>,
    is_final: bool,
}

impl UnCompiledNode {
    fn new(_final: bool) -> UnCompiledNode {
        Self {
            num_arc: 0,
            arcs: Vec::new(),
            is_final: _final,
        }
    }

    fn print(&self) {
        for v in self.arcs.iter() {
            print!("arc: in: {}, out:{} ", v._in as char, v.out);
        }
    }

    fn last_compiled(&mut self, addr: u32) {
        let arc = &mut self.arcs[self.num_arc - 1];
        arc.target = addr;
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

    fn last_out(&self) -> i64 {
        if self.num_arc == 0 {
            return 0;
        }
        self.arcs[self.num_arc - 1].out
    }

    fn set_last_out(&mut self, out: i64) {
        self.arcs[self.num_arc - 1].out = out
    }

    fn set_in_out(&mut self, _in: u8, out: i64) {
        if self.num_arc == 0 {
            self.push_arc(Arc::new(_in, out));
            return;
        }
        let arc = &mut self.arcs[self.num_arc - 1];
        arc._in = _in;
        arc.out = out;
    }
}

struct BuilderNode {}

pub struct Arc {
    pub _in: u8,
    pub out: i64,
    pub is_final: bool,
    pub target: u32,
}

impl Arc {
    fn new(_in: u8, out: i64) -> Arc {
        Self {
            _in: _in,
            out: out,
            is_final: false,
            target: 0,
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
        unfinished.push_empty(false);
        Self {
            unfinished: unfinished,
            encoder: Encoder::new(w),
        }
    }

    fn print(&self) {
        self.unfinished.print();
    }

    fn add(&mut self, key: &[u8], val: i64) -> Result<()> {
        let (prefix_len, out) = self.unfinished.find_common_prefix(key, val);
        self.freeze_tail(prefix_len)?;
        self.unfinished.add_suffix(&key[prefix_len..], out);
        Ok(())
    }

    //c a t
    //d e e p
    fn freeze_tail(&mut self, prefix_len: usize) -> Result<()> {
        let mut addr: u32 = 0;
        while prefix_len + 1 < self.unfinished.stack.len() {
            if addr == 0 {
                self.unfinished.pop_empty();
                addr = 1;
            } else {
                if let Some(mut unfinish_node) = self.unfinished.pop_freeze() {
                    unfinish_node.last_compiled(addr);
                    addr = self.compile_node(unfinish_node)?;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    fn compile_node(&mut self, node: UnCompiledNode) -> Result<u32> {
        // for a in node.arcs.iter() {
        //     // self.encoder.write_byte(b: u8)
        // }
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut b = Builder::new(vec![]);
        b.add("cat".as_bytes(), 5);
        b.add("deep".as_bytes(), 10);
        b.add("do".as_bytes(), 15);
        b.add("dog".as_bytes(), 2);
        b.print()
    }
}
