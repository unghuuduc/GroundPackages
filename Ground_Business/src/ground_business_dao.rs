/*!
# [GroundBusinessDAO]: Make a Ground for your Web 3 Business!

GroundBusinessDAO is the core blueprint of the Ground Business package for any business operator build a business Ground in Web3 Society.

## Main Features:
The blueprint is for web3 business operators to build a DAO for their business. 

Through the blueprint, business operators can maximize business effeciency, earn consumers trust, and promote collective intelligence on Web3.

The blueprint included a Quartic Voting Mechanism which algorithmically calculate the member(or delegator)'s voting power 
through four components: ID trust score, staking amount, committed year, aggregrated CVs component score (with accepted CVs).

The blueprint included a value-driven Proof of Concept Consensus Model.
DAO members are incentivized to agree on collective actions which benefit for the whole

The blueprint included a internal DEX treasury for people to easily exchange between a DAO accepted stablecoin the DAO share token.
The internal DEX works like an automated, internal stock market to reward share holders.

The blueprint included a method for any volunteer or the DAO to 
deposit their protocol's revenue directly into the treasury to raise the 
DAO share price on the internal DEX and benefit the DAO share holders.

The blueprint included a method for DAO protocol's consumers to 
take the compensation directly from the treasury to degrade the 
DAO share price on the internal DEX and put the consequence on the share holders.

The blueprint also utilized the Human Resource Service from GroundCV blueprint, the Oracle solution from NeuRacle blueprint 
and the Liquid Democracy Service from the GroundVotingPool blueprint.

- The Human Resource Service is for the DAO to check entry requirement before made a wallet address became DAO member. 
The service also help the DAO aggregrate the CV id score for the quartic voting mechanism.

- The Oracle solution is for the DAO to keep track of member, voting delegator's committed time for the DAO, 
make sure they cannot withdraw their stake from the DAO until the committed time.

- The Liquid Democracy Service is for the DAO's member to make their own voting pool 
where user who don't meet the member entry requirement can also contribute to the DAO.

## DAO entities:
1. **DAO's Members**: Main managers of the DAO. Through the blueprint's method, *DAO's Members* are allowed to:
- Reassess the voting power through new ID SBT and CV component SBTs.
- Create a voting pool for delegators.
- Vote on a proposed concept.
- Begin unstake their DAO share when their Member NFT passed the commited year.
- Withdraw their DAO share after passed the unstake delay time.
- Take their dividend from the DAO.
- Propose a "Concept" (require a CV component SBT prove the DAO Member's Scrypto skill).
 
2. **DAO's Shareholders**: Any wallet address can become DAO shareholders by buying the DAO share token directly from the internal DEX. Through the blueprint's method, *DAO's Shareholders* are allowed to:
- Buy, sell their DAO share token from the internal DEX.
- Become a DAO members (require the unique Person ID SBT and the CV components SBTs for entry requirement).

3. **DAO's Voting Pools**: DAO's voting pools created from DAO's members. Through the blueprint's method, *DAO's voting pool* are allowed to:
- Change DAO total voting power based on the pool's delegator voting powers.
- Get current time data through the DAO for it's method.
- Take delegator's dividend from the DAO.

4. **DAO's Protocols**: The DAO can run many protocols. Through the blueprint's method, *DAO's protocols* are allowed to:
- Deposit protocol's revenue directly into the DAO's treasury.
- Withdraw an amount directly from the DAO's treasury to compensate it's users.
*/

use scrypto::prelude::*;
use neuracle::neuracle::*;
use ground_id::{Identity, GroundID};
use crate::utils::*;
use crate::cv_id_const::*;
use crate::ground_cv::*;
use crate::ground_voting_pool::*;

const SCRYPTO_CV: u32 = SKILL + SCRYPTO_PROGRAMMING as u32;
pub const YEAR: u64 = 60 * 60 * 365;

#[derive(TypeId, Encode, Decode, Describe)]
pub struct Method {
      
      pub component: ComponentAddress,

      pub method: String,
  
      pub args: Vec<Vec<u8>>,

}

impl Method {

    pub fn call(&self) {
        Runtime::call_method(self.component, &self.method, self.args.to_vec());
    }

}

/// The struct keep track off all the methods that DAO devs proposed.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Methods {
    pub methods: Vec<Method>,
}

impl Methods {

    /// The function to call all the methods that reach consensus.
    pub fn call_all(&self) {
        self.methods.iter().for_each(|method| {
            method.call()
        })
    }
}

/// The DAO Member SBT.
/// 
/// The SBT will only be issued when the user stake an amount of DAO token for a period of time.
#[derive(NonFungibleData)]
pub struct DAOMember {

