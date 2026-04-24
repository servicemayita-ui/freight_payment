#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct FreightPayment;

#[contractimpl]
impl FreightPayment {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Create a new shipment. Only the shipper can call this.
    pub fn create_shipment(env: Env, shipment_id: u64, shipper: Address, carrier: Address, cost: u64) {
        shipper.require_auth();

        let mut shipments: Map<u64, (Address, Address, u64, bool, bool)> = env
            .storage()
            .instance()
            .get(&"shipments")
            .unwrap_or(Map::new(&env));

        if shipments.contains_key(shipment_id) {
            panic!("Shipment already exists");
        }

        shipments.set(shipment_id, (shipper, carrier, cost, false, false));
        env.storage().instance().set(&"shipments", &shipments);
    }

    /// Confirm delivery. Only the carrier can call this.
    pub fn confirm_delivery(env: Env, carrier: Address, shipment_id: u64) {
        carrier.require_auth();

        let mut shipments: Map<u64, (Address, Address, u64, bool, bool)> = env
            .storage()
            .instance()
            .get(&"shipments")
            .unwrap_or(Map::new(&env));

        let entry = shipments.get(shipment_id).unwrap_or_else(|| panic!("Shipment not found"));
        let (shipper, stored_carrier, cost, delivered, paid) = entry;

        if stored_carrier != carrier {
            panic!("Only the assigned carrier can confirm delivery");
        }

        if delivered {
            panic!("Delivery already confirmed");
        }

        if paid {
            panic!("Payment already released");
        }

        shipments.set(shipment_id, (shipper, stored_carrier, cost, true, false));
        env.storage().instance().set(&"shipments", &shipments);
    }

    /// Release payment. Only the shipper can call this after delivery is confirmed.
    pub fn release_payment(env: Env, shipper: Address, shipment_id: u64) {
        shipper.require_auth();

        let mut shipments: Map<u64, (Address, Address, u64, bool, bool)> = env
            .storage()
            .instance()
            .get(&"shipments")
            .unwrap_or(Map::new(&env));

        let entry = shipments.get(shipment_id).unwrap_or_else(|| panic!("Shipment not found"));
        let (stored_shipper, carrier, cost, delivered, paid) = entry;

        if stored_shipper != shipper {
            panic!("Only the shipper can release payment");
        }

        if !delivered {
            panic!("Delivery not confirmed yet");
        }

        if paid {
            panic!("Payment already released");
        }

        shipments.set(shipment_id, (stored_shipper, carrier, cost, true, true));
        env.storage().instance().set(&"shipments", &shipments);
    }

    /// Get shipment details. Returns (shipper, carrier, cost, delivered, paid)
    pub fn get_shipment(env: Env, shipment_id: u64) -> Option<(Address, Address, u64, bool, bool)> {
        let shipments: Map<u64, (Address, Address, u64, bool, bool)> = env
            .storage()
            .instance()
            .get(&"shipments")
            .unwrap_or(Map::new(&env));

        shipments.get(shipment_id)
    }
}
