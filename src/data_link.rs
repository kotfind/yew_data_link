use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::prelude::*;

#[hook]
pub fn use_data<T, F>(init_fn: F) -> UseDataHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_mut_ref(init_fn);
    let update = use_force_update();

    UseDataHandle { inner, update }
}

#[hook]
pub fn use_link<T>() -> UseStateHandle<DataLink<T>>
where
    T: 'static,
{
    use_state(|| DataLink::new())
}

#[hook]
pub fn use_bind_link<T>(link: DataLink<T>, data: UseDataHandle<T>) {
    link.set(data);
}

pub struct UseDataHandle<T> {
    inner: Rc<RefCell<T>>,
    update: UseForceUpdateHandle,
}

impl<T> Clone for UseDataHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
            update: self.update.clone(),
        }
    }
}

impl<T> UseDataHandle<T> {
    pub fn apply<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(self.inner.deref().borrow().deref())
    }

    pub fn apply_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let ret = f(self.inner.deref().borrow_mut().deref_mut());
        self.update.force_update();
        ret
    }
}

impl<T: Clone> UseDataHandle<T> {
    pub fn get_cloned(&self) -> T {
        self.inner.deref().borrow().deref().clone()
    }
}

pub struct DataLink<T>(Rc<RefCell<Option<UseDataHandle<T>>>>);

impl<T> Clone for DataLink<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Default for DataLink<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PartialEq for DataLink<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> DataLink<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }

    pub fn get(&self) -> Option<UseDataHandle<T>> {
        self.0.deref().borrow().deref().clone()
    }

    fn set(&self, h: UseDataHandle<T>) {
        *self.0.deref().borrow_mut() = Some(h);
    }
}
