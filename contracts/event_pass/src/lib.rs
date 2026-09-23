#![no_std]

use soroban_sdk::{
    contract,
    contractevent,
    contractimpl,
    contracttype,
    Address,
    Env,
};

// ----------------------------
// CLAVES PARA GUARDAR DATOS
// ----------------------------

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Purchased(Address),
    Used(Address),
}

// ----------------------------
// EVENTOS
// ----------------------------

// Se genera cuando una address compra su pase.
#[contractevent]
pub struct PassPurchased {
    #[topic]
    pub user: Address,
    pub status: u32,
}

// Se genera cuando una address usa su pase.
#[contractevent]
pub struct PassUsed {
    #[topic]
    pub user: Address,
    pub status: u32,
}

// ----------------------------
// CONTRATO
// ----------------------------

#[contract]
pub struct EventPass;

#[contractimpl]
impl EventPass {

    // Comprar un pase
    pub fn buy(env: Env, user: Address) -> bool {

        // La address debe autorizar la operación
        user.require_auth();

        let purchased_key = DataKey::Purchased(user.clone());

        let already_purchased: bool = env
            .storage()
            .persistent()
            .get(&purchased_key)
            .unwrap_or(false);

        // No puede comprar dos veces
        if already_purchased {
            panic!("El usuario ya compro un pase");
        }

        // Guardamos que compró el pase
        env.storage()
            .persistent()
            .set(&purchased_key, &true);

        // Publicamos evento
        PassPurchased {
            user,
            status: 1,
        }
        .publish(&env);

        true
    }

    // Usar el pase
    pub fn use_pass(env: Env, user: Address) -> bool {

        user.require_auth();

        let purchased_key = DataKey::Purchased(user.clone());
        let used_key = DataKey::Used(user.clone());

        let purchased: bool = env
            .storage()
            .persistent()
            .get(&purchased_key)
            .unwrap_or(false);

        // Debe haber comprado un pase primero
        if !purchased {
            panic!("El usuario no tiene un pase");
        }

        let already_used: bool = env
            .storage()
            .persistent()
            .get(&used_key)
            .unwrap_or(false);

        // Impedimos usarlo dos veces
        if already_used {
            panic!("El pase ya fue utilizado");
        }

        // Guardamos que el pase fue usado
        env.storage()
            .persistent()
            .set(&used_key, &true);

        // Publicamos evento
        PassUsed {
            user,
            status: 2,
        }
        .publish(&env);

        true
    }

    // Consultar estado del pase
    //
    // 0 = no tiene pase
    // 1 = compró el pase
    // 2 = ya utilizó el pase
    pub fn status(env: Env, user: Address) -> u32 {

        let purchased_key = DataKey::Purchased(user.clone());
        let used_key = DataKey::Used(user);

        let purchased: bool = env
            .storage()
            .persistent()
            .get(&purchased_key)
            .unwrap_or(false);

        let used: bool = env
            .storage()
            .persistent()
            .get(&used_key)
            .unwrap_or(false);

        if used {
            2
        } else if purchased {
            1
        } else {
            0
        }
    }
}