// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use super::Call;

#[test]
fn call_size() {
	assert!(
		core::mem::size_of::<Call>() <= 272,
		"size of Call is {:?} bytes which is more than 240 bytes: some calls have too big arguments, use Box to reduce the size of Call.
		If the limit is too strong, maybe consider increase the limit to 300.",
		core::mem::size_of::<Call>(),
	);
}