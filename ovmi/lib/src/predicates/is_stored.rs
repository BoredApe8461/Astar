use crate::executor::*;
use crate::predicates::*;

pub struct IsStoredPredicate<'a, Ext: ExternalCall> {
    pub ext: &'a Ext,
}

impl<Ext: ExternalCall> AtomicPredicateInterface<AddressOf<Ext>> for IsStoredPredicate<'_, Ext> {
    fn decide(&self, inputs: Vec<Vec<u8>>) -> ExecResult<AddressOf<Ext>> {
        require!(inputs.len() > 2);
        let address = Ext::bytes_to_address(&inputs[0])?;
        Ok(self
            .ext
            .ext_is_stored(&address, &inputs[1][..], &inputs[2][..]))
    }
}
impl<Ext: ExternalCall> AtomicHelperInterface<AddressOf<Ext>> for IsStoredPredicate<'_, Ext> {
    type Hash = HashOf<Ext>;
    fn ext_address(&self) -> AddressOf<Ext> {
        self.ext.ext_address()
    }
    fn ext_set_predicate_decision(
        &self,
        game_id: Self::Hash,
        decision: bool,
    ) -> ExecResult<AddressOf<Ext>> {
        self.ext.ext_set_predicate_decision(game_id, decision)
    }
    fn ext_get_property_id(&self, property: &Property<AddressOf<Ext>>) -> Self::Hash {
        self.ext.ext_get_property_id(property)
    }
}

impl<Ext: ExternalCall> DecidablePredicateInterface<AddressOf<Ext>> for IsStoredPredicate<'_, Ext> {
    fn decide_with_witness(
        &self,
        inputs: Vec<Vec<u8>>,
        _witness: Vec<Vec<u8>>,
    ) -> ExecResult<AddressOf<Ext>> {
        Self::decide(self, inputs)
    }
}

impl<Ext: ExternalCall> BaseAtomicPredicateInterface<AddressOf<Ext>>
    for IsStoredPredicate<'_, Ext>
{
}
