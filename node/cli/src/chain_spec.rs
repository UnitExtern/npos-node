// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use clarus_runtime::{
    constants::currency::*, wasm_binary_unwrap, Block, MaxNominations, SessionKeys, StakerStatus,
};
use grandpa_primitives::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

pub use clarus_runtime::RuntimeGenesisConfig;
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const CLARUS_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
const STASH: Balance = ENDOWMENT / 1000;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
    mixnet: MixnetId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn configure_accounts_for_clarus_testnet() -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
    )>,
    AccountId,
    Vec<AccountId>,
) {
    #[rustfmt::skip]
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
	)> = vec![
        (
			// 5FNCTJVDxfFnmUYKHqbJHjUi7UFbZ6pzC39sL6E5RVpB4vc9
			array_bytes::hex_n_into_unchecked("920c238572e2b31c2efd19dad1a5674c8188388d9a30d0d01847759a5dc64069"),
			// 5GgaLpTUcgbCTGnwVkCjSSzZ5jTaEPuxtWGRDhi8M1BP1hTs
			array_bytes::hex_n_into_unchecked("cc4c78c7f22298f17e0e2dcefb7cff85b30e19dc1699cb9d1de00e5ea65a433d"),
			// 5Fm7Lc3XDxxbH4LBKxn1tf44P1R5M5cm2vmuLZbUnPFLfu5p
			array_bytes::hex2array_unchecked("a3859016b0b17b7ed6a5b2efcb4ce0e2b6b56ec8594d416c0ea3685929f0a15c")
				.unchecked_into(),
			// 5CyLUbfTe941tZDvQQ5AYPXZ6zzqwS987DTwFGnZ3yPFX5wB
			array_bytes::hex2array_unchecked("2824087e4d670acc6f2ac4251736b7fb581b5bff414437b6abc88dc118ea8d5c")
				.unchecked_into(),
			// 5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
			array_bytes::hex2array_unchecked("16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d")
				.unchecked_into(),
			// 5Eeard4qtNM8DBvqDEKn5GBAspbT7QEvhAjxSsYePB26XAiJ
			array_bytes::hex2array_unchecked("724f3e6ec8a61ea3dc5b76c00a049f84fd7f212443b01241e0a2bb4ce503b345")
				.unchecked_into(),
            // 5Eeard4qtNM8DBvqDEKn5GBAspbT7QEvhAjxSsYePB26XAiJ
			array_bytes::hex2array_unchecked("724f3e6ec8a61ea3dc5b76c00a049f84fd7f212443b01241e0a2bb4ce503b345")
            .unchecked_into(),
		),
		(
			// 5DP3mCevjzqrYhJgPpQFkpoERKg55K422u5KiRGPQaoJEgRH
			array_bytes::hex_n_into_unchecked("3a39a8d0654e0f52b2ee8202ed3488e7a82650dde0daadaddbc8ea825e408d13"),
			// 5HeTTicL5u17JCkDhAwcAHUXMGEzXbDLjPYmNC5ahKhwaLgt
			array_bytes::hex_n_into_unchecked("f6eb0cff5244d7437ed659ac34e6ea66daa857f3d1c580f452b8512ae7fdba0f"),
			// 5FKFid7kAaVFkfbpShH8dzw3wJipiuGPruTzc6WB2WKMviUX
			array_bytes::hex2array_unchecked("8fcd640390db86812092a0b2b244aac9d8375be2c0a3434eb9062b58643c60fb")
				.unchecked_into(),
			// 5G4AdD8rQ6MHp2K1L7vF1E43eX69JMZDQ1vknonsALwGQMwW
			array_bytes::hex2array_unchecked("b087cc20818f98e543c55989afccd3ec28c57e425dae970d9dd63cad806c1f6d")
				.unchecked_into(),
			// 5DknzWSQVCpo7bNf2NnBsjb529K2WVpvGv6Q3kn9RgcFgoeQ
			array_bytes::hex2array_unchecked("4acf560d0aa80158ee06971c0ebbf4e6a1a407e6de2df16a003a765b73e63d7b")
				.unchecked_into(),
			// 5DhZENrJzzaJL2MwLsQsvxARhhAPCVXdHxs2oSJuJLxhUsbg
			array_bytes::hex2array_unchecked("485746d4cc0f20b5581f24b30f91b34d49a7b96b85bb8ba202f354aea8e14b1f")
				.unchecked_into(),
            // 5DhZENrJzzaJL2MwLsQsvxARhhAPCVXdHxs2oSJuJLxhUsbg
			array_bytes::hex2array_unchecked("485746d4cc0f20b5581f24b30f91b34d49a7b96b85bb8ba202f354aea8e14b1f")
            .unchecked_into(),
		),
		(
			// 5DJQ1NXeThmu2N5yQHZUsY64Lmgm95nnchpRWi1nSBU2rgod
			array_bytes::hex_n_into_unchecked("36ad94b252606800bc80869baf453663ac2e9276e83f0401107384c053552f3e"),
			// 5EWQq4ns7miu8B8ArsspZ9KBHX6gwjJXptJq5dbLgQucZvdc
			array_bytes::hex_n_into_unchecked("6c1386fd76e4eec0365a439db0decae0d5d715e33db934bc44be28f73df50674"),
			// 5EUsrdaXAAJ87Y7yCRdrYKeyHdTYbSr9tJFCYEy12CNap2v2
			array_bytes::hex2array_unchecked("6ae80477725a1e4f3194fac59286662ea491c9461cb54909432228351be3474a")
				.unchecked_into(),
			// 5FHCHVMPD9VfpzMcGVyL7gqkq2Rd9NomkHFHP8BzP8isUBnh
			array_bytes::hex2array_unchecked("8e3b579b007999dce44a28bb266f73b54e6f7ec219c495ae23fe0dc3c101e158")
				.unchecked_into(),
			// 5GRarw8oivnRh5ViPC9kH6ztbPNiyrfb61BitYz2YzhoqS4L
			array_bytes::hex2array_unchecked("c0dd89e234665e119ac8396af69c37d1956ffbf4a0173c21ee5872fea2366026")
				.unchecked_into(),
			// 5CLfsFaNYPGQvpYkroN1qrWLt54Xpmn6shAxdE45bCy1cvgv
			array_bytes::hex2array_unchecked("0c2d3a4c604c4ad68e285cc1c401dd2665c1cd7193b16d4d9c854c27a9238a1a")
				.unchecked_into(),
            // 5CLfsFaNYPGQvpYkroN1qrWLt54Xpmn6shAxdE45bCy1cvgv
			array_bytes::hex2array_unchecked("0c2d3a4c604c4ad68e285cc1c401dd2665c1cd7193b16d4d9c854c27a9238a1a")
            .unchecked_into(),
		),
	];

    // generated with secret: subkey inspect "$secret"/fir
    let root_key: AccountId = array_bytes::hex_n_into_unchecked(
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809",
    );

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];
    (initial_authorities, root_key, endowed_accounts)
}

