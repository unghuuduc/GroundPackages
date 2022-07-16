//! # [GroundVotingPool]: Build Liquid Democracy Ground on Web3!
//! GroundVotingPool is the blueprint for any DAO to build Liquid Democracy.
//!
//! ## Main Features:
//! The blueprint is for any DAO to build Liquid Democracy, keep track of delegator's voting power through making use of Non-Fungible Token (NFT).
//!
//! ## Protocol entities:
//! 1. **Pool operator**: Main manager of the pool. Through the blueprint's method, *pool operator* is allowed to:
//! - Make a vote in delegators stead.
//! - Withdraw the pool fee allowed by the DAO.
//!
//! 2. **Delegators**: Any user who hold the DAO share token and want to participate in contributing for the DAO can become delegators. Through the blueprint's method, *delegators* are allowed to:
//! - Delegate DAO share token into the pool.
//! - Reassess the voting power through new ID SBT or CV component SBTs.
//! - Begin unstake their DAO share.
//! - Withdraw their DAO share after passed the unstake delay time.
//! - Take their dividend from the DAO.

use scrypto::prelude::*;
use crate::ground_business_dao::*;

#[derive(NonFungibleData)]
pub struct PoolDelegator {
    id: NonFungibleId
}

blueprint! {

    struct GroundVotingPool {

        /// Store delegator's badge.
        delegators: Vault,
        /// Delegator badge resource address
        delegator_badge: ResourceAddress,
        /// Pool Delegator badge resource address
        pool_delegator: ResourceAddress,
        /// Fee for pool operator
        fee_vault: Vault,
        /// The pool unstake badge address
        unstake_badge: ResourceAddress,
        /// VotingPool controller badge
        controller_badge: Vault,
        /// The DAO address
        dao: ComponentAddress,
        /// Total voting power of the pool
        total_voting_power: Decimal

    }

    impl GroundVotingPool {
        
        pub fn new(dao: ComponentAddress, controller_badge: Bucket, name: String, pool_operator: NonFungibleAddress, delegator_badge: ResourceAddress) -> (ComponentAddress, ResourceAddress) {

            let gdao: GroundBusinessDAO = dao.into();
            let dao_share = gdao.dao_share_address();

            let unstake_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s DAO Unstake Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let pool_delegator_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "Pool Delegator Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let controller_badge_address = controller_badge.resource_address();

            let rules = AccessRules::new()
                .method("vote_pool", rule!(require(pool_operator.clone())))
                .method("take_fee", rule!(require(pool_operator)))
                .default(rule!(allow_all));

            let component = Self {
                
                    delegators: Vault::new(delegator_badge),
                    delegator_badge: delegator_badge,
                    pool_delegator: pool_delegator_badge,
                    fee_vault: Vault::new(dao_share),
                    unstake_badge: unstake_badge,
                    controller_badge: Vault::with_bucket(controller_badge),
                    dao: dao,
                    total_voting_power: Decimal::ZERO

                }
                .instantiate()
                .add_access_check(rules)
                .globalize();

            return (component, controller_badge_address)
            
        }

        pub fn delegate(&mut self, delegator_badge: Bucket) -> Bucket {
            assert!(delegator_badge.resource_address() == self.delegator_badge, "Wrong resource");
            let delegator = delegator_badge.non_fungible::<Delegator>();
            let id = delegator.id();
            let voting_power = delegator.data().data.voting_power;
            self.total_voting_power += voting_power;
            self.delegators.put(delegator_badge);
            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.pool_delegator)
                .mint_non_fungible(&NonFungibleId::random(), PoolDelegator {id})
            })
        }

        pub fn stop_delegate(&mut self, pool_delegator_badge: Bucket) -> Bucket {
            assert!(pool_delegator_badge.resource_address() == self.pool_delegator, "Wrong resource");
            let id = pool_delegator_badge.non_fungible::<PoolDelegator>().data().id;
            self.controller_badge.authorize(|| {
                pool_delegator_badge.burn()
            });
            self.delegators.take_non_fungible(&id)
        }

        pub fn vote_pool(&mut self, id: NonFungibleId, vote: bool) -> Decimal {

            self.controller_badge.authorize(|| {
                for delegator in self.delegators.non_fungibles::<Delegator>() {
                    let mut data = delegator.data();
                    let voting_power = data.data.voting_power;
                    data.data.voted_proposal.insert(id.clone(), VotedData {voted_power: voting_power, vote: vote});
                    delegator.update_data(Delegator {..data})
                }
            });

            self.total_voting_power
        }

        pub fn take_dividend(&mut self, pool_delegator_proof: Proof) -> Bucket {

            assert!(pool_delegator_proof.resource_address() == self.pool_delegator, "Wrong resource!");

            let id = pool_delegator_proof.non_fungible::<PoolDelegator>().data().id;

            let dao: GroundBusinessDAO = self.dao.into();

            let delegator_badge =  self.delegators.take_non_fungible(&id);

            let delegator = delegator_badge.non_fungible::<Delegator>();

            let mut data = delegator.data();

            let vote_data = data.data.voted_proposal;

            let share = dao.calculate_share(vote_data);

            data.data.voted_proposal = HashMap::new();

            self.controller_badge.authorize(|| {
                delegator.update_data(
                    Delegator {
                        ..data
                    }
                )
            });

            self.delegators.put(delegator_badge);

            let mut dividend = dao.take_dividend_delegator(self.controller_badge.create_proof(), share);

            let amount = dividend.amount();

            self.fee_vault.put(dividend.take(amount * dao.pool_fee()));

            return dividend

        }

        pub fn take_fee(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }

    }
}
