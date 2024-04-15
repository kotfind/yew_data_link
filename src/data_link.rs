use std::{
    cell::RefCell,
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    ptr,
    rc::Rc,
};

use yew::{html::ImplicitClone, prelude::*};

#[hook]
pub fn use_data_link<T, F>(init_fn: F) -> UseLinkHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_mut_ref(|| UseDataHandleInner {
        value: init_fn(),
        listeners: HashSet::new(),
    });
    let link = use_link::<T>();

    let data = UseDataHandle {
        inner,
        parent_link: link.clone(),
        was_data_read: RefCell::new(false),
        was_data_wrote: RefCell::new(false),
    };

    *link.inner.deref().borrow_mut() = Some(data);

    link
}

#[hook]
pub fn use_link<T>() -> UseLinkHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| None);
    let update = Some(use_force_update());

    UseLinkHandle { inner, update }
}

struct UseDataHandleInner<T> {
    value: T,
    listeners: HashSet<UseLinkHandle<T>>,
}

pub struct UseDataHandle<T> {
    inner: Rc<RefCell<UseDataHandleInner<T>>>,
    parent_link: UseLinkHandle<T>,
    was_data_read: RefCell<bool>,
    was_data_wrote: RefCell<bool>,
}

impl<T> Drop for UseDataHandle<T> {
    fn drop(&mut self) {
        let mut borrow = self.inner.deref().borrow_mut();
        let listeners = &mut borrow.deref_mut().listeners;

        if *self.was_data_read.borrow() {
            listeners.insert(self.parent_link.clone());
        } else {
            // NOTE: breaks because of closures
            // listeners.remove(&self.parent_link);
        }

        if *self.was_data_wrote.borrow() {
            for listener in listeners.iter() {
                listener.update.as_ref().map(|u| u.force_update());
            }
        }
    }
}

// TODO: delete me
impl<T> PartialEq for UseDataHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> UseDataHandle<T> {
    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        *self.was_data_read.borrow_mut() = true;
        f(&self.inner.deref().borrow().deref().value)
    }

    pub fn apply<F, R>(&self, f: F)
    where
        F: FnOnce(&mut T) -> R,
    {
        *self.was_data_wrote.borrow_mut() = true;
        f(&mut self.inner.deref().borrow_mut().deref_mut().value);
    }

    pub fn apply_get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        *self.was_data_wrote.borrow_mut() = true;
        *self.was_data_read.borrow_mut() = true;
        f(&mut self.inner.deref().borrow_mut().deref_mut().value)
    }

    // NOTE: Do NOT implement actual Clone trait
    // for UseDataHandle as cloning into closure
    // would give problems with drop.
    fn inner_clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
            parent_link: self.parent_link.clone(),
            was_data_wrote: RefCell::new(false),
            was_data_read: RefCell::new(false),
        }
    }
}

impl<T: Clone> UseDataHandle<T> {
    pub fn get_cloned(&self) -> T {
        *self.was_data_read.borrow_mut() = true;
        self.inner.deref().borrow().deref().value.clone()
    }
}

pub struct UseLinkHandle<T> {
    inner: Rc<RefCell<Option<UseDataHandle<T>>>>,
    update: Option<UseForceUpdateHandle>, // update = None is returned by Default::default
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

impl<T> Hash for UseLinkHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(&*self.inner, state);
    }
}

impl<T> PartialEq for UseLinkHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Eq for UseLinkHandle<T> {}

impl<T> Default for UseLinkHandle<T> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
            update: None,
        }
    }
}

impl<T> UseLinkHandle<T> {
    pub fn get(&self) -> Option<UseDataHandle<T>> {
        self.inner
            .deref()
            .borrow()
            .deref()
            .as_ref()
            .map(|d| d.inner_clone())
    }

    pub fn bind(&self, link: &UseLinkHandle<T>) {
        // NOTE: following code panics on second render
        // if self.inner.deref().borrow().is_some() {
        //     panic!(
        //         "Cannot bind. \
        //         First link is already binded. \
        //         Maybe you are binding in the wrong order?"
        //     );
        // }
        let Some(mut data) = link
            .inner
            .deref()
            .borrow()
            .as_ref()
            .map(|d| d.inner_clone())
        else {
            panic!(
                "Cannot bind. \
                Second link doesn't point to any data. \
                Maybe you are binding in the wrong order?"
            );
        };
        data.parent_link = self.clone();
        *self.inner.deref().borrow_mut() = Some(data);
    }
}
