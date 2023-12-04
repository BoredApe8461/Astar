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

use nfts_chain_extension_types::{select_origin, Origin, Outcome};
use frame_support::traits::{
    fungibles::approvals::Inspect as ApprovalInspect,
    fungibles::metadata::Inspect as MetadataInspect,
};
use frame_system::RawOrigin;
use pallet_uniques::WeightInfo;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use parity_scale_codec::Encode;
use sp_runtime::traits::StaticLookup;
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

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
            21 => Ok(NftsFunc::Collection),
            22 => Ok(NftsFunc::OwnershipAcceptance),
            23 => Ok(NftsFunc::Account),
            24 => Ok(NftsFunc::CollectionAccount),
            25 => Ok(NftsFunc::Item),
            26 => Ok(NftsFunc::CollectionMetadataOf),
            27 => Ok(NftsFunc::ItemMetadataOf),
            28 => Ok(NftsFunc::Attribute),
            29 => Ok(NftsFunc::ItemPriceOf),
            30 => Ok(NftsFunc::CollectionMaxSupply),
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
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
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
            },
            _ => todo!()
        }

        Ok(RetVal::Converging(Outcome::Success as u32))
    }
}
