pub(crate) const BIT_FINAL_STATE: u8 = 1 << 0;
pub(crate) const BIT_LAST_STATE: u8 = 1 << 1;
pub(crate) const BIT_TAGET_NEXT: u8 = 1 << 2;
pub(crate) const BIT_STOP_NODE: u8 = 1 << 3;
pub(crate) const BIT_STATE_HAS_OUPPUT: u8 = 1 << 4;
pub(crate) const BIT_STATE_HAS_FINAL_OUTPUT: u8 = 1 << 5;
pub(crate) const ARCS_AS_FIXED_ARRAY: u8 = 1 << 6;
//pub(crate) const ARCS_AS_FIXED_ARRAY: u8 = BIT_STATE_HAS_FINAL_OUTPUT;
pub(crate) struct UnCompiledNodes {
    pub(crate) stack: Vec<UnCompiledNode>,
}

impl UnCompiledNodes {
    pub fn new() -> UnCompiledNodes {
        let stack: Vec<UnCompiledNode> = Vec::with_capacity(64);
        Self { stack: stack }
    }

    //计算当前字符串和上一个字符串的公共前缀
    pub fn reset(&mut self) {
        //后期引入对象池
        self.stack.clear()
    }

    pub fn find_common_prefix(&mut self, key: &[u8], mut out: u64) -> (usize, u64) {
        let mut i: usize = 0;
        while i < key.len() {
            if i >= self.stack.len() {
                break;
            }
            let mut add_prefix: u64 = 0;
            if self.stack[i].last_in() == key[i] {
                //查找最小值 上一个输入是10 下一个输入是15 则公共输出前缀是 10
                //如果下一个输入是2 则公共输出前缀是 2
                let common_pre = Self::output_prefix(self.stack[i].last_out(), out);
                //输出add_prefix 共有前缀 例如 common_pre是10 上一个输入是10
                //上一个输入是15 则add_prefix = 13
                add_prefix = Self::output_sub(self.stack[i].last_out(), common_pre);
                //out = 15 common_pre=10 out=5
                out = Self::output_sub(out, common_pre);
                self.stack[i].set_last_out(common_pre);
                i += 1;
            } else {
                break;
            }
            //如果下一个输入 out 比 上一个 要小 则会有 final_out
            if add_prefix > 0 {
                let final_out = self.stack[i].add_output_prefix(add_prefix);
                self.stack[i - 1].set_final_out(final_out);
            }
        }
        (i, out)
    }

    pub(crate) fn push_empty(&mut self, _final: bool) {
        self.stack.push(UnCompiledNode::new(_final));
    }

    pub(crate) fn pop_empty(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn pop_root(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    pub(crate) fn pop_freeze(&mut self) -> Option<UnCompiledNode> {
        self.stack.pop()
    }

    pub(crate) fn top_last_freeze(&mut self, addr: u64) {
        if let Some(n) = self.stack.last_mut() {
            n.last_compiled(addr);
        }
    }

    fn add_prefix(&mut self, key: &[u8]) {}

    pub(crate) fn add_suffix(&mut self, key: &[u8], out: u64) {
        if key.len() == 0 {
            return;
        }
        let last = self.stack.len() - 1;
        self.stack[last].push_state(State::new(key[0], out));
        for (i, v) in key[1..].iter().enumerate() {
            let mut next = UnCompiledNode::new(false);
            let state = State::new(*v, 0);
            next.push_state(state);
            self.stack.push(next);
        }
        if let Some(n) = self.stack.last_mut() {
            if let Some(s1) = n.states.last_mut() {
                s1.is_final = true;
            }
        }

        self.push_empty(true);
    }

    fn output_prefix(l: u64, r: u64) -> u64 {
        core::cmp::min(l, r)
    }

    fn output_sub(l: u64, r: u64) -> u64 {
        l - r
    }
}

pub struct UnCompiledNode {
    pub(crate) states: Vec<State>,
    pub(crate) is_final: bool,
    pub(crate) final_output: u64,
}

impl UnCompiledNode {
    fn new(_final: bool) -> UnCompiledNode {
        Self {
            states: Vec::new(),
            is_final: _final,
            final_output: 0,
        }
    }

    fn add_output_prefix(&mut self, prefix_len: u64) -> u64 {
        let mut final_out: u64 = 0;
        if self.is_final {
            final_out = Self::output_cat(prefix_len, self.final_output);
        }
        if self.states.len() == 0 {
            return final_out;
        }
        //依次修改每个状态的输出
        for i in 0..self.states.len() - 1 {
            let state = self.states.get_mut(i).unwrap();
            state.out = Self::output_cat(prefix_len, state.out);
        }
        let state = self.last_mut_state();
        state.out = Self::output_cat(prefix_len, state.out);
        return final_out;
    }

    pub(crate) fn last_compiled(&mut self, addr: u64) {
        if let Some(a) = self.states.last_mut() {
            a.target = addr;
        }
    }

    fn push_state(&mut self, state: State) {
        self.states.push(state);
    }

    fn last_state(&self) -> &State {
        self.states.last().expect("get last state fail")
    }

    fn last_mut_state(&mut self) -> &mut State {
        self.states.last_mut().expect("get last mut state fail")
    }

    fn last_in(&self) -> u8 {
        if let Some(a) = self.states.last() {
            return a._in;
        }
        0
    }

    fn last_out(&self) -> u64 {
        if let Some(a) = self.states.last() {
            return a.out;
        }
        0
    }

    fn set_last_out(&mut self, out: u64) {
        if let Some(a) = self.states.last_mut() {
            a.out = out
        }
    }

    fn set_final_out(&mut self, final_output: u64) {
        if let Some(a) = self.states.last_mut() {
            a.final_out = final_output;
        }
    }

    fn set_in_out(&mut self, _in: u8, out: u64) {
        if self.states.len() == 0 {
            self.push_state(State::new(_in, out));
            return;
        }
        if let Some(a) = self.states.last_mut() {
            a._in = _in;
            a.out = out;
        }
    }

    fn output_cat(l: u64, r: u64) -> u64 {
        l + r
    }
}

#[derive(Default, Clone)]
pub(crate) struct State {
    pub(crate) flag: u8,
    pub(crate) _in: u8,
    pub(crate) out: u64,
    pub(crate) final_out: u64,
    pub(crate) target: u64,
    //pub(crate) node: u64,
    pub(crate) next_state: u64,
    //  pub(crate) is_last: bool,
    //  pub(crate) is_stop: bool,
    pub(crate) is_final: bool,
}

impl State {
    pub(crate) fn new(_in: u8, out: u64) -> State {
        Self {
            flag: 0,
            _in: _in,
            out: out,
            final_out: 0,
            target: 0,
            //node: 0,
            next_state: 0,
            //       is_last: false,
            //       is_stop: false,
            is_final: false,
        }
    }

    pub(crate) fn is_final(&self) -> bool {
        self.flag(BIT_FINAL_STATE)
    }

    pub(crate) fn is_last(&self) -> bool {
        self.flag(BIT_LAST_STATE)
    }

    pub(crate) fn is_stop(&self) -> bool {
        self.flag(BIT_STOP_NODE)
    }

    pub(crate) fn flag(&self, f: u8) -> bool {
        return (self.flag & f) != 0;
    }

    pub(crate) fn reset(&mut self) {
        self._in = 0;
        self.out = 0;
        self.final_out = 0;
        self.target = 0;
        //    self.is_last = false;
        self.flag = 0;
        //    self.is_stop = false;
        self.is_final = false;
        self.next_state = 0;
    }
}
