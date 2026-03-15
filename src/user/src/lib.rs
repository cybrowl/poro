use candid::Principal;
use ic_cdk::api::msg_caller;
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableCell,
};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static GREETING: RefCell<StableCell<String, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            "Hello, ".to_string(),
        )
    );
}

#[init]
fn init() {
    ic_cdk::println!("🎉 Greeting canister initialized with stable memory!");
}

#[update]
fn set_greeting(prefix: String) {
    GREETING.with(|cell| {
        let _ = cell.borrow_mut().set(prefix);
    });
}

#[query]
fn greet(name: String) -> String {
    GREETING.with(|cell| format!("{}{}!", cell.borrow().get(), name))
}

#[query]
fn get_caller() -> Principal {
    msg_caller()
}

ic_cdk::export_candid!();
