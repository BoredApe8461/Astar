// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

use frame_system::RawOrigin;
use sp_runtime::BoundedVec;
use nfts_chain_extension_types::{select_origin, Origin, Outcome};
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use pallet_uniques::DestroyWitness;
use parity_scale_codec::Encode;
use sp_runtime::traits::StaticLookup;
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

type AccountIdLookup<T> = <<T as SysConfig>::Lookup as StaticLookup>::Source;

enum NftsFunc {
    Create,
    Transfer,
    Mint,
    Redeposit,
    Burn,
    Destroy,
    ApproveTransfer,
    CancelApproval,
    Freeze,
    Thaw,
    FreezeCollection,
    ThawCollection,
    TransferOwnership,
    SetTeam,
    SetAttribute,
    ClearAttribute,
    SetMetadata,
    ClearMetadata,
    SetCollectionMetadata,
    ClearCollectionMetadata,
    SetAcceptOwnership,
    SetCollectionMaxSupply,
    Collection,
    OwnershipAcceptance,
    Account,
    CollectionAccount,
    Item,
    CollectionMetadataOf,
    ItemMetadataOf,
    Attribute,
    ItemPriceOf,
    CollectionMaxSupply,
}

impl TryFrom<u16> for NftsFunc {
    type Error = DispatchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NftsFunc::Create),
            2 => Ok(NftsFunc::Transfer),
            3 => Ok(NftsFunc::Mint),
            4 => Ok(NftsFunc::Redeposit),
            5 => Ok(NftsFunc::Burn),
            6 => Ok(NftsFunc::Destroy),
            7 => Ok(NftsFunc::ApproveTransfer),
            8 => Ok(NftsFunc::CancelApproval),
            9 => Ok(NftsFunc::Freeze),
            10 => Ok(NftsFunc::Thaw),
            11 => Ok(NftsFunc::FreezeCollection),
            12 => Ok(NftsFunc::ThawCollection),
            13 => Ok(NftsFunc::TransferOwnership),
            14 => Ok(NftsFunc::SetTeam),
            15 => Ok(NftsFunc::SetAttribute),
            16 => Ok(NftsFunc::ClearAttribute),
            17 => Ok(NftsFunc::SetMetadata),
            18 => Ok(NftsFunc::ClearMetadata),
            19 => Ok(NftsFunc::SetCollectionMetadata),
            20 => Ok(NftsFunc::ClearCollectionMetadata),
            21 => Ok(NftsFunc::SetAcceptOwnership),
            22 => Ok(NftsFunc::SetCollectionMaxSupply),
            23 => Ok(NftsFunc::Collection),
            24 => Ok(NftsFunc::OwnershipAcceptance),
            25 => Ok(NftsFunc::Account),
            26 => Ok(NftsFunc::CollectionAccount),
            27 => Ok(NftsFunc::Item),
            28 => Ok(NftsFunc::CollectionMetadataOf),
            29 => Ok(NftsFunc::ItemMetadataOf),
            30 => Ok(NftsFunc::Attribute),
            31 => Ok(NftsFunc::ItemPriceOf),
            32 => Ok(NftsFunc::CollectionMaxSupply),
            _ => Err(DispatchError::Other("Unimplemented func_id for NftsFunc")),
        }
    }
}

/// Pallet Nfts chain extension.
pub struct NftsExtension<T, W>(PhantomData<(T, W)>);

impl<T, W> Default for NftsExtension<T, W> {
    fn default() -> Self {
        NftsExtension(PhantomData)
    }
}

