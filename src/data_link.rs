use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use yew::{html::ImplicitClone, prelude::*};

#[hook]
pub fn use_data<T, F>(init_fn: F) -> UseDataHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let update = use_force_update();
    let inner = use_mut_ref(|| UseDataHandleInner {
        value: init_fn(),
        update,
    });

    UseDataHandle(inner)
}

#[hook]
pub fn use_link<T>() -> UseLinkHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| None);

    UseLinkHandle(inner)
}

#[hook]
pub fn use_bind_link<T: 'static>(link: UseLinkHandle<T>, data: UseDataHandle<T>) {
    link.bind(data.clone());
    use_effect_with((), move |_| {
        move || {
            link.unbind();
        }
    });
}

pub trait MsgData {
    type Msg;

    fn msg(&mut self, msg: Self::Msg);
}

struct UseDataHandleInner<T> {
    value: T,
    update: UseForceUpdateHandle,
}

pub struct UseDataHandle<T>(Rc<RefCell<UseDataHandleInner<T>>>);

impl<T> Clone for UseDataHandle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: MsgData> UseDataHandle<T> {
    pub fn msg(&self, msg: <T as MsgData>::Msg) {
        self.0.borrow_mut().value.msg(msg);
        self.0.borrow().update.force_update();
    }
}

impl<T> UseDataHandle<T> {
    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.value)
    }
}

pub struct UseLinkHandle<T>(Rc<RefCell<Option<UseDataHandle<T>>>>);

impl<T> Clone for UseLinkHandle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> ImplicitClone for UseLinkHandle<T> {}

impl<T> PartialEq for UseLinkHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Default for UseLinkHandle<T> {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

impl<T: MsgData> UseLinkHandle<T> {
    pub fn try_msg(&self, msg: <T as MsgData>::Msg) -> Result<(), ()> {
        self.0.borrow().as_ref().map(|v| v.msg(msg)).ok_or(())
    }

    pub fn msg(&self, msg: <T as MsgData>::Msg) {
        self.try_msg(msg).unwrap()
    }
}

impl<T> UseLinkHandle<T> {
    pub fn is_binded(&self) -> bool {
        self.0.borrow().is_some()
    }

    fn bind(&self, data_handle: UseDataHandle<T>) {
        *self.0.borrow_mut() = Some(data_handle.clone());
    }

    fn unbind(&self) {
        *self.0.borrow_mut() = None;
    }
}
