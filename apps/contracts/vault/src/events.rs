//! Definition of the Events used in the DeFindex Vault contract
use soroban_sdk::{contracttype, symbol_short, Address, Env, Vec};

// DEPOSIT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultDepositEvent {
    pub depositor: Address,
    pub amounts: Vec<i128>,
    pub df_tokens_minted: i128,
}

/// Publishes a `VaultDepositEvent` to the event stream.
pub(crate) fn emit_deposit_event(
    e: &Env,
    depositor: Address,
    amounts: Vec<i128>,
    df_tokens_minted: i128,
) {
    let event = VaultDepositEvent {
        depositor,
        amounts,
        df_tokens_minted,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("deposit")), event);
}

// WITHDRAW EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultWithdrawEvent {
    pub withdrawer: Address,
    pub df_tokens_burned: i128,
    pub amounts_withdrawn: Vec<i128>,
}

/// Publishes a `VaultWithdrawEvent` to the event stream.
pub(crate) fn emit_withdraw_event(
    e: &Env,
    withdrawer: Address,
    df_tokens_burned: i128,
    amounts_withdrawn: Vec<i128>,
) {
    let event = VaultWithdrawEvent {
        withdrawer,
        df_tokens_burned,
        amounts_withdrawn,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("withdraw")), event);
}

// EMERGENCY WITHDRAW EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmergencyWithdrawEvent {
    pub caller: Address,
    pub strategy_address: Address,
    pub amount_withdrawn: i128,
}

/// Publishes an `EmergencyWithdrawEvent` to the event stream.
pub(crate) fn emit_emergency_withdraw_event(
    e: &Env,
    caller: Address,
    strategy_address: Address,
    amount_withdrawn: i128,
) {
    let event = EmergencyWithdrawEvent {
        caller,
        strategy_address,
        amount_withdrawn,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("ewithdraw")), event);
}

// STRATEGY PAUSED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyPausedEvent {
    pub strategy_address: Address,
    pub caller: Address,
}

/// Publishes a `StrategyPausedEvent` to the event stream.
pub(crate) fn emit_strategy_paused_event(e: &Env, strategy_address: Address, caller: Address) {
    let event = StrategyPausedEvent {
        strategy_address,
        caller,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("paused")), event);
}

// STRATEGY UNPAUSED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyUnpausedEvent {
    pub strategy_address: Address,
    pub caller: Address,
}

/// Publishes a `StrategyUnpausedEvent` to the event stream.
pub(crate) fn emit_strategy_unpaused_event(e: &Env, strategy_address: Address, caller: Address) {
    let event = StrategyUnpausedEvent {
        strategy_address,
        caller,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("unpaused")), event);
}

// FEE RECEIVER CHANGED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeReceiverChangedEvent {
    pub new_fee_receiver: Address,
    pub caller: Address,
}

/// Publishes a `FeeReceiverChangedEvent` to the event stream.
pub(crate) fn emit_fee_receiver_changed_event(e: &Env, new_fee_receiver: Address, caller: Address) {
    let event = FeeReceiverChangedEvent {
        new_fee_receiver,
        caller,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("nreceiver")), event);
}

// MANAGER CHANGED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerChangedEvent {
    pub new_manager: Address,
}

/// Publishes a `ManagerChangedEvent` to the event stream.
pub(crate) fn emit_manager_changed_event(e: &Env, new_manager: Address) {
    let event = ManagerChangedEvent { new_manager };

    e.events()
        .publish(("DeFindexVault", symbol_short!("nmanager")), event);
}

// EMERGENCY MANAGER CHANGED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmergencyManagerChangedEvent {
    pub new_emergency_manager: Address,
}

/// Publishes an `EmergencyManagerChangedEvent` to the event stream.
pub(crate) fn emit_emergency_manager_changed_event(e: &Env, new_emergency_manager: Address) {
    let event = EmergencyManagerChangedEvent {
        new_emergency_manager,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("nemanager")), event);
}

// FEES DISTRIBUTED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeesDistributedEvent {
    pub distributed_fees: Vec<(Address, i128)>,
}

/// Publishes an `EmergencyManagerChangedEvent` to the event stream.
pub(crate) fn emit_fees_distributed_event(e: &Env, distributed_fees: Vec<(Address, i128)>) {
    let event = FeesDistributedEvent { distributed_fees };

    e.events()
        .publish(("DeFindexVault", symbol_short!("dfees")), event);
}
