//! # [GroundID]: Make an Identity Ground for your journey into Web 3! 
//! This is the blueprint for any organization to help user build a identity + trust Ground in Web3 Society by utilizing SBT characteristics. 
//! 
//! ## Main Features:
//! The blueprint is for web3 organizations to manage user's identity through a simple off-chain KYC process and making use of Soul Bound Tokens (SBTs).
//! 
//! ## Protocol entities:
//! 1. **Service operator**: Main manager of the protocol. Through the blueprint's method, *service operator* is allowed to:
//! - Issue new ID SBT for users.
//! - Review Identity data update requests.
//! 
//! To operate GroundID blueprint, the operator is required to use an off-chain unique identity verification service.
//! 
//! Service operator is also required to protect user's private data.
//! 
//! 2. **Users**: Any type of user (Person, Business, Organization,...) wish for a unique identity on web3. Through the blueprint's method, *users* are allowed to:
//! - Make identity data update requests.
//! - Use the identity data update badge (provided by the operator after the request has passed) to update ID SBT data.

use scrypto::prelude::*;

pub fn assert_rate(rate: Decimal) {
    assert!(rate <= dec!("100") && rate > Decimal::ZERO, "Wrong data!");
}

/// The identity type of an user, this could be included more type in the future when needed.
#[derive(TypeId, Encode, Decode, Describe)]
pub enum IdentityType {
    Person, 
    Business,
    Organization
}

impl IdentityType {

    pub fn check_human_proof(&self) {

        assert!(matches!(self, IdentityType::Person), "The ID SBT is not a Person SBT.");

    }

}

/// The SBT keep track of an user's unique identity, yearly income rate and trust score. 
/// 
/// The data can be feeded on-chain through an Oracle and a private data sever.
/// 
/// For now, the ID SBT must be given by a trusted service operator.
/// 
/// ## Uses:
/// Income and trust factor data are needed to algorithmically calculate the maximum credit allowance for user on GroundFi protocol.
/// 
/// Trust factor score is needed for future cubic voting mechanism on GroundFi DAO.
/// 
/// Income and trust factor data is used on-chain given a high possibility that user would be comfortable 
/// with publicing it as long as their other private data (name, age, location,...) is protected.
#[derive(NonFungibleData)]
pub struct Identity {

    /// A workaround way for restrictive proof.
    #[scrypto(mutable)]
    pub data: IdentityData
    
}

/// A workaround way for restrictive proof.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct IdentityData {

    /// User's ID type: Person, Business or Organization.
    pub identity: IdentityType,

    /// User's annualy income amount.
    // #[scrypto(mutable)]
    pub income: Decimal,

    /// An user trust will be scored from 0 to 100. 
    /// 
    /// User's trust is assessed by many factors (information transparency; current job, business industry or organization's purpose; criminal record; social activities; legal contract,...).
    // #[scrypto(mutable)]
    pub trust_factor: Decimal
}

/// The NFT keep track of user's data update request.
#[derive(NonFungibleData)]
pub struct Request {}

/// The NFT badge allow users to update their Identity data.
/// 
/// ## Uses:
/// After made a identity data update request, the ID service operator will review the request.
/// 
/// If passed, user can use this badge to update their ID data.
#[derive(NonFungibleData)]
pub struct IDDataUpdateBadge {
    /// The user's Identity SBT ID
    sbt_id: NonFungibleId,
    /// The user's new income
    income: Decimal,
    /// The user's new trust factor score
    trust_factor: Decimal
}

