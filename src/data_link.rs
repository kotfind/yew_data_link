use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use yew::{html::ImplicitClone, prelude::*};

#[hook]
pub fn use_link<T>() -> UseLinkHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| None);

    UseLinkHandle(inner)
}

#[hook]
pub fn use_create_data<T, F>(init_fn: F) -> UseDataHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_mut_ref(|| UseDataHandleInner {
        value: init_fn(),
        listeners: HashMap::new(),
    });

    use_data_helper(Some(inner)).unwrap()
}

#[hook]
pub fn use_link_data<T>(link: UseLinkHandle<T>) -> Option<UseDataHandle<T>>
where
    T: 'static,
{
    use_data_helper(link.0.borrow().clone())
}

// Returns Some if inner was Some
#[hook]
fn use_data_helper<T>(inner: Option<Rc<RefCell<UseDataHandleInner<T>>>>) -> Option<UseDataHandle<T>>
where
    T: 'static,
{
    let update = use_force_update();

    static COMP_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let comp_id = *use_mut_ref(|| COMP_ID_COUNTER.fetch_add(1, Ordering::Relaxed)).borrow();

    inner.map(|inner| UseDataHandle {
        update,
        inner,
        comp_id,
    })
}

struct UseDataHandleInner<T> {
    value: T,
    listeners: HashMap<usize /*comp_id*/, UseForceUpdateHandle>,
}

pub struct UseDataHandle<T> {
    inner: Rc<RefCell<UseDataHandleInner<T>>>,
    update: UseForceUpdateHandle,
    comp_id: usize,
}

impl<T> Clone for UseDataHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
            update: self.update.clone(),
            comp_id: self.comp_id,
        }
    }
}

impl<T> UseDataHandle<T> {
    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.add_to_listeners();
        f(&self.inner.borrow().value)
    }

    pub fn apply<F, R>(&self, f: F)
    where
        F: FnOnce(&mut T) -> R,
    {
        self.update_listeners();
        f(&mut self.inner.borrow_mut().value);
    }

    pub fn apply_get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        self.update_listeners();
        self.add_to_listeners();
        f(&mut self.inner.borrow_mut().value)
    }

    pub fn link(&self) -> UseLinkHandle<T> {
        UseLinkHandle(Rc::new(RefCell::new(Some(Rc::clone(&self.inner)))))
    }

    fn update_listeners(&self) {
        for listener in self.inner.borrow().listeners.values() {
            listener.force_update();
        }
    }

    fn add_to_listeners(&self) {
        self.inner
            .borrow_mut()
            .listeners
            .insert(self.comp_id, self.update.clone());
    }
}

impl<T: Clone> UseDataHandle<T> {
    pub fn get_cloned(&self) -> T {
        self.add_to_listeners();
        self.inner.borrow().value.clone()
    }
}

pub struct UseLinkHandle<T>(Rc<RefCell<Option<Rc<RefCell<UseDataHandleInner<T>>>>>>);

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

impl<T> UseLinkHandle<T> {
    pub fn bind(&self, data: &UseDataHandle<T>) {
        *self.0.borrow_mut() = Some(Rc::clone(&data.inner));
    }
}
