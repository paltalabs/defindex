//! Definition of the Events used in the DeFindex Vault contract
use common::models::AssetStrategySet;
use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol, Val, Vec};

use crate::{models::AssetInvestmentAllocation, report::Report};

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
pub(crate) fn emit_rescue_event(
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
        .publish(("DeFindexVault", symbol_short!("rescue")), event);
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

// MANAGER QUEUE CHANGED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerQueuedEvent {
    pub new_manager_data: Vec<(u64, Address)>,
}

/// Publishes a `ManagerChangedEvent` to the event stream.
pub(crate) fn emit_queued_manager_event(e: &Env, new_manager_data: Vec<(u64, Address)>) {
    let event = ManagerQueuedEvent { new_manager_data };

    e.events()
        .publish(("DeFindexVault", symbol_short!("qmanager")), event);
}
// MANAGER QUEUE CLEAR EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerQueueClearEvent {
    pub timestamp: u64,
}

/// Publishes a `ManagerChangedEvent` to the event stream.
pub(crate) fn emit_clear_manager_queue_event(e: &Env, timestamp: u64) {
    let event = ManagerQueueClearEvent { timestamp };

    e.events()
        .publish(("DeFindexVault", symbol_short!("qmanager")), event);
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

// REBALANCE MANAGER CHANGED EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceManagerChangedEvent {
    pub new_rebalance_manager: Address,
}

/// Publishes a `RebalanceManagerChangedEvent` to the event stream.
pub(crate) fn emit_rebalance_manager_changed_event(e: &Env, new_rebalance_manager: Address) {
    let event = RebalanceManagerChangedEvent {
        new_rebalance_manager,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rbmanager")), event);
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
// EXECUTE INVESTMENT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecuteInvestmentEvent {
    pub assets: Vec<AssetStrategySet>,
    pub rebalance_method: Symbol,
    pub asset_investments: Vec<Option<AssetInvestmentAllocation>>,
}

/// Publishes an `ExecuteInvestmentEvent` to the event stream.
pub(crate) fn emit_execute_investment_event(
    e: &Env,
    assets: Vec<AssetStrategySet>,
    asset_investments: Vec<Option<AssetInvestmentAllocation>>,
) {
    let rebalance_method = symbol_short!("invest");
    let event = ExecuteInvestmentEvent {
        assets: assets,
        rebalance_method,
        asset_investments: asset_investments,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("invest")), event);
}


// REBALANCE WITHDRAW EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnwindEvent {
    pub call_params: Vec<(Address, i128, Address)>,
    pub rebalance_method: Symbol,
    pub report: Report,

}

pub(crate) fn emit_rebalance_unwind_event(
    e: &Env,
    call_params: Vec<(Address, i128, Address)>,
    report: Report,
) {
    let rebalance_method = symbol_short!("unwind");
    let event = UnwindEvent {
        call_params,
        rebalance_method,
        report,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rebalance")), event);
}

// REBALANCE INVEST EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvestEvent {
    pub asset_investments: Vec<AssetInvestmentAllocation>,
    pub rebalance_method: Symbol,
    pub report: Report,
}
pub(crate) fn emit_rebalance_invest_event(
    e: &Env, 
    asset_investments: Vec<AssetInvestmentAllocation>,
    report: Report,
) {
    let rebalance_method = symbol_short!("invest");
    let event = InvestEvent {
        asset_investments,
        rebalance_method,
        report,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rebalance")),event);
}

// SWAP EXACT IN EVENT

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

pub struct SwapExactInEvent {
    pub swap_args: Vec<Val>,
    pub rebalance_method: Symbol,
}
pub(crate) fn emit_rebalance_swap_exact_in_event(
    e: &Env, 
    swap_args: Vec<Val>,
) {
    let rebalance_method = symbol_short!("SwapEIn");
    let event: SwapExactInEvent = SwapExactInEvent {
        swap_args,
        rebalance_method,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rebalance")), event);
}

// SWAP EXACT OUT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapExactOutEvent {
    pub swap_args: Vec<Val>,
    pub rebalance_method: Symbol,
}
pub(crate) fn emit_rebalance_swap_exact_out_event(
    e: &Env, 
    swap_args: Vec<Val>,
) {
    let rebalance_method = symbol_short!("SwapEOut");
    let event = SwapExactOutEvent {
        swap_args,
        rebalance_method,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rebalance")), event);
}

// ZAPPER EVENT
//Waiting for zapper implementation
/* #[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZapperEvent {
    pub zap_args: Vec<Val>,
    pub rebalance_method: Symbol,
    pub report: Report,
}
pub(crate) fn emit_rebalance_zapper_event(
    e: &Env, 
    zap_args: Vec<Val>,
    report: Report,
) {
    let rebalance_method = symbol_short!("Zapper");
    let event = ZapperEvent {
        zap_args,
        rebalance_method,
        report,
    };

    e.events()
        .publish(("DeFindexVault", symbol_short!("rebalance")), event);
} */