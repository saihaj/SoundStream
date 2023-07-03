mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1::{self as contract, SoundEditions};
use substreams::{log, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

const TRACKED_CONTRACT: [u8; 20] = hex!("aef3e8c8723d9c31863be8de54df2668ef7c4b89");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        ownership_handover_canceleds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::OwnershipHandoverCanceled::match_and_decode(log)
                    {
                        return Some(contract::OwnershipHandoverCanceled {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            pending_owner: event.pending_owner,
                        });
                    }

                    None
                })
            })
            .collect(),
        ownership_handover_requesteds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::OwnershipHandoverRequested::match_and_decode(log)
                    {
                        return Some(contract::OwnershipHandoverRequested {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            pending_owner: event.pending_owner,
                        });
                    }

                    None
                })
            })
            .collect(),
        ownership_transferreds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::OwnershipTransferred::match_and_decode(log)
                    {
                        return Some(contract::OwnershipTransferred {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            new_owner: event.new_owner,
                            old_owner: event.old_owner,
                        });
                    }

                    None
                })
            })
            .collect(),
        roles_updateds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) = abi::contract::events::RolesUpdated::match_and_decode(log)
                    {
                        return Some(contract::RolesUpdated {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            roles: event.roles.to_string(),
                            user: event.user,
                        });
                    }

                    None
                })
            })
            .collect(),
        sound_edition_createds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::SoundEditionCreated::match_and_decode(log)
                    {
                        return Some(contract::SoundEditionCreated {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            contracts: event.contracts.into_iter().map(|x| x).collect::<Vec<_>>(),
                            data: event.data.into_iter().map(|x| x).collect::<Vec<_>>(),
                            deployer: Hex(&event.deployer).to_string(),
                            init_data: event.init_data,
                            results: event.results.into_iter().map(|x| x).collect::<Vec<_>>(),
                            sound_edition: Hex(&event.sound_edition).to_string(),
                            block_number: blk.number,
                        });
                    }

                    None
                })
            })
            .collect(),
        sound_edition_implementation_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::SoundEditionImplementationSet::match_and_decode(log)
                    {
                        return Some(contract::SoundEditionImplementationSet {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            new_implementation: event.new_implementation,
                        });
                    }

                    None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn map_sound_editions(
    blk: eth::Block,
) -> Result<contract::SoundEditions, substreams::errors::Error> {
    Ok(contract::SoundEditions {
        editions: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if log.address != TRACKED_CONTRACT {
                        return None;
                    }

                    if let Some(event) =
                        abi::contract::events::SoundEditionCreated::match_and_decode(log)
                    {
                        return Some(contract::SoundEditionCreated {
                            trx_hash: Hex(&view.transaction.hash).to_string(),
                            log_index: log.block_index,
                            contracts: event.contracts.into_iter().map(|x| x).collect::<Vec<_>>(),
                            data: event.data.into_iter().map(|x| x).collect::<Vec<_>>(),
                            deployer: Hex(&event.deployer).to_string(),
                            init_data: event.init_data,
                            results: event.results.into_iter().map(|x| x).collect::<Vec<_>>(),
                            sound_edition: Hex(&event.sound_edition).to_string(),
                            block_number: blk.number,
                        });
                    }

                    None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn graph_out(
    sound_editions: SoundEditions,
) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    sound_editions.editions.into_iter().for_each(|edition| {
        log::info!("edition: {:?}", edition); // TODO: need to see why this log is not printed?
        tables
            .create_row("SoundEdition", edition.sound_edition)
            .set("transactionHash", edition.trx_hash)
            .set("deployer", edition.deployer)
            .set("blockNumber", edition.block_number);
    });

    Ok(tables.to_entity_changes())
}
