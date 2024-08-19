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


use frame_support::traits::nonfungibles::{Inspect, InspectEnumerable};
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use pallet_uniques::{DestroyWitness, WeightInfo};
use parity_scale_codec::Encode;
use sp_runtime::traits::StaticLookup;
use sp_runtime::BoundedVec;
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

use uniques_chain_extension_types::{select_origin, Origin, Outcome};

type AccountIdLookup<T> = <<T as SysConfig>::Lookup as StaticLookup>::Source;

enum UniquesFunc {
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
    Owner,
    CollectionOwner,
    Attribute,
    CollectionAttribute,
    CanTransfer,
    Collections,
    Items,
    Owned,
    OwnedInCollection,
}

impl TryFrom<u16> for UniquesFunc {
    type Error = DispatchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UniquesFunc::Create),
            2 => Ok(UniquesFunc::Transfer),
            3 => Ok(UniquesFunc::Mint),
            4 => Ok(UniquesFunc::Redeposit),
            5 => Ok(UniquesFunc::Burn),
            6 => Ok(UniquesFunc::Destroy),
            7 => Ok(UniquesFunc::ApproveTransfer),
            8 => Ok(UniquesFunc::CancelApproval),
            9 => Ok(UniquesFunc::Freeze),
            10 => Ok(UniquesFunc::Thaw),
            11 => Ok(UniquesFunc::FreezeCollection),
            12 => Ok(UniquesFunc::ThawCollection),
            13 => Ok(UniquesFunc::TransferOwnership),
            14 => Ok(UniquesFunc::SetTeam),
            15 => Ok(UniquesFunc::SetAttribute),
            16 => Ok(UniquesFunc::ClearAttribute),
            17 => Ok(UniquesFunc::SetMetadata),
            18 => Ok(UniquesFunc::ClearMetadata),
            19 => Ok(UniquesFunc::SetCollectionMetadata),
            20 => Ok(UniquesFunc::ClearCollectionMetadata),
            21 => Ok(UniquesFunc::SetAcceptOwnership),
            22 => Ok(UniquesFunc::SetCollectionMaxSupply),
            23 => Ok(UniquesFunc::Owner),
            24 => Ok(UniquesFunc::CollectionOwner),
            25 => Ok(UniquesFunc::Attribute),
            26 => Ok(UniquesFunc::CollectionAttribute),
            27 => Ok(UniquesFunc::CanTransfer),
            28 => Ok(UniquesFunc::Collections),
            29 => Ok(UniquesFunc::Items),
            30 => Ok(UniquesFunc::Owned),
            31 => Ok(UniquesFunc::OwnedInCollection),
            _ => Err(DispatchError::Other(
                "Unimplemented func_id for UniquesFunc",
            )),
        }
    }
}

/// Pallet Uniques chain extension.
pub struct UniquesExtension<T, W>(PhantomData<(T, W)>);

impl<T, W> Default for UniquesExtension<T, W> {
    fn default() -> Self {
        UniquesExtension(PhantomData)
    }
}

