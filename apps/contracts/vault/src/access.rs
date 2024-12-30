use crate::error::ContractError;
use crate::utils::bump_instance;
use soroban_sdk::{contracttype, panic_with_error, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum RolesDataKey {
    EmergencyManager, // Role: Emergency Manager
    VaultFeeReceiver, // Role: Fee Receiver
    Manager,          // Role: Manager
    RebalanceManager, // Role: Rebalance Manager
}

#[derive(Clone)]
pub struct AccessControl(Env);

impl AccessControl {
    pub fn new(env: &Env) -> AccessControl {
        AccessControl(env.clone())
    }
}

pub trait AccessControlTrait {
    fn has_role(&self, key: &RolesDataKey) -> bool;
    fn get_role(&self, key: &RolesDataKey) -> Option<Address>;
    fn set_role(&self, key: &RolesDataKey, role: &Address);
    fn check_role(&self, key: &RolesDataKey) -> Result<Address, ContractError>;
    fn require_role(&self, key: &RolesDataKey);
    fn require_any_role(&self, keys: &[RolesDataKey], caller: &Address);
}

impl AccessControlTrait for AccessControl {
    fn has_role(&self, key: &RolesDataKey) -> bool {
        bump_instance(&self.0);
        self.0.storage().instance().has(key)
    }

    fn get_role(&self, key: &RolesDataKey) -> Option<Address> {
        bump_instance(&self.0);
        self.0.storage().instance().get(key)
    }

    fn set_role(&self, key: &RolesDataKey, role: &Address) {
        bump_instance(&self.0);
        self.0.storage().instance().set(key, role);
    }

    fn check_role(&self, key: &RolesDataKey) -> Result<Address, ContractError> {
        if !self.has_role(key) {
            panic_with_error!(&self.0, ContractError::RoleNotFound);
        }
        self.get_role(key).ok_or(ContractError::RoleNotFound)
    }

    fn require_role(&self, key: &RolesDataKey) {
        let role = match self.check_role(key) {
            Ok(v) => v,
            Err(err) => panic_with_error!(self.0, err),
        };

        role.require_auth();
    }

    fn require_any_role(&self, keys: &[RolesDataKey], caller: &Address) {
        let mut authorized = false;

        // Check if the caller has any of the provided roles
        for key in keys {
            if let Some(role_address) = self.get_role(key) {
                if role_address == *caller {
                    role_address.require_auth();
                    authorized = true;
                    break;
                }
            }
        }

        if !authorized {
            panic_with_error!(&self.0, ContractError::Unauthorized);
        }
    }
}

// Role-specific setters and getters
impl AccessControl {
    pub fn set_fee_receiver(&self, caller: &Address, vault_fee_receiver: &Address) {
        self.require_any_role(
            &[RolesDataKey::Manager, RolesDataKey::VaultFeeReceiver],
            caller,
        );
        self.set_role(&RolesDataKey::VaultFeeReceiver, vault_fee_receiver);
    }

    pub fn get_fee_receiver(&self) -> Result<Address, ContractError> {
        self.check_role(&RolesDataKey::VaultFeeReceiver)
    }

    pub fn set_manager(&self, manager: &Address) {
        self.require_role(&RolesDataKey::Manager);
        self.set_role(&RolesDataKey::Manager, manager);
    }

    pub fn get_manager(&self) -> Result<Address, ContractError> {
        self.check_role(&RolesDataKey::Manager)
    }

    pub fn set_emergency_manager(&self, emergency_manager: &Address) {
        self.require_role(&RolesDataKey::Manager);
        self.set_role(&RolesDataKey::EmergencyManager, emergency_manager);
    }

    pub fn get_emergency_manager(&self) -> Result<Address, ContractError> {
        self.check_role(&RolesDataKey::EmergencyManager)
    }

    pub fn set_rebalance_manager(&self, rebalance_manager: &Address) {
        self.require_role(&RolesDataKey::Manager);
        self.set_role(&RolesDataKey::RebalanceManager, rebalance_manager);
    }

    pub fn get_rebalance_manager(&self) -> Result<Address, ContractError> {
        self.check_role(&RolesDataKey::RebalanceManager)
    }
}
