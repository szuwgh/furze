use crate::decoder::Decoder;
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

    fn find_common_prefix(&mut self, key: &[u8], mut out: u64) -> (usize, u64) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < key.len() {
            if i >= self.stack.len() {
                break;
            }
            let mut add_prefix: u64 = 0;
            if self.stack[i].last_in() == key[i] {
                let common_pre = Self::output_prefix(self.stack[i].last_out(), out);
                add_prefix = Self::output_sub(self.stack[i].last_out(), common_pre);
                out = Self::output_sub(out, common_pre);
                self.stack[i].set_last_out(common_pre);
                j = i;
                i += 1;
            } else {
                break;
            }
            if add_prefix > 0 {
                let final_out = self.stack[i].add_output_prefix(add_prefix);
                self.stack[j].set_final_out(final_out);
            }
        }
        (i, out)
    }

    fn push_empty(&mut self, _final: bool) {
        self.stack.push(UnCompiledNode::new(_final));
    }

    fn pop_empty(&mut self) {
        self.stack.pop();
    }

    fn pop_root(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    fn pop_freeze(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    fn top_last_freeze(&mut self, addr: u64) {
        if let Some(n) = self.stack.last_mut() {
            n.last_compiled(addr);
        }
    }

    fn add_prefix(&mut self, key: &[u8]) {}

    fn add_suffix(&mut self, key: &[u8], out: u64) {
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

    fn output_prefix(l: u64, r: u64) -> u64 {
        if l < r {
            return l;
        }
        r
    }

    fn output_sub(l: u64, r: u64) -> u64 {
        l - r
    }
}

pub struct UnCompiledNode {
    pub num_arc: usize, //边的数量
    pub arcs: Vec<Arc>,
    pub is_final: bool,
    pub final_output: u64,
}

impl UnCompiledNode {
    fn new(_final: bool) -> UnCompiledNode {
        Self {
            num_arc: 0,
            arcs: Vec::new(),
            is_final: _final,
            final_output: 0,
        }
    }

    fn print(&self) {
        for v in self.arcs.iter() {
            print!(
                "arc: in: {}, out:{} ,final_out:{}, target: {} ;",
                v._in as char, v.out, v.final_out, v.target,
            );
        }
    }

    fn add_output_prefix(&mut self, prefix_len: u64) -> u64 {
        let mut final_out: u64 = 0;
        if self.is_final {
            final_out = Self::output_cat(prefix_len, self.final_output);
        }
        if self.num_arc == 0 {
            return final_out;
        }
        for i in 0..self.num_arc - 1 {
            let arc = &mut self.arcs[i];
            arc.out = Self::output_cat(prefix_len, arc.out);
        }
        let arc = &mut self.arcs[self.num_arc - 1];
        arc.out = Self::output_cat(prefix_len, arc.out);
        return final_out;
    }

    fn last_compiled(&mut self, addr: u64) {
        if let Some(a) = self.arcs.last_mut() {
            a.target = addr;
        }
        // let arc = &mut self.arcs[self.num_arc - 1];
        //arc.target = addr;
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

    fn set_final_out(&mut self, final_output: u64) {
        self.arcs[self.num_arc - 1].final_out = final_output
    }

    fn set_in_out(&mut self, _in: u8, out: u64) {
        if self.num_arc == 0 {
            self.push_arc(Arc::new(_in, out));
            return;
        }
        let arc = &mut self.arcs[self.num_arc - 1];
        arc._in = _in;
        arc.out = out;
    }

    fn output_cat(l: u64, r: u64) -> u64 {
        l + r
    }
}

struct BuilderNode {}

pub struct Arc {
    pub _in: u8,
    pub out: u64,
    pub final_out: u64,
    pub target: u64,
    pub is_last: bool,
    pub flag: u8,
}

impl Arc {
    pub fn new(_in: u8, out: u64) -> Arc {
        Self {
            _in: _in,
            out: out,
            final_out: 0,
            target: 0,
            is_last: false,
            flag: 0,
        }
    }

    pub fn flag(&self, f: u8) -> bool {
        return (self.flag & f) != 0;
    }

    pub fn reset(&mut self) {
        self._in = 0;
        self.out = 0;
        self.final_out = 0;
        self.target = 0;
        self.is_last = false;
        self.flag = 0;
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

    fn add(&mut self, key: &[u8], val: u64) -> Result<()> {
        let (prefix_len, out) = self.unfinished.find_common_prefix(key, val);
        self.freeze_tail(prefix_len)?;
        self.unfinished.add_suffix(&key[prefix_len..], out);
        Ok(())
    }

    //c a t
    //d e e p
    fn freeze_tail(&mut self, prefix_len: usize) -> Result<()> {
        let mut addr: i64 = -1;
        while prefix_len + 1 < self.unfinished.stack.len() {
            if addr == -1 {
                self.unfinished.pop_empty();
                addr = 0;
            } else {
                if let Some(mut unfinish_node) = self.unfinished.pop_freeze() {
                    unfinish_node.last_compiled(addr as u64);
                    addr = self.compile_node(unfinish_node)?;
                } else {
                    break;
                }
            }
        }
        self.unfinished.top_last_freeze(addr as u64);
        Ok(())
    }

    fn compile_node(&mut self, node: UnCompiledNode) -> Result<i64> {
        let addr = self.encoder.add_node(node)?;
        Ok(addr as i64)
    }

    fn finish(&mut self) -> Result<()> {
        self.freeze_tail(0)?;
        if let Some(node) = self.unfinished.pop_root() {
            self.encoder.add_node(node)?;
        }
        Ok(())
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
        b.finish();

        println!("{:?}", b.encoder.get_ref());

        let mut d = Decoder::new(b.encoder.get_ref().to_vec());
        let v = d.find("do".as_bytes());
        match v {
            Ok(vv) => {
                println!("v:{}", vv);
            }
            Err(e) => {
                println!("e:{}", e);
            }
        }
    }
}
