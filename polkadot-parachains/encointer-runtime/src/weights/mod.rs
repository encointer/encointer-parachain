// Copyright (c) 2019 Alain Brenzikofer
// This file is part of Encointer
//
// Encointer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Encointer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Encointer.  If not, see <http://www.gnu.org/licenses/>.

//! The weights used in the encointer-parachain-runtime

// the generated files do not pass clippy
#![allow(clippy::all)]

pub mod frame_system;
pub mod pallet_balances;
pub mod pallet_collective;
pub mod pallet_encointer_balances;
pub mod pallet_encointer_bazaar;
pub mod pallet_encointer_ceremonies;
pub mod pallet_encointer_communities;
pub mod pallet_encointer_scheduler;
pub mod pallet_membership;
pub mod pallet_timestamp;
pub mod pallet_treasury;
pub mod pallet_utility;
