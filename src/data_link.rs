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
pub fn use_bind_link<T: 'static + MsgData>(link: UseLinkHandle<T>, data: UseDataHandle<T>) {
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

struct UseLinkHandleInner<T: MsgData> {
    data_handle: Option<UseDataHandle<T>>,
    msgs_on_bind: Vec<<T as MsgData>::Msg>,
}

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
    fn default() -> Self {
        Self(Rc::new(RefCell::new(UseLinkHandleInner {
            data_handle: None,
            msgs_on_bind: Vec::new(),
        })))
    }
}

impl<T: MsgData> UseLinkHandle<T> {
    pub fn try_msg(&self, msg: <T as MsgData>::Msg) -> Result<(), ()> {
        self.0
            .borrow()
            .data_handle
            .as_ref()
            .map(|v| v.msg(msg))
            .ok_or(())
    }

    pub fn msg(&self, msg: <T as MsgData>::Msg) {
        self.try_msg(msg).unwrap()
    }

    pub fn msg_on_bind(&self, msg: <T as MsgData>::Msg) {
        if self.is_binded() {
            self.msg(msg);
        } else {
            self.0.borrow_mut().msgs_on_bind.push(msg);
        }
    }

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
