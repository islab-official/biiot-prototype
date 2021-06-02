use crate::transaction::RawTransaction;

/// 현재 노드가 Leader일 경우, TxPool을 통해 전송받은 트랜잭션을 커밋하기 위해 사용한다.
/// 현재 노드가 Follower일 경우, 커밋된 트랜잭션을 확인하기 위해 사용한다.
pub struct TxPool {
    value: Vec<RawTransaction>,
}

impl TxPool {
    pub fn new() -> Self {
        TxPool { value: vec![] }
    }

    pub fn push(&mut self, raw_tx: RawTransaction) {
        self.value.push(raw_tx);
    }

    pub fn remove(&mut self, raw_tx: RawTransaction) {
        let mut no_error = false;
        let mut idx = 0;
        for element in self.value.iter() {
            if element.recipient == raw_tx.recipient
                && element.nonce == raw_tx.nonce
                && element.data == raw_tx.data {
                no_error = true;
                break
            }
            idx += 1;
        }

        if !no_error { return }
        self.value.remove(idx);
    }
}