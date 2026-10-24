#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,Address,Env};
#[contracttype]#[derive(Clone)]pub enum Key{Owner,Value}
#[contract]pub struct Specimen;
#[contractimpl]impl Specimen{
 pub fn __constructor(e:Env,owner:Address){e.storage().instance().set(&Key::Owner,&owner);}
 pub fn read(e:Env)->u32{e.storage().persistent().get(&Key::Value).unwrap_or(0)}
 pub fn write(e:Env,value:u32){let owner:Address=e.storage().instance().get(&Key::Owner).unwrap();owner.require_auth();e.storage().persistent().set(&Key::Value,&value);}
}
#[cfg(test)]mod test{use super::*;use soroban_sdk::testutils::Address as _;
 #[test]fn missing_value_reads_zero(){let e=Env::default();let a=Address::generate(&e);let id=e.register(Specimen,(a,));assert_eq!(SpecimenClient::new(&e,&id).read(),0);}
 #[test]fn owner_write_is_visible(){let e=Env::default();e.mock_all_auths();let a=Address::generate(&e);let id=e.register(Specimen,(a,));let c=SpecimenClient::new(&e,&id);c.write(&17);assert_eq!(c.read(),17);}
 #[test]fn write_requires_owner_authorization(){let e=Env::default();let a=Address::generate(&e);let id=e.register(Specimen,(a,));assert!(SpecimenClient::new(&e,&id).try_write(&9).is_err());}
 #[test]fn read_does_not_change_value(){let e=Env::default();e.mock_all_auths();let a=Address::generate(&e);let id=e.register(Specimen,(a,));let c=SpecimenClient::new(&e,&id);c.write(&5);assert_eq!(c.read(),c.read());assert_eq!(c.read(),5);}
}
