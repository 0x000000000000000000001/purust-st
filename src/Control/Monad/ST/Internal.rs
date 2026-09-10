use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

struct PurustSTRef(Mutex<crate::UnknownType>);

impl PurustSTRef {
    fn lock(&self) -> MutexGuard<'_, crate::UnknownType> {
        self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

fn purust_st_action(action: impl Fn() -> crate::UnknownType + 'static) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| action())))
}

pub fn Control_Monad_ST_Internal_run(action: crate::UnknownType) -> crate::UnknownType {
    action.unwrap_func1()(crate::Value::Unit)
}

pub fn Control_Monad_ST_Internal_pure_(value: crate::UnknownType) -> crate::UnknownType {
    purust_st_action(move || value.clone())
}

pub fn Control_Monad_ST_Internal_map_(
    map: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
    action: crate::UnknownType,
) -> crate::UnknownType {
    purust_st_action(move || map(Control_Monad_ST_Internal_run(action.clone())))
}

pub fn Control_Monad_ST_Internal_bind_(
    action: crate::UnknownType,
    next: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
) -> crate::UnknownType {
    purust_st_action(move || {
        let value = Control_Monad_ST_Internal_run(action.clone());
        Control_Monad_ST_Internal_run(next(value))
    })
}

pub fn Control_Monad_ST_Internal_while(
    condition: crate::UnknownType,
    body: crate::UnknownType,
) -> crate::UnknownType {
    purust_st_action(move || {
        while Control_Monad_ST_Internal_run(condition.clone()).unwrap_bool() {
            Control_Monad_ST_Internal_run(body.clone());
        }
        crate::Value::Unit
    })
}

pub fn Control_Monad_ST_Internal_for(
    lo: i64,
    hi: i64,
    body: purust_core::Func1<i64, crate::UnknownType>,
) -> crate::UnknownType {
    purust_st_action(move || {
        for index in lo..hi {
            Control_Monad_ST_Internal_run(body(index));
        }
        crate::Value::Unit
    })
}

pub fn Control_Monad_ST_Internal_foreach(
    values: crate::UnknownType,
    body: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
) -> crate::UnknownType {
    purust_st_action(move || {
        for value in values.unwrap_array().iter() {
            Control_Monad_ST_Internal_run(body(value.clone()));
        }
        crate::Value::Unit
    })
}

pub fn Control_Monad_ST_Internal_new(value: crate::UnknownType) -> crate::UnknownType {
    purust_st_action(move || crate::Value::Class(Rc::new(PurustSTRef(Mutex::new(value.clone())))))
}

pub fn Control_Monad_ST_Internal_read(reference: crate::UnknownType) -> crate::UnknownType {
    purust_st_action(move || reference.unwrap_class::<PurustSTRef>().lock().clone())
}

pub fn Control_Monad_ST_Internal_modifyImpl(
    update: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
    reference: crate::UnknownType,
) -> crate::UnknownType {
    purust_st_action(move || {
        let cell = reference.unwrap_class::<PurustSTRef>();
        // Read, pure callback and write form one modification, also in Arc mode.
        let mut guard = cell.lock();
        let result = update(guard.clone());
        let state = result.get_state();
        let value = result.get_value();
        *guard = state;
        value
    })
}

pub fn Control_Monad_ST_Internal_write(
    value: crate::UnknownType,
    reference: crate::UnknownType,
) -> crate::UnknownType {
    purust_st_action(move || {
        *reference.unwrap_class::<PurustSTRef>().lock() = value.clone();
        value.clone()
    })
}
