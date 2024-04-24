//! # Yew Data Link
//! This crate allows one to mutate yew function component's data from other component
//! via sending messages.
//!
//! # Usage
//! To mutate state of component B from component A:
//! * Implement [`MsgData`] for your data type.
//! * Create link in component A via [`use_link`].
//! * Create data in component B via [`use_data`].
//! * Pass link from A to B and bind it via [`use_bind_link`].
//! * Mutate data with [`UseLinkHandle::msg`] and [`UseDataHandle::msg`].
//! * Read data with [`UseDataHandle::current`].
//!
//! # Example
//! ```rust
//! use yew::prelude::*;
//! use yew_autoprops::autoprops;
//! use yew_data_link::{use_bind_link, use_data, use_link, MsgData, UseLinkHandle};
//!
//! struct Num(i64);
//!
//! enum NumMsg {
//!     Inc,
//!     Dec,
//! }
//!
//! impl MsgData for Num {
//!     type Msg = NumMsg;
//!
//!     fn msg(&mut self, msg: NumMsg) {
//!         match msg {
//!             NumMsg::Inc => self.0 += 1,
//!             NumMsg::Dec => self.0 -= 1,
//!         };
//!     }
//! }
//!
//! #[autoprops]
//! #[function_component]
//! fn Counter(#[prop_or_default] link: &UseLinkHandle<Num>) -> Html {
//!     let num = use_data(|| Num(0));
//!     use_bind_link(link.clone(), num.clone());
//!
//!     html! {
//!         <div>{num.current().0}</div>
//!     }
//! }
//!
//! #[function_component]
//! fn App() -> Html {
//!     let link = use_link();
//!
//!     html! {
//!         <div>
//!             <button onclick={
//!                 let link = link.clone();
//!                 move |_| link.msg(NumMsg::Inc)
//!             }>{"+"}</button>
//!
//!             <Counter link={link.clone()} />
//!
//!             <button onclick={
//!                 let link = link.clone();
//!                 move |_| link.msg(NumMsg::Dec)
//!             }>{"-"}</button>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     yew::Renderer::<App>::new().render();
//! }
//! ```
//!
//! Check examples folder for more.

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use yew::{html::ImplicitClone, prelude::*};

#[hook]
/// A hook to manage component's data, that can be mutated from other components.
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
/// Create a link, that can be used to mutate other component's data.
pub fn use_link<T: MsgData>() -> UseLinkHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| UseLinkHandleInner {
        data_handle: None,
        msgs_on_bind: Vec::new(),
    });

    UseLinkHandle(inner)
}

#[hook]
/// Binds a link to a data.
///
/// Unbinds link, when current component is deleted.
pub fn use_bind_link<T: 'static + MsgData>(link: UseLinkHandle<T>, data: UseDataHandle<T>) {
    link.bind(data.clone());
    use_effect_with((), move |_| {
        move || {
            link.unbind();
        }
    });
}

/// A trait that is implemented on data to make it mutable by messages.
pub trait MsgData {
    type Msg;

    /// Applies message `msg` to `self`.
    fn msg(&mut self, msg: Self::Msg);
}

struct UseDataHandleInner<T> {
    value: T,
    update: UseForceUpdateHandle,
}

/// A handle for [`use_data`] hook.
pub struct UseDataHandle<T>(Rc<RefCell<UseDataHandleInner<T>>>);

impl<T> Clone for UseDataHandle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: MsgData> UseDataHandle<T> {
    /// Applies message `msg` to data.
    pub fn msg(&self, msg: <T as MsgData>::Msg) {
        self.0.borrow_mut().value.msg(msg);
        self.0.borrow().update.force_update();
    }
}

impl<T> UseDataHandle<T> {
    /// Returns reference to current data.
    pub fn current(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.value)
    }
}

struct UseLinkHandleInner<T: MsgData> {
    data_handle: Option<UseDataHandle<T>>,
    msgs_on_bind: Vec<<T as MsgData>::Msg>,
}

/// A handle for [`use_link`] hook.
pub struct UseLinkHandle<T: MsgData>(Rc<RefCell<UseLinkHandleInner<T>>>);

impl<T: MsgData> Clone for UseLinkHandle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: MsgData> ImplicitClone for UseLinkHandle<T> {}

impl<T: MsgData> PartialEq for UseLinkHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: MsgData> Default for UseLinkHandle<T> {
    /// Returns empty link.
    fn default() -> Self {
        Self(Rc::new(RefCell::new(UseLinkHandleInner {
            data_handle: None,
            msgs_on_bind: Vec::new(),
        })))
    }
}

impl<T: MsgData> UseLinkHandle<T> {
    /// Tries to apply message `msg` to data.
    ///
    /// Applies message and returns `Ok(())` if link is currently binded.
    /// Does nothing and returns `Err(())` otherwise.
    pub fn try_msg(&self, msg: <T as MsgData>::Msg) -> Result<(), ()> {
        self.0
            .borrow()
            .data_handle
            .as_ref()
            .map(|v| v.msg(msg))
            .ok_or(())
    }

    /// Applies message `msg` to data or panics.
    ///
    /// Applies message if link is currently binded, panics otherwise.
    pub fn msg(&self, msg: <T as MsgData>::Msg) {
        self.try_msg(msg).unwrap()
    }

    /// Applies message `msg` to data now or when link is binded.
    ///
    /// Applies message immediately if link is currently binded.
    /// Saves it and applies when link is binded otherwise.
    pub fn msg_on_bind(&self, msg: <T as MsgData>::Msg) {
        if self.is_binded() {
            self.msg(msg);
        } else {
            self.0.borrow_mut().msgs_on_bind.push(msg);
        }
    }

    /// Checks if link is currently binded.
    pub fn is_binded(&self) -> bool {
        self.0.borrow().data_handle.is_some()
    }

    fn bind(&self, data_handle: UseDataHandle<T>) {
        let msgs;
        {
            let mut inner = self.0.borrow_mut();
            inner.data_handle = Some(data_handle.clone());
            msgs = inner.msgs_on_bind.drain(..).collect::<Vec<_>>();

            // inner drops here
        }

        for msg in msgs {
            self.msg(msg);
        }
    }

    fn unbind(&self) {
        self.0.borrow_mut().data_handle = None;
    }
}
