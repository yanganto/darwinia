use darwinia_ring as ring;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::{impl_outer_origin, parameter_types};
// use pallet_balances::Balance;
use sp_core::H256;
use sp_io;
use sp_runtime::{
	testing::Header,
	traits::{
		BlakeTwo256,
		//ConvertInto,
		IdentityLookup,
	},
	Perbill,
};
use sp_timestamp as timestamp;

use crate::*;

/// The AccountId alias in this test module.
pub type AccountId = u64;
pub type BlockNumber = u64;
pub type Moment = u64;

pub type System = system::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;

#[cfg(feature = "transfer-fee")]
pub type Ring = ring::Module<Test>;
pub type Kton = Module<Test>;

type Balance = u128;

pub const NANO: Balance = 1;
pub const MICRO: Balance = 1_000 * NANO;
pub const MILLI: Balance = 1_000 * MICRO;
pub const COIN: Balance = 1_000 * MILLI;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
}

parameter_types! {
	pub const MinimumPeriod: Moment = 5;
}
impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
}

#[cfg(feature = "transfer-fee")]
parameter_types! {
	pub const TransferFee: Balance = 1 * MICRO;
}
#[cfg(not(feature = "transfer-fee"))]
parameter_types! {
	pub const TransferFee: Balance = 0;
}
impl ring::Trait for Test {
	type Balance = Balance;
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type TransferPayment = ();
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ();
	type TransferFee = TransferFee;
	type CreationFee = ();
}

impl Trait for Test {
	type Event = ();
}

pub struct ExtBuilder {
	balance_factor: Balance,
	vesting: bool,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balance_factor: COIN,
			vesting: false,
		}
	}
}

impl ExtBuilder {
	pub fn balance_factor(mut self, balance_factor: Balance) -> Self {
		self.balance_factor = balance_factor;
		self
	}
	pub fn vesting(mut self, vesting: bool) -> Self {
		self.vesting = vesting;
		self
	}
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		GenesisConfig::<Test> {
			balances: vec![
				(1, 10 * self.balance_factor),
				(2, 20 * self.balance_factor),
				(3, 300 * self.balance_factor),
				(4, 400 * self.balance_factor),
				(10, self.balance_factor),
				(11, 1000 * self.balance_factor),
				(20, self.balance_factor),
				(21, 2000 * self.balance_factor),
				(30, self.balance_factor),
				(31, 2000 * self.balance_factor),
				(40, self.balance_factor),
				(41, 2000 * self.balance_factor),
				(100, 2000 * self.balance_factor),
				(101, 2000 * self.balance_factor),
			],
			vesting: if self.vesting {
				vec![
					(1, 0, 10, 5 * self.balance_factor),
					(2, 10, 20, 0),
					(12, 10, 20, 5 * self.balance_factor),
				]
			} else {
				vec![]
			},
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}
}
