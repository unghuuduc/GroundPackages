# Ground Finance: Make a Ground for your Web 3 Finance!

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Ground Finance is a blueprint package with 2 main usecases: on-chain credit service; on-chain lending protocol.

## Current on-chain problem of uncollateral lending protocols:

Most on-chain lending protocol recent day cannot do uncollateral lending, thus missed the 11 Tn market potential of uncolleteral lending. The problem came from the contradictory between "trust" characteristic of uncollateral lending with on-chain "trustless" characteristic.

Some new projects are trying to challenge the problem through permissioned methods: 
- [Aave](https://docs.aave.com/developers/guides/credit-delegation) credit delegation solution: push the "trust problem" to "lenders".
- [GoldFinch](https://docs.goldfinch.finance/goldfinch/goldfinch-overview) trust through consensus solution: solve the "trust problem" through a consensus from many permissioned entities (Backers, Auditors).
- [Centrifuge Tinlake](https://docs.centrifuge.io/getting-started/centrifuge-at-a-glance/) full permissioned solution for Investors, Issuers and Asset Originators, highly require off-chain "trust".
- [TrueFi](https://blog.trusttoken.com/introducing-truefi-the-defi-protocol-for-uncollateralized-lending-9bfd6594a48)  permissioned solution for institution borrowers, voted through by a DAO and provide a risk-backed solution for lenders.

Although Ground Finance also used permissioned solutions, it combined the best charateristic of these 4 uncollateral lending solutions and evolved them into on-chain "consumer level" credit and "bank level" earning tracker while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.

## [GroundCredit](./src/ground_credit.rs): Make a Credit Ground for your journey into Web 3!

Ground Credit is the blueprint for any organization to help users build a credit Ground in Web3 Society by utilizing SBT characteristics. 

### Main Features:

The blueprint is for web3 organizations to manage user's credit through making use of Soul Bound Tokens (SBTs). 

The blueprint included installment type credit, allow [TrueFi](https://truefi.io/) level credit. 

The blueprint also included two revolving credit types: "Monthly" and "Yearly", allow on-chain "consumer level" credit for borrowers.

### Protocol entities:

1. **Credit service operator**: Main manager of the protocol. Through the blueprint's method, *Credit service operator* is allowed to:
- Issue new Credit SBT for users (for user who wish to migrate his off-chain credit history). (Require off-chain process)
- Review installment credit request. (Require off-chain process)
- List, delist a lending protocol to use the Credit service. (Require off-chain process if the protocols weren't run by the same entity)
- Blacklist, whitelist credit users who have issue with the ID SBT (wrong income, trust score) or have a large loan default. (Require off-chain process)
- Change the credit degrade and restore rate when credit users have late (or on-time) repayment frequency.

Service operator is also required to protect user's private data.

2. **Credit users**: Verified unique identity on web3 who wish to use on-chain credit or take a loan. Through the blueprint's method, *Credit users* are allowed to:
- Use the ID SBT to take new credit SBT.
- Change credit type ("Monthly" or "Yearly") (Require no-debt credit status).
- Check the maximum credit and current credit allowance.
- Request an installment credit.
- Take the installment credit badge after the request has passed.

<!-- 3. **Lending protocols**: Listed lending protocols can use this blueprint for on-chain credit service. Through the blueprint's method, *Lending protocols* are allowed to:
- Automatically evaluate user's credit score through late (or on-time) repayment frequency. 
- Edit user's current debt or the credit's due time.
- Let protocol users use the installment credit badge to change credit into installment type (Require no-debt credit status).
- Let protocol users stop using installment credit and change the credit back into revolving type. (Now not working on Credit blueprint) -->

3. **Lending protocols**: Listed lending protocols can use this blueprint for on-chain credit service. Through the blueprint's method, *Lending protocols* are allowed to:
- Edit the Credit data and burn the Installment Credit Badge.

## [GroundLending](./src/ground_lending.rs): Make a Ground for your Web 3 Finance!

Ground Lending is the core blueprint of the Ground Finance package, provide collateral-free lending solution to maximize capital efficiency for borrowers and earn rates for lenders, allow on-chain "bank level" earning tracker while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.

### Main Features

The blueprint is for web3 organizations to instantiate and manage a collateral-free lending protocol on-chain. 

The blueprint utilized the Credit Service from GroundCredit blueprint, the Oracle solution from NeuRacle blueprint 
and the business DAO solution from GroundBusinessDAO blueprint:

- The Credit Service is for the protocol to keep track and update the borrower's credit data: current debt (include initial debt, debt interest and extra debt by late repayment), credit score, credit due time, credit start time.

- The Oracle solution is for the protocol to keep track on the passage of time, to see which repayment is on-time (or late) and which lending accounts are eligible for the interest from borrowers, enable "bank level" earning tracker for lenders.

- The DAO solution is to run the protocol by collective actions, reduce human "bias" in the lending protocol. 

The DAO also provide a "risk-backed" method called "compensate" which will compensate lenders a part of their lending, taken directly from the DAO treasury in case of cooperated loan defaults.

### Protocol Entities:
1. **Protocol Operator**: Main manager of the protocol (can also be a DAO). Through the blueprint's method, *protocol operator* is allowed to:
- Change the DAO component address the protocol is using.
- Change the Oracle component address the protocol is using.
- Funding the Oracle account from a badge received from that Oracle.
- Change the protocol's revolving credit interest rates.
- Change the protocol's fee and compensate rate.
- Change the protocol's tolerance threshold (the minimum remained percent in protocol's vault allowed for user to take a loan).
- Take the protocol's fee.
- Deposit a stable coin bucket into the protocol's vault to support the protocol in case of loan default.


- Evaluate user's credit score through late (or on-time) repayment frequency. 
- Edit user's current debt or the credit's due time.
- Let protocol users use the installment credit badge to change credit into installment type (Require no-debt credit status).
- Let protocol users stop using installment credit and change the credit back into revolving type.

2. **Lenders**: Any wallet address (permissionless) wish to lend the protocol their stable coin to maximize earn rates. Through the blueprint's method, *lenders* are allowed to:
- Lend an amount of stable coins into the protocol to earn interest and get the Account badge.
- Withdraw part of (or all) the return amount from the Account badge.
- Take the compensate amount from the DAO running this protocol in the worse case of cooperated loan default.

3. **Borrowers**: Permissioned wallet address (require ID SBT and Credit SBT) can make an automated collateral-free 
loan through this blueprint to maximize capital efficiency. 
Through the blueprint's method, *borrowers* are allowed to:
- Use the revolving credit SBT to take the revolving loan
- Use the installment credit badge to take the installment loan and change credit SBT into installment type.
- Get the current total debt (the debt is increased if user's late on repayment).
- Repay part of the current debt or repay in full.

## Security, Utility

### Dynamic credit types, enable "consumer-level" credit for borrowers:
The Credit service blueprint has two credit type: "Revolving Credit" and "Installment Credit".

- Revolving Credit is allowed for any on-chain unique Identity with the income data (require an unique Identity SBT), maximum credit allowance will be calculated by a cubic function from 3 params: user's income, user's credit score, user's Identity trust score.

- Installment Credit is permissioned, only allowed for off-chain entity that likely need a legal procedure to protect the lending protocol from delinquent loan.

### Automatic credit scoring mechanism:
The Ground Credit blueprint included an Automatic credit scoring mechanism:
- Credit user who is late on repayment will automatically get his credit score degraded.
- Credit user who has on-time repayment frequency and the total repayment reach the maximum allowance will get his credit score restored.

### "Bank level" earning tracker for lenders: 
Lenders can only earn the interest if their lending time on the protocol cover the **borrowers**'s borrowing time. Precisely, only when borrower borrow after a lender has lended their token on protocol and the lender won't withdraw the token until the borrower made repayment, that lender would earn the interest rate.

This is a "bank-like" utility that will incentive lenders to lend their money on the bank for a long time or they would not get the interest. This will also reduce mass-withdrawal risk from the protocol.

### Risk-tolerance mechanism:

The Ground Lending blueprint included a Risk-tolerance mechanism, introduce a risk-tolerance threshold that prevent borrowers from getting a loan pass that risk-tolerance threshold. 

Specifically, if the threshold is 60%, all the borrower's current loan can never be >40% of the protocol's total asset.

### Risk-backed compensation
Although it's permissionless for lenders, all borrowers require on-chain unique identity and thus have to use the [Ground ID](../Ground_ID/) service, which converge into the centralization problem. Ground Finance cannot achive [Decentralized Credit](https://cointelegraph.com/news/decentralized-credit-scores-how-can-blockchain-tech-change-ratings) yet and vulnerable to "single point of failure" from the Identity service provider.

Even if the Ground Finance protocol and the Ground ID service are ran by the same DAO, bringing unique identity on-chain require human interaction (because identity is a "given" thing by other, like how our name are given by our parents) and thus still vulnerable to human "bias".

Confronting such risk, the protocol included a compensate method which utilized [GroundBusiness](../Ground_Business/) package to back the protocol through the DAO's treasury.

### Study more about the Ground Finance package**: 
`cargo doc --no-deps --document-private-items --open`

## License & P/s

This work is licensed under MIT and Apache 2.0.
