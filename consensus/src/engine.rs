use ethereum_types::{Address, H256};
use ledger::transaction::Transaction;

pub trait Engine {
    /// 트랜잭션 검증자에 대한 주소를 반환한다.
    fn author(&self) -> Address;
    /// 해당 트랜잭션으로 인한 상태 전이 해시 값을 확인한다.
    fn verify_state_hash(&self) -> Address;
    /// 트랜잭션 정보를 가져온다.  PoA의 경우 리더 후보를, PoS의 경우 각 그룹의 인원을 조사한다.
    fn prepare(&self);
    /// seal은 DAG에 추가될 트랜잭션을 생성한다.
    fn seal(&self);
    /// finalize는 해당 트랜잭션을 DAG에 추가한다.
    fn finalize(&self);
}

pub struct PoaContext {
    authorities: Vec<Address>,
    target_transaction: Transaction, // 검증을 수행하고자 하는 대상 트랜잭션
}

pub struct PoaEngine {
    contract_address: Address,          // 명령이 처리되는 컨트랙트의 주소
    target_transaction: Transaction,    // 명령이 들어있는, 처리해야 하는 트랜잭션
    committed_transaction: Transaction, // PoA로 합의된 트랜잭션
    dirty_state: Vec<(H256, H256)>,     // 트랜잭션으로 인해 변화된 상태값인데 필요없는 값.
    authority: bool,
}

impl PoaEngine {
    pub fn new(contract_address: &Address,
               transaction: Transaction,
               dirty_state: Vec<(H256, H256)>) -> Self {
        PoaEngine {
            contract_address: contract_address.clone(),
            target_transaction: Default::default(),
            committed_transaction: Default::default(),
            dirty_state,
            authority: false
        }
    }
}

impl Engine for PoaEngine {
    fn author(&self) -> Address {
        accounts::DeviceAccount::read_account().unwrap().address
        // let public_key = crypto::secp256k1::recover_from_sig(
        //     &self.committed_transaction.v as i32,
        //     &self.committed_transaction.r,
        //     &self.committed_transaction.s
        // ).as_ref();
        // let mut address_value = [0u8;20];
        // let mut cnt = 0;
        // for element in public_key {
        //     if cnt < 11 { continue; }
        //     address_value[cnt - 12] = element[cnt];
        // }
    }

    fn verify_state_hash(&self, ) -> Address {
        let str_state_hash = String::new();

    }

    fn prepare(&self) {
        // do nothing
        todo!()
    }

    fn seal(&self) {
        todo!()
    }

    fn finalize(&self) {
        todo!()
    }
}

pub struct PosEngine {
    contract_address: Address,
}

impl PosEngine {
    pub fn new(contract_address: &Address) -> Self {
        PosEngine { contract_address: contract_address.clone() }
    }
}

impl Engine for PosEngine {
    /// 트랜잭션의 합의자(리더)의 주소
    fn author(&self) -> Address {
        todo!()
    }

    /// 트랜잭션에 포함된 상태의 해시 값을 검증하는 메서드
    fn verify_state_hash(&self) -> Address {
        todo!()
    }

    /// 트랜잭션을 합의하기 위해 필요한 내용들을 초기화 하는 메서드
    fn prepare(&self) {
        todo!()
    }

    /// 주어진 입력에 대한 합의된(채굴자 서명된) 트랜잭션을 생성하는 메서드
    /// 채굴 과정이 발생하는 시점이며 PoW의 경우 적합한 nonce를 찾는 과정을 수행한다.
    fn seal(&self) {
        todo!()
    }

    /// 서명된 트랜잭션을 확정하는 메서드
    fn finalize(&self) {
        todo!()
    }
}