# Ground Business: Make a Ground for your Web 3 Business!

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Ground Business is a blueprint package included on-chain CV verificaion service; on-chain human resource service; on-chain business DAO.

## [GroundCV](./src/ground_cv.rs): Make a CV Ground for your Web 3's employment!

GroundCV is the blueprint for any organization to help users build a CV Ground in Web3 Society by utilize SBT characteristics. 

### Main Features:
The blueprint is for web3 organizations to provide and manage user's CV components through an off-chain KYC process and making use of Soul Bound Tokens (SBTs).

The blueprint also help business operators build their human resource Ground in Web3 Society by on-chain human resource service.
The organization operate this blueprint is required to protect user's private data.
The users must have a unique Person ID SBT. 

To operate GroundCV blueprint, the operator is required to use an off-chain CV verification service.

### Protocol entities:
1. **Service operator**: Main manager of the protocol. Through the blueprint's method, *service operator* is allowed to:
- Issue new CV component SBT for any user wish for a CV proof on web3. (require off-chain process)

2. **Web 3 Business Operators**: Any Web 3 Business Operator can use the protocol's human resource service. Through the blueprint's method, *Web 3 Business Operators* are allowed to:
- Get the CV ID from CV component data.
- Check the human proof from the provided unique ID SBT.
- Check the CV requirement from a list of CV component proofs and CV data requirements.
- Get aggregrated CV score from a list of CV component proofs based on input CV id requirements.

## [GroundVotingPool](./src/ground_voting_pool.rs): Build Liquid Democracy Ground on Web3!
GroundVotingPool is the blueprint for any DAO to build Liquid Democracy.

### Main Features:
The blueprint is for any DAO to build Liquid Democracy, keep track of delegator's voting power through making use of Non-Fungible Token (NFT).

### Protocol entities:
1. **Pool operator**: Main manager of the pool. Through the blueprint's method, *pool operator* is allowed to:
- Make a vote in delegators stead.
- Withdraw the pool fee allowed by the DAO.

2. **Delegators**: Any user who hold the DAO share token and want to participate in contributing for the DAO. Through the blueprint's method, *delegators* are allowed to:
- Delegate DAO share token into the pool.
- Reassess the voting power through new ID SBT or CV component SBTs.
- Begin unstake their DAO share.
- Withdraw their DAO share after passed the unstake delay time.
- Take their dividend from the DAO.

## [GroundBusinessDAO]: Make a Ground for your Web 3 Business!

GroundBusinessDAO is the core blueprint of the Ground Business package for any business operator build a business Ground in Web3 Society.

### Main Features:
The blueprint is for web3 business operators to build a DAO for their business. 

Through the blueprint, business operators can maximize business effeciency, earn consumers trust, and promote collective intelligence on Web3.

The blueprint included a Quartic Voting Mechanism which algorithmically calculate the member(or delegator)'s voting power 
through four components: ID trust score, staking amount, committed year, aggregrated CVs component score (with accepted CVs).

The blueprint included a value-driven Proof of Concept Consensus Model.
DAO members are incentivized to agree on collective actions which benefit for the whole

The blueprint included an internal DEX treasury for people to easily exchange between a DAO accepted stablecoin the DAO share token.
The internal DEX works like an automated, internal stock market to reward share holders.

The blueprint included a method for any volunteer or the DAO to 
deposit their protocol's revenue directly into the treasury to raise the 
DAO share price on the internal DEX and benefit the DAO share holders.

The blueprint included a method for DAO protocol's consumers to 
take the compensation directly from the treasury to degrade the 
DAO share price on the internal DEX and put the consequence on the share holders.

The blueprint also utilized the Human Resource Service from GroundCV blueprint, the Oracle solution from NeuRacle blueprint 
and the Liquid Democracy Service from the GroundVotingPool blueprint:

- The Human Resource Service is for the DAO to check entry requirement before made a wallet address became DAO member. 
The service also help the DAO aggregrate the CV id score for the quartic voting mechanism.

- The Oracle solution is for the DAO to keep track of member, voting delegator's committed time for the DAO, 
make sure they cannot withdraw their stake from the DAO until the committed time.

- The Liquid Democracy Service is for the DAO's member to make their own voting pool 
where user who don't meet the member entry requirement can also contribute to the DAO.

### DAO's entities:
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

### Study about the Ground Business package**: 
`cargo doc --no-deps --document-private-items --open`
