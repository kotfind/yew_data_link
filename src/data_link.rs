use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html::ImplicitClone, prelude::*};

#[hook]
pub fn use_data<T, F>(init_fn: F) -> UseDataHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_mut_ref(|| UseDataHandleInner {
        value: init_fn(),
        links: Vec::new(),
    });
    let update = use_force_update();

    UseDataHandle { inner, update }
}

#[hook]
pub fn use_link<T>() -> UseLinkHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| None);
    let update = use_force_update();

    UseLinkHandle {
        inner,
        update: Some(update),
    }
}

#[hook]
pub fn use_bind_link<T: 'static>(link: UseLinkHandle<T>, data: UseDataHandle<T>) {
    use_effect_with((), move |_| {
        link.bind(data.clone());
        move || {
            link.unbind(data);
        }
    });
}

struct UseDataHandleInner<T> {
    value: T,
    links: Vec<UseLinkHandle<T>>,
}

pub struct UseDataHandle<T> {
    inner: Rc<RefCell<UseDataHandleInner<T>>>,
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
        f(&self.inner.deref().borrow().deref().value)
    }

    pub fn apply_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let ret = f(&mut self.inner.deref().borrow_mut().deref_mut().value);

        self.update.force_update();
        for link in &self.inner.deref().borrow().links {
            let Some(ref update) = link.update else {
                panic!("dummy UseLinkHandles should not be added to UseDataHandle::links");
            };
            update.force_update();
        }

        ret
    }
}

impl<T: Clone> UseDataHandle<T> {
    pub fn get_cloned(&self) -> T {
        self.inner.deref().borrow().deref().value.clone()
    }
}

// UseLinkHandles may be dummy.
// Dummy link still hold a refference to data, though
// update is not triggered, when data changes.
// Dummy links are used to create optional properties (#[prop_or_default])
pub struct UseLinkHandle<T> {
    inner: Rc<RefCell<Option<UseDataHandle<T>>>>,
    update: Option<UseForceUpdateHandle>, // is None for dummy links
}

impl<T> Clone for UseLinkHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
            update: self.update.clone(),
        }
    }
}

impl<T> ImplicitClone for UseLinkHandle<T> {}

impl<T> PartialEq for UseLinkHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Default for UseLinkHandle<T> {
    fn default() -> Self {
        UseLinkHandle::<T>::dummy()
    }
}

impl<T> UseLinkHandle<T> {
    pub fn dummy() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
            update: None,
        }
    }

    pub fn is_dummy(&self) -> bool {
        self.update.is_none()
    }

    pub fn get(&self) -> Option<UseDataHandle<T>> {
        self.inner.deref().borrow().deref().clone()
    }

    fn bind(&self, data_handle: UseDataHandle<T>) {
        *self.inner.deref().borrow_mut() = Some(data_handle.clone());
        if !self.is_dummy() {
            data_handle
                .inner
                .deref()
                .borrow_mut()
                .links
                .push(self.clone());
        }
    }

    fn unbind(&self, data_handle: UseDataHandle<T>) {
        *self.inner.deref().borrow_mut() = None;
        if !self.is_dummy() {
            data_handle
                .inner
                .deref()
                .borrow_mut()
                .links
                .retain(|link| link != self);
        }
    }
}
