//! Implements support for the superfluid module.
use crate::{
    codec::Encoded,
    metadata::MetadataError,
    srml::{system::System, ModuleCalls},
    Valid, XtBuilder,
};
use runtime_primitives::traits::{Member, SimpleArithmetic};
use runtime_support::Parameter;
use substrate_primitives::Pair;

///
pub trait Superfluid: System {
    ///
    type Balance: Member + Parameter + SimpleArithmetic + Default + Copy;
    ///
    type AssetId: Parameter + SimpleArithmetic + Default + Copy;
}

///
pub trait SuperfluidXt {
    ///
    type Superfluid: Superfluid;
    ///
    type Pair: Pair;

    ///
    fn superfluid<F>(&self, f: F) -> XtBuilder<Self::Superfluid, Self::Pair, Valid>
    where
        F: FnOnce(
            ModuleCalls<Self::Superfluid, Self::Pair>,
        ) -> Result<Encoded, MetadataError>;
}

impl<T: Superfluid + 'static, P, V> SuperfluidXt for XtBuilder<T, P, V>
where
    P: Pair,
{
    type Superfluid = T;
    type Pair = P;

    fn superfluid<F>(&self, f: F) -> XtBuilder<T, P, Valid>
    where
        F: FnOnce(
            ModuleCalls<Self::Superfluid, Self::Pair>,
        ) -> Result<Encoded, MetadataError>,
    {
        self.set_call("Superfluid", f)
    }
}

impl<T: Superfluid + 'static, P> ModuleCalls<T, P>
where
    P: Pair,
{
    ///
    pub fn swap_assets_with_exact_output(
        self,
        output_account: T::AccountId,
        asset_input: T::AssetId,
        asset_output: T::AssetId,
        output_amount: T::Balance,
        max_input: T::Balance,
    ) -> Result<Encoded, MetadataError> {
        self.module.call(
            "swap_assets_with_exact_output",
            (
                output_account,
                asset_input,
                asset_output,
                output_amount,
                max_input,
            ),
        )
    }

    ///
    pub fn add_liquidity(
        self,
        asset_id: T::AssetId,
        inherent_asset_amount: T::Balance,
        asset_amount: T::Balance,
        min_liquidity: T::Balance,
    ) -> Result<Encoded, MetadataError> {
        self.module.call(
            "add_liquidity",
            (asset_id, inherent_asset_amount, asset_amount, min_liquidity),
        )
    }
}
