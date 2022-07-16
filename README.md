# Peter-s-Work-Space
Private Peter's Work Space

# Ground Finance: Make a Ground for your journey into Web 3!

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

![](./logo.svg)

## Main Features

Ground Finance contains 3 main blueprint packages with a wide-range of usecases built on Radix Ledger: on-chain unique identity service; on-chain lending protocol & credit service; on-chain human resource service & Proof of Concept DAO.

The [Ground_ID](./Ground_ID/) blueprint package contain a blueprint for on-chain unique identity service. For now, the blueprint is a permissioned solution.

The [Ground_Finance](./Ground_Finance/) blueprint package contain two blueprints for on-chain "bank-like" collateral-free lending solution and "consumer-level" credit service with many risk-backed methods.

The [Ground_Business](./Ground_Business/) blueprint package contain three blueprints for on-chain Business DAO with a wide range of on-chain governance technology: Quartic Voting Mechanism; Liquid Democracy; value-driven PoC Consensus Model.

Extra: The [GroundTestEngine](./Ground_Test/) blueprint package is for easy testing of the ground's blueprint packages.

Study more on each package directory.

## Quick Start

Clone this git repository: `git clone https://github.com/unghuuduc/community-scrypto-examples && cd defi/real-estate-manager`

### Unit-test

1. Build the package: `cd Ground_Test && scrypto build`
2. Quick test: `scrypto test`

### PTE test
The test used both the PTE resim client and the PTE Browser Extension, *the test will be running on https://pte01.radixdlt.com/ sever*

**Test Component initializing and get testing resource through resim client:**
1. Connect to the PTE `cd resim-client && resim-client --address pte01-socket.radixdlt.com:8010`
2. Check the test component `resim show 02b1fda21fe1fd0f006b7d449efddf5ad5a6a19b6cde327b0e4dec`. If the test component already existed, go directly to the step 8.
3. Close the resim client and build the test package `cd .. && scrypto build`
4. Connect to the PTE again `cd resim-client && resim-client --address pte01-socket.radixdlt.com:8010` and publish the test package `resim run publish` 
5. Edit the package address, account address on file [instantiate](./Ground_Test/resim-client/instantiate) and run `resim run instantiate`
6. Edit the output component address (Instruction Outputs:) on file [init](./Ground_Test/resim-client/init) and run `resim run init`
7. Recheck the test component by new component address `resim show <ComponentAddress>`
8. Edit the component address, your account address on file [get_test_resources](./Ground_Test/resim-client/get_test_resource) and get the tesing resources for your account by `resim run get_test_resources`.

**Frontend Public Test:**
1. run `cd GroundWeb && npm install`
2. run the UI `npm run dev`
3. Try the UI!

*The frontend is bootstraped with Vite and React.*


