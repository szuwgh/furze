use crate::bytes::Clear;
use crate::encoder::Encoder;
use crate::error::FstResult;
use crate::state::UnCompiledNode;
use crate::state::UnCompiledNodes;
use std::io::Write;

pub struct Builder<W>
where
    W: Write + Clear,
{
    unfinished: UnCompiledNodes,
    encoder: Encoder<W>,
}

impl<W> Builder<W>
where
    W: Write + Clear,
{
    pub fn new(w: W) -> Builder<W> {
        let mut unfinished = UnCompiledNodes::new();
        unfinished.push_empty(false);
        Self {
            unfinished: unfinished,
            encoder: Encoder::new(w),
        }
    }

    pub fn get(&self) -> &W {
        self.encoder.get_ref()
    }

    //插入一个节点
    pub fn add(&mut self, key: &[u8], val: u64) -> FstResult<()> {
        //查找公共前缀
        let (prefix_len, out) = self.unfinished.find_common_prefix(key, val);
        self.freeze_tail(prefix_len)?;
        self.unfinished.add_suffix(&key[prefix_len..], out);
        Ok(())
    }

    // 调用freezeTail方法, 从尾部一直到公共前缀的节点，将已经确定的状态节点冻结。
    // 这里会将UncompiledNode序列化到bytes当中，并转换成CompiledNode
    fn freeze_tail(&mut self, prefix_len: usize) -> FstResult<()> {
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

    fn compile_node(&mut self, node: UnCompiledNode) -> FstResult<i64> {
        let addr = self.encoder.add_node(node)?;
        Ok(addr as i64)
    }

    pub fn finish(&mut self) -> FstResult<()> {
        self.freeze_tail(0)?;
        if let Some(node) = self.unfinished.pop_root() {
            self.encoder.add_node(node)?;
        }
        Ok(())
    }

    pub fn bytes(&self) -> &W {
        self.encoder.get_ref()
    }

    pub fn reset(&mut self) {
        self.unfinished.reset();
        self.unfinished.push_empty(false);
        self.encoder.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::Bytes;
    use crate::decoder::Decoder;
    #[test]
    fn test_add() {
        let mut b = Builder::new(Bytes::new());
        b.add("cat".as_bytes(), 5);
        b.add("dog".as_bytes(), 10);
        b.add("deep".as_bytes(), 15);
        b.add("logs".as_bytes(), 2);
        b.finish();

        let mut d = Decoder::new(b.encoder.get_ref());
        let res = d.get("logs".as_bytes());
        match res {
            Ok(v) => {
                println!("out:{}", v);
            }
            Err(e) => {
                println!("e:{:?}", e);
            }
        }

        b.reset()
    }
}
