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

use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{traits::StaticLookup, DispatchError};
use sp_std::marker::PhantomData;

#[cfg(not(feature = "local"))]
use cumulus_pallet_parachain_system::RelaychainDataProvider;
#[cfg(not(feature = "local"))]
use sp_runtime::traits::BlockNumberProvider;

type AccountIdLookup<T> = <<T as SysConfig>::Lookup as StaticLookup>::Source;

enum BlockNumberProviderFunc {
    RelayChainBlockNumber,
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Outcome {
    /// Success
    Success = 0,
    /// Origin Caller is not supported
    OriginCannotBeCaller = 98,
    /// Unknown error
    RuntimeError = 99,
}

impl TryFrom<u16> for BlockNumberProviderFunc {
    type Error = DispatchError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::RelayChainBlockNumber),
            _ => Err(DispatchError::Other(
                "Unimplemented func_id for BlockNumberProvider",
            )),
        }
    }
}

/// Block number provider chain extension.
pub struct BlockNumberProviderExtension<T, W>(PhantomData<(T, W)>);

impl<T, W> Default for BlockNumberProviderExtension<T, W> {
    fn default() -> Self {
        BlockNumberProviderExtension(PhantomData)
    }
}

#[cfg(feature = "local")]
impl<T, W> ChainExtension<T> for BlockNumberProviderExtension<T, W>
where
    T: pallet_contracts::Config,
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
            BlockNumberProviderFunc::RelayChainBlockNumber => {
                let base_weight = <W as weights::WeightInfo>::relay_chain_block_number();
                env.charge_weight(base_weight)?;

                let current_block_number = frame_system::Pallet::<T>::block_number();
                env.write(&current_block_number.encode(), false, None)?;
            }
        }

        Ok(RetVal::Converging(Outcome::Success as u32))
    }
}

#[cfg(not(feature = "local"))]
impl<T, W> ChainExtension<T> for BlockNumberProviderExtension<T, W>
where
    T: pallet_contracts::Config + cumulus_pallet_parachain_system::Config,
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
            BlockNumberProviderFunc::RelayChainBlockNumber => {
                let base_weight = <W as weights::WeightInfo>::relay_chain_block_number();
                env.charge_weight(base_weight)?;

                let current_block_number = RelaychainDataProvider::<T>::current_block_number();
                env.write(&current_block_number.encode(), false, None)?;
            }
        }

        Ok(RetVal::Converging(Outcome::Success as u32))
    }
}