impl<T, W> ChainExtension<T> for NftsExtension<T, W>
where
    T: pallet_uniques::Config + pallet_contracts::Config,
    AccountIdLookup<T>: From<<T as SysConfig>::AccountId>,
    <T as SysConfig>::AccountId: From<[u8; 32]>,
    W: weights::WeightInfo,
{
    fn call<E: Ext>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        E: Ext<T = T>,
    {
        let func_id = env.func_id().try_into()?;
        let mut env = env.buf_in_buf_out();

        match func_id {
            NftsFunc::Create => {
                let (origin, collection_id, admin): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::create(
                    raw_origin.into(),
                    collection_id.into(),
                    admin.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            }
            NftsFunc::Destroy => {
                let (origin, collection_id, witness): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    DestroyWitness,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::destroy(
                    raw_origin.into(),
                    collection_id.into(),
                    witness,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            }
            NftsFunc::Transfer => {
                let (origin, collection_id, item, dest): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::transfer(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    dest.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            }
            NftsFunc::Mint => {
                let (origin, collection_id, item, owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::mint(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    owner.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            }
            NftsFunc::Burn => {
                let (origin, collection_id, item, check_owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    Option<T::AccountId>,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let check_owner: Option<AccountIdLookup<T>> = check_owner.map(|owner| owner.into());
                let call_result = pallet_uniques::Pallet::<T>::burn(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    check_owner.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::Redeposit => {
                let (origin, collection_id, items): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Vec<<T as pallet_uniques::Config>::ItemId>,
                ) = env.read_as_unbounded(env.in_len())?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::redeposit(
                    raw_origin.into(),
                    collection_id.into(),
                    items,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::Freeze => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::freeze(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::Thaw => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::thaw(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::FreezeCollection => {
                let (origin, collection_id): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::freeze_collection(
                    raw_origin.into(),
                    collection_id.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::ThawCollection => {
                let (origin, collection_id): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::thaw_collection(
                    raw_origin.into(),
                    collection_id.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::TransferOwnership => {
                let (origin, collection_id, owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::transfer_ownership(
                    raw_origin.into(),
                    collection_id.into(),
                    owner.into()
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetTeam => {
                let (origin, collection_id, issuer, admin, freezer): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                    T::AccountId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_team(
                    raw_origin.into(),
                    collection_id.into(),
                    issuer.into(),
                    admin.into(),
                    freezer.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::ApproveTransfer => {
                let (origin, collection_id, item, delegate): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::approve_transfer(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    delegate.into(),
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::CancelApproval => {
                let (origin, collection_id, item, maybe_check_delegate): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    Option<T::AccountId>,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let maybe_check_delegate: Option<AccountIdLookup<T>> = maybe_check_delegate.map(|d| d.into());
                let call_result = pallet_uniques::Pallet::<T>::cancel_approval(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    maybe_check_delegate,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetAttribute => {
                let (origin, collection_id, maybe_item, key, value): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Option<<T as pallet_uniques::Config>::ItemId>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_attribute(
                    raw_origin.into(),
                    collection_id.into(),
                    maybe_item,
                    key,
                    value
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::ClearAttribute => {
                let (origin, collection_id, maybe_item, key): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Option<<T as pallet_uniques::Config>::ItemId>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::clear_attribute(
                    raw_origin.into(),
                    collection_id.into(),
                    maybe_item,
                    key,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetMetadata => {
                let (origin, collection_id, item, data, is_frozen): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
                    bool
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_metadata(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                    data,
                    is_frozen,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::ClearMetadata => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::clear_metadata(
                    raw_origin.into(),
                    collection_id.into(),
                    item,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetCollectionMetadata => {
                let (origin, collection_id, data, is_frozen): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
                    bool
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_collection_metadata(
                    raw_origin.into(),
                    collection_id.into(),
                    data,
                    is_frozen,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetAcceptOwnership => {
                let (origin, maybe_collection_id): (
                    Origin,
                    Option<<T as pallet_uniques::Config>::CollectionId>,
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_accept_ownership(
                    raw_origin.into(),
                    maybe_collection_id,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            NftsFunc::SetCollectionMaxSupply => {
                let (origin, collection_id, max_supply): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    u32 
                ) = env.read_as()?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_collection_max_supply(
                    raw_origin.into(),
                    collection_id,
                    max_supply,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            },
            _ => todo!(),
        }

        Ok(RetVal::Converging(Outcome::Success as u32))
    }
}
