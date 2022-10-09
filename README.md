# Staking ESDT 
This Smartcontract is able to stake ESDT to get same ESDT back as reward with certain apr

#### disclaimer: 
- using this code is on your own risk. This code is meant to be used for inspirational pruposes

To interact with the Smart Contract, you would need to have:
- python3.8 and upper
- [Pem wallet](https://github.com/ReneDuris/GeneratePem-erdjs) - Generating pem file through erdjs.
- [Elrond IDE](https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide/) - extension for Visual Studio Code
```
ctrl+shift+P
Elrond: Setup Workspace
```
# Interaction
To interact with the Smart Contract, you would need to supply your wallet with some funds:
- [ESDT/EGLD faucet](https://r3d4.fr/elrond/devnet/)


        
# Contract build
example:
```
erdpy --verbose contract build "/home/project/staking"
```
# Deploying smartcontract through erdpy.json
There is stored whole configuration to get smartcontract deployed by using command. You will need to do this only once.
```
erdpy contract deploy
```
# Upgrading smartcontract
example:
```
erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgq6lsfc55vs5yk56mrnd5s95jgm9lrevvs0ztsrkernq --recall-nonce --pem="wallet.pem" --gas-limit=60000000 --proxy="https://devnet-gateway.elrond.com" --chain=D --project=staking --arguments str:EFOO-8e80a5 100000000000000000 25 259200 --send || return
```
# Arguments upon deployement
Upon deployement or while ugrading you have to set your arguments.
- ESDT token to be staked and supplied for rewards
```
"str:TokenIdentifier"
```
- Minimum staking amount for the users (BigUint)
```
"1000000000000000000"
```
- APR
```
"25"
```
- Lock time for ESDT unstake in seconds
```
"259200"
```
```
   #[init]
    fn init(
        &self,
        staking_token: EgldOrEsdtTokenIdentifier,
        minimum_staking_amount: BigUint,
        apr: u64,
        lock_assets_time_in_seconds: u64
        
    ) {
        require!(minimum_staking_amount > BigUint::zero() ,"Staking amount cannot be 0");
        require!(apr > 0u64 ,"APR cannot be 0");
       
        self.staking_token().set(staking_token);
        self.minimum_stake_amount().set(minimum_staking_amount);
        self.apr().set(apr);
        self.lock_assets_time_in_seconds().set(lock_assets_time_in_seconds);
    }

```        
# Simulating contract call || mandos tests
- [Mandos tests](https://docs.elrond.com/developers/mandos-reference/structure/#docsNav)
- erdpy --verbose contract call
example
```
erdpy --verbose contract call erd1qqqqqqqqqqqqqpgq6lsfc55vs5yk56mrnd5s95jgm9lrevvs0ztsrkernq --function=ESDTTransfer --pem="wallet.pem" --proxy="https://devnet-gateway.elrond.com" --chain=D --recall-nonce --gas-limit=5000000 --arguments str:EFOO-8e80a5 1000000000000000000 str:stake --simulate
```
# Query SmartContract
Using view methods you are able to query your smartcontract to view informations.
- [query SmartContract-erdjs](https://github.com/ReneDuris/Query-SmartContract-erdjs)
       
 # View methods, storage mappers
- [View annotations](https://docs.elrond.com/developers/developer-reference/elrond-wasm-annotations/#endpoint-and-view)

Storage mappers can be used to store single values or multiple values. And with view method you are able to query stored value.
- [singleValueMapper](https://docs.elrond.com/developers/developer-reference/storage-mappers/#get)
```
    #[view(getStakedAmount)]
    #[storage_mapper("stakedAmount")]
    fn staked_amount(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;
```
    
# Endpoints
- [Endpoint annotations](https://docs.elrond.com/developers/developer-reference/elrond-wasm-annotations/#endpoint-and-view)
- #[payable("*")] - throught the endpoint can be sent tokens, nfts
- #[only_owner] - only owner can call the endpoint
- #[endpoint] - annotation of endpoint which can be called
```
    #[payable("*")]
    #[only_owner]
    #[endpoint]
    fn supply(&self){
        let staking_token = self.staking_token().get();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(payment_token == staking_token, "Invalid token input");
        self.reward_supply().update(|staked| *staked += &payment_amount);

    }
```
```
   #[endpoint(claim)]
    fn claim(&self){
        let caller = self.blockchain().get_caller();
        let staking_token = self.staking_token().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let rewards = self.storage_rewards(&caller).get()+self.calculated_rewards();

        require!(&self.blockchain().get_sc_balance(&staking_token,0) - &rewards >= self.staked_all().get() , "Claim disabled, low amount of provided tokens");

        self.storage_rewards(&caller).set(BigUint::zero());
        self.reset_timestamp(&self.blockchain().get_caller()).set(current_timestamp);
        self.send().direct(&caller, &staking_token, 0, &rewards);
    }
```
# SmartContract API functions
[SmartContract API functions](https://docs.elrond.com/developers/developer-reference/elrond-wasm-api-functions/#docsNav)
```
let caller = self.blockchain().get_caller();
let current_timestamp = self.blockchain().get_block_timestamp();
let balance_smartcontract = &self.blockchain().get_sc_balance(&staking_token,0)

```
# Calculation of rewards

``` #[view(calculatedRewards)]
    fn calculated_rewards(&self) -> BigUint{
        let caller = self.blockchain().get_caller();
        if self.reset_timestamp(&caller).is_empty(){ 
            return BigUint::zero();
         }

        let stake_timestamp = self.reset_timestamp(&caller).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let difference_timestamp = current_timestamp - stake_timestamp;
        let staked_amount = self.staked_amount(&caller).get();
        let apr =self.apr().get();
        let rewards =(BigUint::from(difference_timestamp) * BigUint::from(apr) *staked_amount)/BigUint::from(YEAR_IN_SECONDS)/BigUint::from(PERCENTAGE);
        rewards
    }
```

