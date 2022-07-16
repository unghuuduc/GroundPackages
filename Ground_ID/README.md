# [GroundID]: Make an Identity Ground for your journey into Web 3

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Ground ID is the blueprint for any organization to help user build a identity + trust Ground in Web3 Society by utilizing SBT characteristics. 

## Main Features:
The blueprint is for web3 organizations to manage user's identity through an off-chain KYC process by making use of Soul Bound Tokens (SBTs).

### Protocol entities:
1. **Service operator**: Main manager of the protocol. Through the blueprint's method, *service operator* is allowed to:
- Issue new ID SBT for users.
- Review Identity data update requests.

To operate GroundID blueprint, the operator is required to use an off-chain unique identity verification service.

Service operator is also required to protect user's private data.

2. **Users**: Any type of user (Person, Business, Organization,...) wish for a unique identity on web3. Through the blueprint's method, *users* are allowed to:
- Make identity data update requests.
- Use the identity data update badge (provided by the operator after the request has passed) to update ID SBT data.

### Security, Utilities:

***The unique ID service is permissioned, highly depend on an off-chain KYC process, lead to one main problem of centralization.***

Regarding this problem, current DeFi adoption and technology advancement cannot algorithmically assess 
an user's trust through a combination of many non-numeric factor (person's job, business's industry, organization's purpose, criminal record, social activities, legal status,...).

To address any non-numeric factor, these factors also need to be represented as SBTs 
of the community in which the user belong to (work place, local police, local court,...).

Since the tech adoption curve will need a long time to really build a full-fledged Decentralized Society (DeSoc),
in the future, when Web3 Society is adopted enough, 
GroundFi will consider building a blueprint to algorithmically assess the unique identity by only on-chain data through (constellations of SBTs)[https://www.cryptotimes.io/what-are-soulbound-tokens-sbts/].

### Study about the Ground ID package**: 
`cargo doc --no-deps --document-private-items --open`

## License & P/s

This work is licensed under MIT and Apache 2.0.
