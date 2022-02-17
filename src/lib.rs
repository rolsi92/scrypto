use scrypto::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[derive(Debug, sbor::Decode, sbor::Encode, sbor::Describe, sbor::TypeId)]
pub struct Event {
    id : u128,
    name : String,
    location : String,
    date : DateTime,
    price: Decimal
}

impl ToString  for Event {
    fn to_string(&self)  -> String
    { 
        return format!("{}|{}|{}|{}|{}", self.id, self.name, self.location, self.date, self.price);
    }
}

#[derive(NftData)]
pub struct EventTicketNftData {
    event_id : u128,
    #[scrypto(mutable)]
    has_been_used : bool
} 

blueprint! {
    struct EventTicket {
        // Define what resources and data will be managed by Hello components
        sample_vault: Vault,
        event_for_sale : HashMap<u128, Event>,
        admin_badge: Address,
        permanent_badge_vault : Vault,
        event_badge_def : ResourceDef,
        token_type : Address,
    }

    impl EventTicket {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(token_type : Address) -> Component {

            let admin_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
            .initial_supply_fungible(1);

            let permanent_minter_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
            .metadata("name", "event ticket minter permanent badge")
            .initial_supply_fungible(1);

            let event_badge_def = ResourceBuilder::new_non_fungible()
            .metadata("name", "event badge ")
            .metadata("description", "this badge give to buyer of event ticket. The buyer use this badge to access to event.")
            .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE | BURNABLE)
            .badge(permanent_minter_badge.resource_address(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA | MAY_BURN)
            .no_initial_supply(); 

            Self {
                sample_vault: Vault::new(token_type),
                event_for_sale: HashMap::new(),
                admin_badge: admin_badge.resource_address(),
                permanent_badge_vault: Vault::with_bucket(permanent_minter_badge),
                event_badge_def: event_badge_def,
                token_type: token_type
            }
            .instantiate()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.sample_vault.take(1)
        }
    }
}