    #[scrypto(mutable)]
    data: DAOContributorData,
    #[scrypto(mutable)]
    voting_pool: Option<ComponentAddress>

}

/// The NFT keep track of a Delegator's data.
#[derive(NonFungibleData)]
pub struct Delegator {

    #[scrypto(mutable)]
    pub data: DAOContributorData,
    
}

#[derive(TypeId, Encode, Decode, Describe)]
pub struct DAOContributorData {

    /// The voting power will be aggregrated from a quartic function with 4 params: 
    /// 
    /// ID trust score, staking amount, locked staking year, skill score (with accepted skill).
    /// 
    /// Aggregrated CVs score (aggregrated_cvs_score) = medium of all required skill scores.
    pub voting_power: Decimal,
    pub voted_proposal: HashMap<NonFungibleId, VotedData>,
    committed_year: u8,
    staking_amount: Decimal,
    allow_unstaking_time: u64

}

#[derive(TypeId, Encode, Decode, Describe)]
pub struct VotedData {

   pub voted_power: Decimal,
   pub vote: bool

}

/// The unstaking badge. To keep track of DAO member or Delegator's unstaking.
#[derive(NonFungibleData)]
pub struct Unstake {

    pub amount: Decimal,
    pub end_time: u64

}

/// The proposal badge for the dev DAO member who propose a transaction require the DAO authority.
#[derive(NonFungibleData)]
pub struct Proposal {
    methods: Methods,
    reward_demand: Decimal
}

#[derive(TypeId, Encode, Decode, Describe)]
pub struct ConceptData {
    total_voted: Decimal,
    voted_power: Decimal,
    status: Option<bool>
}

impl ConceptData {
    pub fn new() -> Self {
        Self {
            total_voted: Decimal::ZERO,
            voted_power: Decimal::ZERO,
            status: None
        }
    }
}

