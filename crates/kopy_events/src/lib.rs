#![no_std]
#![no_main]

pub mod events;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Notifier<E> {
    subscribers: Vec<Box<dyn Fn(&E) + Send>>,
}

impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier {
            subscribers: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F)
    where
        F: 'static + Fn(&E) + Send,
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&self, event: E) {
        for callback in &self.subscribers {
            callback(&event);
        }
    }
}