fn clarus_testnet_config_genesis() -> serde_json::Value {
    let (initial_authorities, root_key, endowed_accounts) = configure_accounts_for_clarus_testnet();
    testnet_genesis(
        initial_authorities,
        vec![],
        root_key,
        Some(endowed_accounts),
    )
}

/// clarus testnet config
pub fn clarus_chain_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("Clarus Testnet")
        .with_id("Clarus_testnet")
        .with_chain_type(ChainType::Live)
        .with_genesis_config_patch(clarus_testnet_config_genesis())
        .with_telemetry_endpoints(
            TelemetryEndpoints::new(vec![(CLARUS_TELEMETRY_URL.to_string(), 0)])
                .expect("clarus telemetry url is valid; qed"),
        )
        .with_boot_nodes(boot_nodes)
        .with_properties(get_chain_properties())
        .build()
}

/// Returns common properties for all modes
fn get_chain_properties() -> serde_json::map::Map<String, serde_json::Value> {
    let mut properties = serde_json::map::Map::new();
    properties.insert("tokenDecimals".into(), serde_json::json!(4));
    properties.insert("tokenSymbol".into(), serde_json::json!("MYID"));
    properties
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
    )
}

fn configure_accounts(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
    )>,
    initial_nominators: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    stash: Balance,
) -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
    )>,
    Vec<AccountId>,
    usize,
    Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.0.clone(), stash, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                stash,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();

    (
        initial_authorities,
        endowed_accounts,
        num_endowed_accounts,
        stakers,
    )
}

/// Helper function to create RuntimeGenesisConfig json patch for testing.
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
) -> serde_json::Value {
    let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) = configure_accounts(
        initial_authorities,
        initial_nominators,
        endowed_accounts,
        STASH,
    );

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "staking": {
            "validatorCount": initial_authorities.len() as u32,
            "minimumValidatorCount": initial_authorities.len() as u32,
            "invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
            "slashRewardFraction": Perbill::from_percent(10),
            "stakers": stakers.clone(),
        },
        "technicalCommittee": {
            "members": endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect::<Vec<_>>(),
        },
        "sudo": { "key": Some(root_key.clone()) },
        "babe": {
            "epochConfig": Some(clarus_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        "nominationPools": {
            "minCreateBond": 10 * DOLLARS,
            "minJoinBond": 1 * DOLLARS,
        },
    })
}

fn development_config_genesis_json() -> serde_json::Value {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
    )
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("Development")
        .with_id("dev")
        .with_chain_type(ChainType::Development)
        .with_genesis_config_patch(development_config_genesis_json())
        .with_properties(get_chain_properties())
        .build()
}

fn local_testnet_genesis() -> serde_json::Value {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob).
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("Local Testnet")
        .with_id("local_testnet")
        .with_chain_type(ChainType::Local)
        .with_genesis_config_patch(local_testnet_genesis())
        .build()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, NewFullBase};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    /// Local testnet config (single validator - Alice).
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::builder(wasm_binary_unwrap(), Default::default())
            .with_name("Integration Test")
            .with_id("test")
            .with_chain_type(ChainType::Development)
            .with_genesis_config_patch(testnet_genesis(
                vec![authority_keys_from_seed("Alice")],
                vec![],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                None,
            ))
            .build()
    }

    /// Local testnet config (multivalidator Alice + Bob).
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::builder(wasm_binary_unwrap(), Default::default())
            .with_name("Integration Test")
            .with_id("test")
            .with_chain_type(ChainType::Development)
            .with_genesis_config_patch(local_testnet_genesis())
            .build()
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sp_tracing::try_init_simple();

        sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
            let NewFullBase {
                task_manager,
                client,
                network,
                sync,
                transaction_pool,
                ..
            } = new_full_base(config, None, false, |_, _| ())?;
            Ok(sc_service_test::TestNetComponents::new(
                task_manager,
                client,
                network,
                sync,
                transaction_pool,
            ))
        });
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_clarus_test_net_chain_spec() {
        clarus_testnet_config().build_storage().unwrap();
    }
}