blueprint! {

    struct GroundID {

        /// Component controller badge
        controller_badge: Vault,
        /// The identity SBT address.
        identity_sbt: ResourceAddress,
        /// Request book for keeping track of update ID data request.
        /// 
        /// ### Format:
        /// ```LazyMap<request_NFT_ID, (identity_SBT_ID, new_income, new_trust_factor, status)>```
        request_book: LazyMap<NonFungibleId, (NonFungibleId, Decimal, Decimal, bool)>,
        /// Request id counter.
        /// 
        /// +1 request = +1 request id
        request_id_counter: u64,
        /// Request badge resource address.
        request_badge: ResourceAddress,
        /// ID update badge resource address.
        id_update_badge: ResourceAddress,
        /// ID update badge vault.
        /// 
        /// After the service operator pass an update ID data request, user can take the ID Data Update Badge from this vault.
        update_badge_vault: Vault

    }

    impl GroundID {
        
        /// This function will create new GroundID component
        /// ### Input: 
        /// - name: the organization's name.
        /// - admin_badge: the service admin badge. (the component holding admin badge can also be a multisig account or a DAO component).
        /// ### Output: 
        /// The component address and the ID SBT resource address (for test purpose).
        pub fn new(name: String, admin_badge: ResourceAddress) -> ComponentAddress {

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Identity Service Controller Badge")
                .initial_supply(dec!(1isize));

            let identity_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s Identity SBT")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            let request_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s ID Date Update Request Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let id_update_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", name +"'s ID Data Update Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("issue_new_id_sbt", rule!(require(admin_badge)))
                .method("review_update_data", rule!(require(admin_badge)))
                .default(rule!(allow_all));

            let comp = Self {

                controller_badge: Vault::with_bucket(controller_badge),
                identity_sbt: identity_sbt,
                request_book: LazyMap::new(),
                request_id_counter: 0,
                request_badge: request_badge,
                id_update_badge: id_update_badge,
                update_badge_vault: Vault::new(id_update_badge)

            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return comp
        }

        /// This method is for the service operator to issue new Identity SBT after an "unique identity verification" process. The data can be fed in through an Oracle.
        /// 
        /// ### Input: 
        /// - **identity**: type of identity (Person, Business or an Organization).
        /// - **income**: yearly income rate of that identity. It can be the estimated amount (if the identity is a business or an organization).
        /// - **trust_factor**: trust factor score of that identity (assessed from the identity's profile, ranged from 0 to 100). 
        /// ### Output: 
        /// - **Bucket**: the new ID SBT.
        pub fn issue_new_id_sbt(&self, identity: IdentityType, income: Decimal, trust_factor: Decimal) -> Bucket {
            
            assert_rate(trust_factor);

            let id = NonFungibleId::random();

            info!("Issued new ID SBT no.{}", id);

            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.identity_sbt)
                    .mint_non_fungible(
                        &id,
                        Identity {
                            data: IdentityData {
                                identity: identity,
                                income: income,
                                trust_factor: trust_factor
                            }
                        }
                )
            })

        } 

        /// This method is for user to request a data update on his Identity SBT.
        /// 
        /// ### Input: 
        /// - **id_sbt**: The Proof of the user's ID SBT.
        /// - **income**: new income amount
        /// - **trust_factor**: new trust_factor score.
        /// ### Output:  
        ///  - **Bucket**: the ID update request badge.
        pub fn request_update_data(&mut self, id_sbt: Proof, income: Decimal, trust_factor: Decimal) -> (Bucket, u64) {

            assert!(id_sbt.resource_address() == self.identity_sbt, "Wrong resource!");

            let sbt_id = id_sbt.non_fungible::<Identity>().id();

            let request_id = self.request_id_counter;

            let id = NonFungibleId::from_u64(request_id);

            self.request_book.insert(id.clone(), (sbt_id.clone(), income, trust_factor, false));

            info!("Created a new ID data update request no.{} by the user no.{}", id.clone(), sbt_id);

            self.request_id_counter += 1;

            id_sbt.drop();

            (self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.request_badge)
                    .mint_non_fungible(&id, Request {})
            }), request_id)

        }

        /// This method is for the service operator to allow an user ID data update if the requested data matched with updated off-chain user data. The data can be fed in through an Oracle.
        /// 
        /// ### Input: 
        /// - **id**: the request ID
        /// - **is_ok**: the request has passed or not
        /// 
        /// ### Output: 
        /// The organization will put the passed ID update badge into the component for users to update the data themselves.
        pub fn review_update_data(&mut self, id: u64, is_ok: bool) {

            let request_id = NonFungibleId::from_u64(id);

            let result = self.request_book.get(&request_id);

            assert!(result.is_some(),
                "The request book doesn't contain this request id."
            );

            let (sbt_id, income, trust_factor, status) = result.unwrap();

            assert!(!status,
                "This request is already reviewed."
            );

            if is_ok {

                info!("The ID data update request no.{} has passed.", request_id);

                self.controller_badge.authorize(|| {
                    self.update_badge_vault.put(
                    borrow_resource_manager!(self.id_update_badge)
                        .mint_non_fungible(&request_id, IDDataUpdateBadge {
                            sbt_id: sbt_id,
                            income: income,
                            trust_factor: trust_factor
                        }))
                })
                
            } else {
                info!("The ID data update request no.{} has been rejected.", request_id);
            }

        }

        /// This method is for the user to get the ID update badge from the component.
        /// 
        /// ### Input: 
        /// - **request_badge**: the request badge bucket
        /// 
        /// ### Output: 
        /// Return None if the request has rejected and the ID data update badge if the request has passed
        pub fn get_update_badge(&mut self, request_badge: Bucket) -> Option<Bucket> {

            assert!(request_badge.resource_address() == self.request_badge, "Wrong resource!");

            let request_id = request_badge.non_fungible::<Request>().id();

            let (_, _, _, status) = self.request_book.get(&request_id).unwrap();

            assert!(status,
                "The organization haven't reviewed your request yet."
            );

            self.controller_badge
                .authorize(|| { 
                    borrow_resource_manager!(self.request_badge)
                        .burn(request_badge);
                });

            if self.update_badge_vault.non_fungible_ids().contains(&request_id) {
                info!("Your data update request no.{} has passed.", request_id);
                Some(self.update_badge_vault.take_non_fungible(&request_id))
            } else {
                info!("Your data update request no.{} has been rejected.", request_id);
                None
            }

        }

        /// This method is for the user to update their ID SBT data.
        /// 
        /// ### Input: 
        /// - **id_sbt**: the Identity SBT proof.
        /// - **update_badge**: the provided ID Data Update Badge.
        /// 
        /// ### Output: 
        /// Edit the Identity SBT data.
        pub fn update_data(&self, id_proof: Proof, update_badge: Bucket) {

            assert!(id_proof.resource_address()==self.identity_sbt && update_badge.resource_address()==self.id_update_badge,
                "Wrong resource."
            );

            let id_sbt = id_proof.non_fungible::<Identity>();

            let update_data = update_badge.non_fungible::<IDDataUpdateBadge>().data();

            assert!(id_sbt.id()==update_data.sbt_id,
                "Wrong Identity SBT proof provided."
            );

            info!("Your new ID data: income: {}, trust factor score: {}", update_data.income, update_data.trust_factor);

            self.controller_badge
                .authorize(|| { 
                    borrow_resource_manager!(self.id_update_badge)
                        .burn(update_badge);
                    id_sbt.update_data(
                        Identity {
                            data: IdentityData {
                                identity: id_sbt.data().data.identity,
                                income: update_data.income,
                                trust_factor: update_data.trust_factor
                            }
                        }
                    )
                });
            
            id_proof.drop();
        }

        /// For easier test
        pub fn get_id(&self, id_proof: Proof) -> NonFungibleId {
            assert!(id_proof.resource_address()==self.identity_sbt,
                "Wrong resource."
            );

            id_proof.non_fungible::<Identity>().id()

        }

        /// Workaround...
        pub fn check_resource(&self, id: ResourceAddress) {
            assert!(id == self.identity_sbt, "Wrong resource!")
        }
    }
}