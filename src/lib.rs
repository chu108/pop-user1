use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserOne {
    one_map: UnorderedMap<String, String>,
}
//实现默认初始化方法
impl Default for UserOne {
    fn default() -> Self {
        Self {
            one_map: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl UserOne {
    //添加消息
    pub fn one_add(&mut self, key: String, message: String) {
        let sig_act_id = env::signer_account_id().to_string();
        let cur_act_id = env::signer_account_id().to_string();
        let tmp = format!("signer_account_id:{}_current_account_id:{}_mess:{}", sig_act_id, cur_act_id, message);
        self.one_map.insert(&key, &tmp);
    }
    //获取消息
    pub fn one_get(&self, access_id: String) -> Option<String> {
        return self.one_map.get(&access_id);
    }
    //遍历消息
    pub fn one_list(&self) -> HashMap<String, String> {
        let mut list_map: HashMap<String, String> = HashMap::new();
        for (k, v) in self.one_map.iter() {
            list_map.insert(k, v);
        }
        return list_map;
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn one_add() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut one: UserOne = UserOne::default();
        one.one_add("张三".to_string(), "1111111".to_string());
        println!("添加成功！");
    }

    #[test]
    fn one_get() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let one: UserOne = UserOne::default();
        let mess = one.one_get(env::signer_account_id());
        println!(
            "获取account_id：{}, mess: {:?}",
            env::signer_account_id(),
            mess
        );
    }

    #[test]
    fn one_list() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut one: UserOne = UserOne::default();

        one.one_add("张三".to_string(), "111111".to_string());
        one.one_add("李四".to_string(), "222222".to_string());
        one.one_add("王五".to_string(), "33333333".to_string());

        for (k, v) in one.one_map.iter() {
            println!("key:{}, val:{}", k, v);
        }
    }

}
