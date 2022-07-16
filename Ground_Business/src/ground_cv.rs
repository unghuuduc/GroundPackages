//! # [GroundCV]: Make a CV Ground for your Web 3's employment!
//! 
//! GroundCV is the blueprint for any organization to help users build a CV Ground in Web3 Society by utilize SBT characteristics. 
//!
//! ## Main Features:
//! The blueprint is for web3 organizations to provide and manage user's CV components through an off-chain KYC process and making use of Soul Bound Tokens (SBTs).
//! 
//! The blueprint also help business operators build their Human Resource Ground in Web3 Society by on-chain Human Resource Service.
//! The organization operate this blueprint is required to protect user's private data.
//! The users must have a unique Person ID SBT. 
//!
//! To operate GroundCV blueprint, the operator is required to use an off-chain CV verification service.
//! 
//! ## Protocol entities:
//! 1. **Service operator**: Main manager of the protocol. Through the blueprint's method, *service operator* is allowed to:
//! - Issue new CV component SBT for any user wish for a CV proof on web3. (require off-chain process)
//! 
//! 2. **Web 3 Business Operators**: Any Web 3 Business Operator can use the protocol's Human Resource Service. Through the blueprint's method, *Web 3 Business Operators* are allowed to:
//! - Get the CV ID from CV component data.
//! - Check the human proof from the provided unique ID SBT.
//! - Check the CV requirement from a list of CV component proofs and CV data requirements.
//! - Get aggregrated CV score from a list of CV component proofs based on input CV id requirements.
//! 
//! ## Utility, Security: 
//! This solution have the same problem as the GroundID blueprint.

use scrypto::prelude::*;
use crate::cv_id_const::*;

/// The data verify user's work experience.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Experience {

    /// position_id: check the cv id const table.
    position_id: u8,
    /// worked year in the position: 0 to 10.
    year: u8

}

/// The data verify user's skill.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Skill {

    /// skill_id: check the cv id const table.
    pub skill_id: u8,
    /// 1 to 10
    pub level: u8

}

/// Education type of a verified graduate.
#[derive(TypeId, Encode, Decode, Describe)]
pub enum EdTypes {

    FullTime,
    PartTime,
    Online

}

/// The data verify user's education background.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Education {

    /// academic_major_id: check the cv id const table.
    academic_major_id: u8,
    university: String,
    ed_type: EdTypes,
    /// graded from 1 to 10
    graduate_grade: u8

}

/// The SBT verify a user's cv component based on type.
#[derive(NonFungibleData)]
pub struct CiV {

    #[scrypto(mutable)]
    pub cv_data: CVdata,

}

/// The enum store CV's data according to it's type
#[derive(TypeId, Encode, Decode, Describe)]
pub enum CVdata {

    /// Education: academic_major_id, university, graduate_grade, ed_type.
    Education(Education),
    /// Experience: position_id, year.
    Experience(Experience),
    /// Skill: skill, level.
    Skill(Skill)

}

impl CVdata {

    pub fn get_cv_id_and_level_from_cvdata(&self) -> (u32, u8) {
            
        match self {
            CVdata::Education(Education {
                academic_major_id,
                university: _,
                ed_type: _,
                graduate_grade
    
            }) => {(EDUCATION + *academic_major_id as u32, *graduate_grade)}

            CVdata::Experience(Experience {
                position_id,
                year
    
            }) => {(EXPERIENCE + *position_id as u32, *year)}

            CVdata::Skill(Skill {
    
                skill_id,
                level
    
            }) => {(SKILL + *skill_id as u32, *level)}
        }
    
    }

}



blueprint! {

    struct GroundCV {

        /// Component controller badge
        controller_badge: Vault,
        /// The CV SBT address
        cv_sbt: ResourceAddress,

    }

    impl GroundCV {
        
        /// This function will create new GroundCV component
        /// ### Input: 
        /// - name: the organization's name.
        /// - admin_badge: the organization admin badge. (the component holding admin badge can be a multisig account or a DAO component).
        /// ### Output: 
        /// Component address.
        pub fn new(
            name: String, 
            admin_badge: ResourceAddress
        ) -> ComponentAddress {

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", name.clone() + "'s Ground CV Controller Badge")
                .initial_supply(dec!(1isize));

            let cv_sbt = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone() + "'s Verified CV Component SBT")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("issue_new_cv_sbt", rule!(require(admin_badge)))
                .default(rule!(allow_all));

            let comp = Self {

                controller_badge: Vault::with_bucket(controller_badge),
                cv_sbt: cv_sbt

            }
            .instantiate()
            .add_access_check(rules)
            .globalize();

            return comp
        }

        /// This method is for the service operator to issue CV SBT for user after a "CV verification" process. 
        /// 
        /// The data can also be fed in through an Oracle.
        /// 
        /// ### Input: 
        /// - cv: the CV type. format: ```Enum(cv_type, Struct(cv_values...))```
        /// ### Output: 
        /// The new CV SBT.
        pub fn issue_new_cv_sbt(&self, cv: CVdata) -> Bucket {

            let id = NonFungibleId::random();
    
            self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.cv_sbt)
                    .mint_non_fungible(
                        &id,
                        CiV{cv_data: cv}
                )
            })
        } 

        /// This method is for Web 3 business operator to check 1 of user's CV requirement.
        pub fn check_cv_requirement(&self, cvs: Vec<CVdata>, requirement: (u32, u8)) { 

            let mut cv_pass = false;

            for cv in cvs {

                let (cv_id, cv_level) = cv.get_cv_id_and_level_from_cvdata();

                cv_pass = cv_pass || (cv_id == requirement.0 && cv_level >= requirement.1)

            };

            assert!(cv_pass, "The cv wasn't met requirement!");

        }

        /// This method is for Web 3 business operator to check a list of user's CV requirements and get the aggregrated cv score.
        pub fn check_requirements_and_get_aggregrated_cv_score(&self, cvs: Vec<CVdata>, requirement: HashMap<u32, u8>) -> Decimal {

            let mut result = Decimal::ZERO;

            let mut times = 0;

            requirement.iter().for_each(|(&id, &level)| {

                times += 1;
                
                let mut cv_pass = false;

                for cv in &cvs {

                    let (cv_id, cv_level) = cv.get_cv_id_and_level_from_cvdata();

                    let check = cv_id == id && cv_level >= level;

                    cv_pass = cv_pass || check;

                    if check {result += dec!("1") + Decimal::from(cv_level) / dec!("10");}

                };

                assert!(cv_pass, "The cv wasn't met requirement!");
        
            });

            result / Decimal::from(times)
        }

        /// This method is for Web 3 business operator to get the aggregrated cv score according to a list of CV component IDs.
        /// 
        /// The aggregrated cv score is calculated by the medium of all the cv level to the range of 1 to 2.
        pub fn get_aggregrated_cv_score(&self, cvs: Vec<CVdata> , id_factors: HashSet<u32>) -> Decimal {

            let mut result = Decimal::ZERO;

            let mut times = 0;
        
            id_factors.iter().for_each(|&id| {

                times += 1;

                for cv in &cvs {

                    let (cv_id, cv_level) = cv.get_cv_id_and_level_from_cvdata();

                    if cv_id == id {result += dec!("1") + Decimal::from(cv_level) / dec!("10");}

                };

            });

            result / Decimal::from(times)
        }

        pub fn check_resource(&self, cv: ResourceAddress) {
            assert!(cv == self.cv_sbt, "Wrong resource!")
        }
    }
}