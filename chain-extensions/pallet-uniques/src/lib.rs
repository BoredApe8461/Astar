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
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use parity_scale_codec::Encode;
use sp_runtime::traits::StaticLookup;
use sp_runtime::BoundedVec;
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

use uniques_chain_extension_types::{select_origin, Origin, Outcome};


type AccountIdLookup<T> = <<T as SysConfig>::Lookup as StaticLookup>::Source;

enum UniquesFunc {
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
            1 => Ok(UniquesFunc::Owner),
            2 => Ok(UniquesFunc::CollectionOwner),
            3 => Ok(UniquesFunc::Attribute),
            4 => Ok(UniquesFunc::CollectionAttribute),
            5 => Ok(UniquesFunc::CanTransfer),
            6 => Ok(UniquesFunc::Collections),
            7 => Ok(UniquesFunc::Items),
            8 => Ok(UniquesFunc::Owned),
            9 => Ok(UniquesFunc::OwnedInCollection),
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