blueprint! {

    struct GroundBusinessDAO {

        /// Component controller badge.
        controller_badge: Vault,
        /// Voting pool controller badge address
        pool_controller_badge: ResourceAddress,
        /// Hold the dao badge. This badge also hold the dao share token's minting, burning authority.
        dao_badge: Vault,
        /// The on-using Ground CV component address for on-chain human resource service,
        ground_cv: ComponentAddress,
        /// The on-using identity service.
        ground_id: ComponentAddress,
        /// The DAO treasury vaults, an AMM mechanism will be implemented on these vaults 
        /// so people can easily exchange between DAO share token and the stable coin.
        treasury: (Vault, Vault),
        /// internal treasury auto swap fee.
        swap_fee: Decimal,
        /// The DAO staking vault
        stake_vault: Vault,
        /// The DAO dividend vault
        dividend_vault: Vault,
        /// The DAO unstake badge address
        unstake_badge: ResourceAddress,
        /// The DAO transient move resource
        transient_move_resource: ResourceAddress,
        /// The DAO member SBT resource address
        member_sbt: ResourceAddress,
        /// Delegator badge resource address
        delegator_badge: ResourceAddress,
        /// Required CV components to become a DAO member.
        /// This is to retain voting capability only for people with required knowledge background.
        /// These skill id will also be taken in account when computing member's voting power
        /// syntax: HashMap<cv_id, cv_level>
        member_entry_requirement: HashMap<u32, u8>,
        /// Required Scrypto skill score for a DAO proposal
        /// Scrypto skill on CV ID is 3002 (check the CV ID Const table)
        proposal_requirement: u8,
        /// The book keep track of on-going DAO dev's proposals
        /// Syntax: <The proposal id, (the proposed methods, the voted weight)>
        proposal_book: HashMap<NonFungibleId, ConceptData>,
        /// The proposal badge address
        proposal_badge: ResourceAddress,
        /// proposal id counter
        proposal_id_counter: u64,
        /// Reward exponential rate for dev, for each % (>66%) voted power / total vote power on the passed proposal (min 0, max 33). (%)
        dev_expo_reward: Decimal,
        /// DAO share dividend rate for each collective action of DAO members (%/action)
        dividend_rate: Decimal,
        /// The longer a member locked-stake their DAO token, the more voting power they get.
        /// This is the voting power increase rate for each year they commited on contributing for the DAO. (%)
        year_commited_rate: Decimal,
        /// The dividend fee delegated voter has to pay for their voting pool.
        /// This is to incentivize DAO members who has required knowledge background and a good reputation. (%)
        voting_pool_fee: Decimal,
        /// Maximum year a member can commited on contributing for the DAO.
        year_cap: u8,
        /// Unstake delay time for a DAO member before they can withdraw their staked amount. (seconds)
        /// On unstake delay time, the DAO member cannot withdraw staked amount or make a vote to receive any DAO share. 
        /// This is to prevent members to voting something harmful for the DAO right before withdraw all their DAO share.
        unstake_delay: u64,
        /// Keeping track of the total voting power of the DAO.
        /// The DAO will only come into consensus when there is > 2/3 voted power on a proposal.
        total_voting_power: Decimal,
        /// The on-using Oracle
        oracle: (ComponentAddress, Vault),
        /// The controller badge of the protocols that this DAO is running
        protocols: Vec<ResourceAddress>,
        /// The controller badge resource address of the DAO's components 
        /// fpr read time data (to prevent re-routing the data) 
        /// or for writing into total voting power data
        dao_controllers: Vec<ResourceAddress>
        

    }

    impl GroundBusinessDAO {
        
        /// This function will create new Identity Service component
        /// Input: 
        /// - name: the organization's name.
        /// - dao_badge: the DAO badge. This badge also hold the dao share token's minting, burning authority.
        /// - dao_token: the inital DAO share token treasury.
        /// - stable_coin: the initial fiat-backed stable coin treasury.
        /// - swap_fee: initial internal treasury auto swap fee.
        /// - ground_cv: the Ground CV component address for on-chain human resource service.
        /// - ground_id: the identity service component address.
        /// - member_entry_requirement: the intial required CV components for the DAO members to make a vote or a delegated voting pool.
        /// - proposal_require: initial required Scrypto skill score for a DAO proposal.
        /// - dvidend_rate: the initial DAO share dividend rate for each collective action of DAO members (%/action).
        /// - reward: initial reward rate for a passed proposal from scrypto dev. (DAO share token)
        /// - year_commited_rate: the inital voting power increase rate for each year members commited on contributing for the DAO. (%)
        /// - voting_pool_fee: initial dividend fee delegated voter has to pay for their voting pool. (%)
        /// - year_cap: initial maximum year a member can commited on contributing for the DAO.
        /// - unstake_delay: initial unstake delay time for a DAO member before they can withdraw their staked amount. (seconds)
        /// - oracle: initial oracle component address and the time data badge.
        /// - protocols: initial controller badges of the protocols the DAO is running.
        /// Output: DAO Component address
        pub fn new(
            name: String, 
            dao_badge: Bucket, 
            dao_token: Bucket,
            stable_coin: Bucket,
            swap_fee: Decimal,
            ground_cv: ComponentAddress,
            ground_id: ComponentAddress,
            member_entry_requirement: HashMap<u32, u8>,
            proposal_requirement: u8,
            dividend_rate: Decimal,
            dev_expo_reward: Decimal,
            year_commited_rate: Decimal,
            voting_pool_fee: Decimal,
            year_cap: u8,
            unstake_delay: u64,
            oracle: (ComponentAddress, Bucket),
            protocols: Vec<ResourceAddress>
        ) -> ComponentAddress {

            assert_rate(swap_fee); assert_rate(voting_pool_fee); assert_rate(dividend_rate); assert_rate(year_commited_rate);

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Dao Controller Badge")
                .initial_supply(dec!(1isize));

            let pool_controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Dao Pool Controller Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let transient_move_resource = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .metadata("name", name.clone() + "'s Dao Transient Move Badge")
                .no_initial_supply();

            let member_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + " DAO Member SBT")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .restrict_deposit(rule!(require(transient_move_resource)), LOCKED)
                .no_initial_supply();

            let delegator_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "DAO Delegator Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address()) || require(pool_controller_badge)), LOCKED)
                .no_initial_supply();

            let proposal_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s DAO Proposal Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let unstake_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s DAO Unstake Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("use_oracle", rule!(require(dao_badge.resource_address())))
                .method("withdraw", rule!(require(dao_badge.resource_address())))
                .method("add_protocol", rule!(require(dao_badge.resource_address())))
                .method("remove_protocol", rule!(require(dao_badge.resource_address())))
                .method("change_voting_pool_fee", rule!(require(dao_badge.resource_address())))
                .method("change_member_entry_requirement", rule!(require(dao_badge.resource_address())))
                .method("change_swap_fee", rule!(require(dao_badge.resource_address())))
                .method("change_proposal_requirement", rule!(require(dao_badge.resource_address())))
                .method("change_dev_initial_reward", rule!(require(dao_badge.resource_address())))
                .method("change_dev_expo_reward", rule!(require(dao_badge.resource_address())))
                .method("change_dividend_rate", rule!(require(dao_badge.resource_address())))
                .method("change_year_commited_rate", rule!(require(dao_badge.resource_address())))
                .method("change_year_cap", rule!(require(dao_badge.resource_address())))
                .method("change_unstake_delay", rule!(require(dao_badge.resource_address())))
                .method("propose_concept", rule!(require(member_sbt)))
                .default(rule!(allow_all));

            let dao_token_address = dao_token.resource_address();

            let controller_badge_address = controller_badge.resource_address();

            let comp = Self {

                controller_badge: Vault::with_bucket(controller_badge),
                pool_controller_badge: pool_controller_badge,
                dao_badge: Vault::with_bucket(dao_badge),
                ground_cv: ground_cv,
                ground_id: ground_id,
                treasury: (Vault::with_bucket(stable_coin), Vault::with_bucket(dao_token)),
                swap_fee: swap_fee / dec!("100"),
                stake_vault: Vault::new(dao_token_address),
                dividend_vault: Vault::new(dao_token_address),
                unstake_badge: unstake_badge,
                transient_move_resource: transient_move_resource,
                member_sbt: member_sbt,
                delegator_badge: delegator_badge,
                member_entry_requirement: member_entry_requirement,
                proposal_requirement: proposal_requirement,
                proposal_book: HashMap::new(),
                proposal_badge: proposal_badge,
                proposal_id_counter: 0,
                dividend_rate: dividend_rate / dec!("100"),
                dev_expo_reward: dev_expo_reward,
                year_commited_rate: year_commited_rate / dec!("100"),
                voting_pool_fee: voting_pool_fee / dec!("100"),
                year_cap: year_cap,
                unstake_delay: unstake_delay,
                total_voting_power: Decimal::ZERO,
                oracle: (oracle.0, Vault::with_bucket(oracle.1)),
                protocols: protocols,
                dao_controllers: Vec::from([controller_badge_address])

            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return comp
        }

        /// This method is for DAO members or the protocols it's running (or any volunteer) to deposit stable coin directly into the treasury
        pub fn deposit(&mut self, bucket: Bucket) {

            self.treasury.0.put(bucket)
            
        }

        /// This method is for a collective action of the DAO to withdraw stable coin directly from the treasury
        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {

            self.treasury.0.take(amount)
            
        }

        /// This method is for the protocols it's running to withdraw stable coin directly from the treasury 
        /// to compensate the protocol's users in case of a problem
        pub fn compensate(&mut self, protocol_proof: Proof, amount: Decimal) -> Bucket {

            assert!(self.protocols.contains(&protocol_proof.resource_address()), "This is not a protocol the DAO's running!");

            self.treasury.0.take(amount)
            
        }

        /// This method is for the DAO to change oracle using.
        pub fn use_oracle(&mut self, oracle: ComponentAddress, data_badge: Bucket) -> Bucket {
            self.oracle.0 = oracle;
            let bucket = self.oracle.1.take_all();
            self.oracle.1.put(data_badge);
            bucket
        }

        /// This method is for the DAO to refund the oracle account.
        pub fn refund_oracle_account(&self, bucket: Bucket) -> Bucket {
            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            neuracle.refund_account(data_proof, bucket)
        }

         /// This method is for the DAO to add a protocol that it's running.
         pub fn add_protocol(&mut self, protocol_controller_address: ResourceAddress) {

            self.protocols.push(protocol_controller_address);

            info!("added the protocol with controller badge address {}", protocol_controller_address);

        }

        /// This method is for the DAO to remove a protocol that it doesn't run anymore.
        pub fn remove_protocol(&mut self, protocol_controller_address: ResourceAddress) {

            let index = self.protocols.iter().position(|x| *x == protocol_controller_address);

            match index {

                None => {info!("Doesn't have this protocol on the list.")}

                Some(x) => {

                    self.protocols.remove(x);

                    info!("remove the protocol with controller badge address {}", protocol_controller_address);

                }
            }
        }

        pub fn calculate_voting_power(&self, id_proof: Option<Proof>, cvs_proof: Option<Proof>, stake_amount: Decimal, committed_year: u8) -> Decimal {

            assert!(stake_amount > Decimal::ZERO && committed_year <= self.year_cap, "Wrong input data!");

            let trust_score = match id_proof {
                None => {Decimal::ONE}
                Some(id_proof) => {
                    let ground_id: GroundID = self.ground_id.into();
                    ground_id.check_resource(id_proof.resource_address());
                    let trust_score = id_proof.non_fungible::<Identity>().data().data.trust_factor;
                    id_proof.drop();
                    Decimal::ONE + trust_score
                }
            };

            let aggregrated_cvs_score = match cvs_proof {
                None => {Decimal::ONE}
                Some(cvs_proof) => {
                    let ground_cv: GroundCV = self.ground_cv.into();
                    ground_cv.check_resource(cvs_proof.resource_address());
                    let cvs = cvs_proof.non_fungibles::<CiV>();
                    let mut cvs_data = Vec::new();
                    cvs.iter().for_each(|cv| {
                        cvs_data.push(cv.data().cv_data)
                    });
                    cvs_proof.drop();
                    ground_cv.get_aggregrated_cv_score(cvs_data, self.member_entry_requirement.keys().cloned().collect())
                }
            };

            let commit_factor = expo(self.year_commited_rate, committed_year);

            aggregrated_cvs_score * stake_amount * commit_factor * trust_score

        }

        /// This method will allow user with required unique ID to become DAO member.
        /// Input:
        /// Output: The DAOMember SBT
        /// Reminder! One identity can get more than one DAO member SBT for more flexibility.
        pub fn become_dao_member(&mut self, id_proof: Proof, cvs_proof: Proof, staking_bucket: Bucket, committed_year: u8) -> (Bucket, Proof) {

            assert!(staking_bucket.resource_address() == self.stake_vault.resource_address() && committed_year > 0 && committed_year <= self.year_cap, "Wrong input");

            let ground_id: GroundID = self.ground_id.into();
            ground_id.check_resource(id_proof.resource_address());
            let trust_score = Decimal::ONE + id_proof.non_fungible::<Identity>().data().data.trust_factor;
            id_proof.drop();

            let ground_cv: GroundCV = self.ground_cv.into();
            ground_cv.check_resource(cvs_proof.resource_address());
            let cvs = cvs_proof.non_fungibles::<CiV>();
            let mut cvs_data = Vec::new();
            cvs.iter().for_each(|cv| {
                cvs_data.push(cv.data().cv_data)
            });
            cvs_proof.drop();

            let aggregrated_cvs_score = ground_cv.check_requirements_and_get_aggregrated_cv_score(cvs_data, self.member_entry_requirement.clone());
            
            let current = self.current(self.controller_badge.create_proof());

            let commit_factor = expo(self.year_commited_rate, committed_year);

            let amount = staking_bucket.amount();
            self.stake_vault.put(staking_bucket);
            let voting_power = aggregrated_cvs_score * amount * commit_factor * trust_score;
            info!("You have become a new DAO member!");
            self.total_voting_power += voting_power;

            self.controller_badge.authorize(|| {

                let move_badge = borrow_resource_manager!(self.transient_move_resource).mint(dec!("1"));
                let move_proof = move_badge.create_proof();
                move_badge.burn();

                (borrow_resource_manager!(self.member_sbt)
                    .mint_non_fungible(
                        &NonFungibleId::random(),
                        DAOMember {

                            data: DAOContributorData {
                                voting_power,
                                voted_proposal: HashMap::new(),
                                committed_year,
                                staking_amount: amount,
                                allow_unstaking_time: current + YEAR * committed_year as u64
                            },

                            voting_pool: None

                        }
                ), move_proof)
            })
        }

        pub fn delegate_dao_share(&mut self, id_proof: Option<Proof>, cvs_proof: Option<Proof>, delegate_bucket: Bucket, committed_year: u8) -> Bucket {

            assert!(delegate_bucket.resource_address() == self.stake_vault.resource_address() && committed_year <= self.year_cap, "Wrong input");

            let amount = delegate_bucket.amount();

            let voting_power = self.calculate_voting_power(id_proof, cvs_proof, amount, committed_year);

            let current = self.current(self.controller_badge.create_proof());

            self.stake_vault.put(delegate_bucket);

            self.total_voting_power += voting_power;

            let id = NonFungibleId::random();

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.delegator_badge)
                    .mint_non_fungible(
                        &id,
                        Delegator {

                            data: DAOContributorData {
                                voting_power,
                                voted_proposal: HashMap::new(),
                                committed_year,
                                staking_amount: amount,
                                allow_unstaking_time: current + YEAR * committed_year as u64
                            }

                        }
                )
            })
        }

        /// This method will allow DAO member to re-assess their voting power based on their new id trust score and skill cv acquired.
        /// Input:
        /// Output: The DAOMember SBT
        pub fn reassess_new_vote_power(&mut self, delegator_or_member_proof: Proof, id_proof: Option<Proof>, cvs_proof: Option<Proof>) {

            if delegator_or_member_proof.resource_address() == self.member_sbt {

                let member = delegator_or_member_proof.non_fungible::<DAOMember>();
                let mut data = member.data();

                let old_voting_power = data.data.voting_power;
                let voting_power = self.calculate_voting_power(id_proof, cvs_proof, data.data.staking_amount, data.data.committed_year);
                self.total_voting_power = self.total_voting_power - old_voting_power + voting_power;
                data.data.voting_power = voting_power;

                self.controller_badge.authorize(|| {
                    member.update_data(
                        DAOMember {
                            ..data
                        }
                    )
                })

            } else if delegator_or_member_proof.resource_address() == self.delegator_badge {

                let delegator = delegator_or_member_proof.non_fungible::<Delegator>();
                let mut data = delegator.data();

                let old_voting_power = data.data.voting_power;
                let voting_power = self.calculate_voting_power(id_proof, cvs_proof, data.data.staking_amount, data.data.committed_year);
                self.total_voting_power = self.total_voting_power + voting_power - old_voting_power;
                data.data.voting_power = voting_power;

                self.controller_badge.authorize(|| {
                    delegator.update_data(
                        Delegator {
                            ..data
                        }
                    )
                });
            } else {panic!("Wrong resource!")}
        }

        pub fn new_voting_pool(&mut self, member_proof: Proof, name: String) -> ComponentAddress {

            assert!(member_proof.resource_address() == self.member_sbt, "Wrong resource!");
            let member = member_proof.non_fungible::<DAOMember>();
            let data = member.data();
            assert!(matches!(data.voting_pool, None), "You're already running a voting pool!");
            let dao: ComponentAddress = Runtime::actor().component_address().unwrap();
            let pool_operator = member.address();
            let pool_controller_badge = self.controller_badge.authorize(|| borrow_resource_manager!(self.pool_controller_badge).mint(1));
            let (voting_pool, controller_badge_address) = GroundVotingPool::new(dao, pool_controller_badge, name, pool_operator, self.delegator_badge);
            self.dao_controllers.push(controller_badge_address);

            self.controller_badge.authorize(|| {
                member.update_data(
                    DAOMember {
                        voting_pool: Some(voting_pool),
                        ..data
                    }
                )
            });

            voting_pool

        }

        pub fn auto_swap(&mut self, token: Bucket) -> Bucket {

            if token.resource_address() == self.treasury.0.resource_address() {

                let dx = token.amount() * (Decimal::ONE - self.swap_fee);

                self.treasury.0.put(token);

                let (x, y) = (self.treasury.0.amount(), self.treasury.1.amount());

                let dy = (dx * y) * (x + dx);

                self.treasury.1.take(dy)

            } else if token.resource_address() == self.treasury.1.resource_address() {

                let dx = token.amount() * (Decimal::ONE - self.swap_fee);

                self.treasury.1.put(token);

                let (x, y) = (self.treasury.1.amount(), self.treasury.0.amount());

                let dy = (dx * y) * (x + dx);

                self.treasury.0.take(dy)

            } else {panic!("Wrong resource!")}

        }

        pub fn start_unstake(&mut self, delegator_badge_or_member_sbt: Bucket) -> Bucket {

            if delegator_badge_or_member_sbt.resource_address() == self.member_sbt {

                let current = self.current(self.controller_badge.create_proof());

                let data = delegator_badge_or_member_sbt.non_fungible::<DAOMember>().data().data;

                assert!(data.allow_unstaking_time <= current, "You cannot unstake yet!");

                let amount = data.staking_amount;

                self.total_voting_power -= data.voting_power;

                self.controller_badge.authorize(|| {
                    delegator_badge_or_member_sbt.burn();
                    borrow_resource_manager!(self.unstake_badge)
                        .mint_non_fungible(
                            &NonFungibleId::random(),
                            Unstake {

                                amount: amount,
                                end_time: current + self.unstake_delay

                            }
                    )
                })
            } else if delegator_badge_or_member_sbt.resource_address() == self.delegator_badge {

                let current = self.current(self.controller_badge.create_proof());

                let data = delegator_badge_or_member_sbt.non_fungible::<Delegator>().data().data;

                assert!(data.allow_unstaking_time <= current, "You cannot unstake yet!");

                let amount = data.staking_amount;

                self.total_voting_power -= data.voting_power;

                self.controller_badge.authorize(|| {
                    delegator_badge_or_member_sbt.burn();
                    borrow_resource_manager!(self.unstake_badge)
                        .mint_non_fungible(
                            &NonFungibleId::random(),
                            Unstake {

                                amount: amount,
                                end_time: current + self.unstake_delay

                            }
                    )
                })

            } else {panic!("Wrong resource!")}
        }

        /// This method is for unstaker to withdraw their DAO share token from stake vault
        pub fn unstake(&mut self, unstake_badge: Bucket) -> Bucket {
            assert!(unstake_badge.resource_address() == self.unstake_badge, "Wrong resource!");

            let current = self.current(self.controller_badge.create_proof());

            let data = unstake_badge.non_fungible::<Unstake>().data();
            assert!(data.end_time <= current, "You cannot withdraw your unstake amount yet!");

            self.controller_badge.authorize(|| {
                unstake_badge.burn()
            });

            self.stake_vault.take(data.amount)

        }

        /// This method will allow DAO member to re-assess their voting power based on their new id trust score and skill cv acquired.
        /// Input:
        /// Output: The DAOMember SBT
        pub fn propose_concept(&mut self, proposal_require_cv_proof: Proof, methods: Methods, reward_demand: Decimal) -> Bucket {

            let ground_cv: GroundCV = self.ground_cv.into();

            ground_cv.check_resource(proposal_require_cv_proof.resource_address());

            let cvs = proposal_require_cv_proof.non_fungibles::<CiV>();
            let mut cvs_data = Vec::new();
            cvs.iter().for_each(|cv| {
                cvs_data.push(cv.data().cv_data)
            });
            proposal_require_cv_proof.drop();

            ground_cv.check_cv_requirement(cvs_data, (SCRYPTO_CV, self.proposal_requirement));

            let id = NonFungibleId::from_u64(self.proposal_id_counter);

            self.proposal_book.insert(id.clone(), ConceptData::new());

            info!("You have made proposal no.{} on the DAO's collective action through your list of methods.", id.clone());

            self.proposal_id_counter += 1;

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.proposal_badge)
                    .mint_non_fungible(
                        &id,
                        Proposal {
                            methods,
                            reward_demand
                        }
                )
            })

        }

        pub fn vote(&mut self, member_proof: Proof, propose_id: NonFungibleId, vote: bool) {

            assert!(member_proof.resource_address() == self.member_sbt, "Wrong resource!");

            let concept_data = self.proposal_book.get_mut(&propose_id).expect("Wrong propose id input!");

            let member = member_proof.non_fungible::<DAOMember>();
            let mut data = member.data();

            assert!(matches!(data.data.voted_proposal.get(&propose_id), None), "You have already voted this proposal.");

            let pool_voting_power = if let Some(voting_pool) = data.voting_pool {

                let pool: GroundVotingPool = voting_pool.into();
                ComponentAuthZone::push(member_proof.clone());
                let pool_voting_power = pool.vote_pool(propose_id.clone(), vote);
                ComponentAuthZone::pop().drop();
                pool_voting_power

                } else {Decimal::ZERO};

            let voting_power = data.data.voting_power;

            let total_voting_power = pool_voting_power + voting_power;

            concept_data.total_voted += total_voting_power;

            if vote {
                concept_data.voted_power += total_voting_power
            };
        
            data.data.voted_proposal.insert(propose_id.clone(), VotedData {voted_power: voting_power, vote: vote});
            
            self.controller_badge.authorize(|| {
                member.update_data(
                    DAOMember {
                        ..data
                    }
                )
            });

            member_proof.drop();

            info!("You have voted {} on the concept proposal id {}", vote, propose_id)
            
        }

        pub fn execute_concept(&mut self, proposal_badge: Bucket) -> Option<Bucket> {

            assert!(proposal_badge.resource_address() == self.proposal_badge, "Wrong resource!");
            let id = proposal_badge.non_fungible::<Proposal>().id();
            let data = proposal_badge.non_fungible::<Proposal>().data();
            let concept_data = self.proposal_book.get_mut(&id).expect("The book doesn't contain this proposal");

            let threshold = self.total_voting_power * dec!("2") / dec!("3");

            if concept_data.total_voted < threshold {

                panic!("Not enough voted power on the concept!")

            } else {

                let dao_share_address = self.stake_vault.resource_address();

                let resource_manager = borrow_resource_manager!(dao_share_address);

                let total_supply = resource_manager.total_supply();

                let dividend = total_supply * self.dividend_rate;

                let proof = self.dao_badge.create_proof();

                ComponentAuthZone::push(proof);
                
                if concept_data.voted_power >= threshold {

                    data.methods.call_all();

                    let dev_expo: u8 = ((concept_data.voted_power - threshold) / self.total_voting_power).floor().to_string().parse().expect("Cannot convert dev expo from Decimal to u8");

                    let dev_reward = data.reward_demand * expo(self.dev_expo_reward, dev_expo);

                    concept_data.status = Some(true);

                    self.controller_badge.authorize(|| {
    
                        proposal_badge.burn();
                        let mut bucket = resource_manager.mint(dividend + dev_reward);
                        self.stake_vault.put(bucket.take(dividend));

                        ComponentAuthZone::pop().drop();

                        info!("The concept proposal id {} has passed", id);

                        return Some(bucket)
                
                    })
    
                } else {

                    self.controller_badge.authorize(|| {
    
                        proposal_badge.burn();
                        let bucket = resource_manager.mint(dividend);
                        self.stake_vault.put(bucket);

                        info!("The concept proposal id {} has been rejected", id);
                
                    });

                    concept_data.status = Some(false);

                    ComponentAuthZone::pop().drop();

                    return None

                }
            }
        }

        pub fn calculate_share(&self, data: HashMap<NonFungibleId, VotedData>) -> Decimal {

            let mut share = Decimal::ZERO;

            for x in 0..(self.proposal_id_counter - 1) {
                let id = NonFungibleId::from_u64(x);
                let voted_data = data.get(&id);
                match voted_data {
                    None => {}
                    Some(voted_data) => {

                        let concept_data = self.proposal_book.get(&id).unwrap();
                        let status = concept_data.status;
                        match status {
                            None => {}
                            Some(result) => {
                                if voted_data.vote == result {
                                    share += voted_data.voted_power / concept_data.total_voted;
                                }
                            }
                        }
                    }
                }
            };

            return share

        }

        pub fn take_dividend(&mut self, member_proof: Proof) -> Bucket {

            assert!(member_proof.resource_address() == self.member_sbt, "Wrong resource!");

            let member = member_proof.non_fungible::<DAOMember>();

            let mut data = member.data();

            let vote_data = data.data.voted_proposal;

            let share = self.calculate_share(vote_data);

            data.data.voted_proposal = HashMap::new();

            self.controller_badge.authorize(|| {
                member.update_data(
                    DAOMember {
                        ..data
                    }
                )
            });

            let dividend = self.dividend_vault.take(share);

            return dividend

        }

        pub fn take_dividend_delegator(&mut self, controller_proof: Proof, share: Decimal) -> Bucket {

            assert!(self.dao_controllers.contains(&controller_proof.resource_address()), "Wrong proof!");
            controller_proof.drop();

            self.dividend_vault.take(share)

        }

        pub fn current(&self, controller_proof: Proof) -> u64 {
            assert!(controller_proof.resource_address() == self.controller_badge.resource_address(), "Wrong proof!");
            controller_proof.drop();
            let neuracle: NeuRacle = self.oracle.0.into();
            let data_proof = self.oracle.1.create_proof();
            let current = neuracle.get_data(data_proof);
            current.parse().expect("Wrong data!")
        }

        pub fn change_voting_pool_fee(&mut self, new_fee: Decimal) {
            assert_rate(new_fee);
            self.voting_pool_fee = new_fee
        }

        pub fn change_member_entry_requirement(&mut self, new_requirement: HashMap<u32, u8>) {
            self.member_entry_requirement = new_requirement
        }

        pub fn change_swap_fee(&mut self, new_fee: Decimal) {
            assert_rate(new_fee);
            self.swap_fee = new_fee
        }

        pub fn change_proposal_requirement(&mut self, new: u8) {
            self.proposal_requirement = new
        }

        pub fn change_dev_expo_reward(&mut self, dev_expo_reward: Decimal) {
            assert_rate(dev_expo_reward);
            self.dev_expo_reward = dev_expo_reward
        }

        pub fn change_dividend_rate(&mut self, dividend_rate: Decimal) {
            assert_rate(dividend_rate);
            self.dividend_rate = dividend_rate
        }

        pub fn change_year_commited_rate(&mut self, year_commited_rate: Decimal) {
            assert_rate(year_commited_rate);
            self.year_commited_rate = year_commited_rate
        }

        pub fn change_year_cap(&mut self, year_cap: u8) {
            self.year_cap = year_cap
        }

        pub fn change_unstake_delay(&mut self, unstake_delay: u64) {
            self.unstake_delay = unstake_delay
        }

        pub fn pool_fee(&self) -> Decimal {
            self.voting_pool_fee
        }

        pub fn stable_coin_address(&self) -> ResourceAddress {
            self.treasury.0.resource_address()
        }

        pub fn dao_share_address(&self) -> ResourceAddress {
            self.stake_vault.resource_address()
        }

        pub fn unstake_delay(&self) -> u64 {
            self.unstake_delay
        }

        pub fn proposal_id_counter(&self) -> u64 {
            self.proposal_id_counter
        }

        pub fn year_cap(&self) -> u8 {
            self.year_cap
        }
    }
}