impl<T, W> ChainExtension<T> for UniquesExtension<T, W>
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
            UniquesFunc::Create => {
                let (origin, collection_id, admin): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::create();
                env.charge_weight(base_weight)?;

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
            UniquesFunc::Destroy => {
                let (origin, collection_id, witness): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    DestroyWitness,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::destroy(
                    witness.items,
                    witness.item_metadatas,
                    witness.attributes,
                );
                env.charge_weight(base_weight)?;

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
            UniquesFunc::Transfer => {
                let (origin, collection_id, item, dest): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::transfer();
                env.charge_weight(base_weight)?;

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
            UniquesFunc::Mint => {
                let (origin, collection_id, item, owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::mint();
                env.charge_weight(base_weight)?;

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
            UniquesFunc::Burn => {
                let (origin, collection_id, item, check_owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    Option<T::AccountId>,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::burn();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::Redeposit => {
                let (origin, collection_id, items): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Vec<<T as pallet_uniques::Config>::ItemId>,
                ) = env.read_as_unbounded(env.in_len())?;

                let base_weight =
                    <T as pallet_uniques::Config>::WeightInfo::redeposit(items.len() as u32);
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::Freeze => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::freeze();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::Thaw => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::thaw();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::FreezeCollection => {
                let (origin, collection_id): (Origin, <T as pallet_uniques::Config>::CollectionId) =
                    env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::freeze_collection();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::ThawCollection => {
                let (origin, collection_id): (Origin, <T as pallet_uniques::Config>::CollectionId) =
                    env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::thaw_collection();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::TransferOwnership => {
                let (origin, collection_id, owner): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::transfer_ownership();
                env.charge_weight(base_weight)?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::transfer_ownership(
                    raw_origin.into(),
                    collection_id.into(),
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
            UniquesFunc::SetTeam => {
                let (origin, collection_id, issuer, admin, freezer): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    T::AccountId,
                    T::AccountId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::set_team();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::ApproveTransfer => {
                let (origin, collection_id, item, delegate): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    T::AccountId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::approve_transfer();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::CancelApproval => {
                let (origin, collection_id, item, maybe_check_delegate): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    Option<T::AccountId>,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::cancel_approval();
                env.charge_weight(base_weight)?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let maybe_check_delegate: Option<AccountIdLookup<T>> =
                    maybe_check_delegate.map(|d| d.into());
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
            }
            UniquesFunc::SetAttribute => {
                let (origin, collection_id, maybe_item, key, value): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Option<<T as pallet_uniques::Config>::ItemId>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::set_attribute();
                env.charge_weight(base_weight)?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::set_attribute(
                    raw_origin.into(),
                    collection_id.into(),
                    maybe_item,
                    key,
                    value,
                );

                return match call_result {
                    Err(e) => {
                        let mapped_error = Outcome::from(e);
                        Ok(RetVal::Converging(mapped_error as u32))
                    }
                    Ok(_) => Ok(RetVal::Converging(Outcome::Success as u32)),
                };
            }
            UniquesFunc::ClearAttribute => {
                let (origin, collection_id, maybe_item, key): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    Option<<T as pallet_uniques::Config>::ItemId>,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::clear_attribute();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::SetMetadata => {
                let (origin, collection_id, item, data, is_frozen): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
                    bool,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::set_metadata();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::ClearMetadata => {
                let (origin, collection_id, item): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::clear_metadata();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::SetCollectionMetadata => {
                let (origin, collection_id, data, is_frozen): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
                    bool,
                ) = env.read_as()?;

                let base_weight =
                    <T as pallet_uniques::Config>::WeightInfo::set_collection_metadata();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::ClearCollectionMetadata => {
                let (origin, collection_id): (Origin, <T as pallet_uniques::Config>::CollectionId) =
                    env.read_as()?;

                let base_weight =
                    <T as pallet_uniques::Config>::WeightInfo::clear_collection_metadata();
                env.charge_weight(base_weight)?;

                let raw_origin = select_origin!(&origin, env.ext().address().clone());

                let call_result = pallet_uniques::Pallet::<T>::clear_collection_metadata(
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
            }
            UniquesFunc::SetAcceptOwnership => {
                let (origin, maybe_collection_id): (
                    Origin,
                    Option<<T as pallet_uniques::Config>::CollectionId>,
                ) = env.read_as()?;

                let base_weight = <T as pallet_uniques::Config>::WeightInfo::set_accept_ownership();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::SetCollectionMaxSupply => {
                let (origin, collection_id, max_supply): (
                    Origin,
                    <T as pallet_uniques::Config>::CollectionId,
                    u32,
                ) = env.read_as()?;

                let base_weight =
                    <T as pallet_uniques::Config>::WeightInfo::set_collection_max_supply();
                env.charge_weight(base_weight)?;

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
            }
            UniquesFunc::Owner => {
                let (collection_id, item): (
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::owner();
                env.charge_weight(base_weight)?;

                let owner = pallet_uniques::Pallet::<T>::owner(collection_id, item);
                env.write(&owner.encode(), false, None)?;
            }
            UniquesFunc::CollectionOwner => {
                let collection_id: <T as pallet_uniques::Config>::CollectionId = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::collection_owner();
                env.charge_weight(base_weight)?;

                let owner = pallet_uniques::Pallet::<T>::collection_owner(collection_id);
                env.write(&owner.encode(), false, None)?;
            }
            UniquesFunc::Attribute => {
                let (collection_id, item, key): (
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                ) = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::attribute();
                env.charge_weight(base_weight)?;

                let attribute = pallet_uniques::Pallet::<T>::attribute(&collection_id, &item, &key);
                env.write(&attribute.encode(), false, None)?;
            }
            UniquesFunc::CollectionAttribute => {
                let (collection_id, key): (
                    <T as pallet_uniques::Config>::CollectionId,
                    BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>,
                ) = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::collection_attribute();
                env.charge_weight(base_weight)?;

                let attribute =
                    pallet_uniques::Pallet::<T>::collection_attribute(&collection_id, &key);
                env.write(&attribute.encode(), false, None)?;
            }
            UniquesFunc::CanTransfer => {
                let (collection_id, item): (
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                ) = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::can_transfer();
                env.charge_weight(base_weight)?;

                let can_transfer = pallet_uniques::Pallet::<T>::can_transfer(&collection_id, &item);
                env.write(&can_transfer.encode(), false, None)?;
            }

            UniquesFunc::Collections => {
                let read_bound: u32 = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::collections(read_bound);
                env.charge_weight(base_weight)?;

                let collections: Vec<<T as pallet_uniques::Config>::CollectionId> =
                    pallet_uniques::Pallet::<T>::collections().collect();

                env.write(&collections.encode(), false, None)?;
            }
            UniquesFunc::Items => {
                let (collection_id, read_bound): (
                    <T as pallet_uniques::Config>::CollectionId,
                    u32,
                ) = env.read_as()?;

                let base_weight = <W as weights::WeightInfo>::items(read_bound);
                env.charge_weight(base_weight)?;

                let items: Vec<<T as pallet_uniques::Config>::ItemId> =
                    pallet_uniques::Pallet::<T>::items(&collection_id).collect();

                env.write(&items.encode(), false, None)?;
            }
            UniquesFunc::Owned => {
                let (who, read_bound): (T::AccountId, u32) = env.read_as()?;

                let items: Vec<(
                    <T as pallet_uniques::Config>::CollectionId,
                    <T as pallet_uniques::Config>::ItemId,
                )> = pallet_uniques::Pallet::<T>::owned(&who).collect();

                let base_weight = <W as weights::WeightInfo>::owned(read_bound);
                env.charge_weight(base_weight)?;

                env.write(&items.encode(), false, None)?;
            }
            UniquesFunc::OwnedInCollection => {
                let (who, collection_id, read_bound): (
                    T::AccountId,
                    <T as pallet_uniques::Config>::CollectionId,
                    u32,
                ) = env.read_as()?;

                let items: Vec<<T as pallet_uniques::Config>::ItemId> =
                    pallet_uniques::Pallet::<T>::owned_in_collection(&collection_id, &who)
                        .collect();

                let base_weight = <W as weights::WeightInfo>::owned_in_collection(read_bound);
                env.charge_weight(base_weight)?;

                env.write(&items.encode(), false, None)?;
            }
        }

        Ok(RetVal::Converging(Outcome::Success as u32))
    }
}
