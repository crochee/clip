use std::{marker::PhantomData, time::Duration};

use anyhow::{Context, Result};
use x11_clipboard::{Atom, Atoms, Clipboard as X11Clipboard};

use super::ClipboardProvider;

pub trait Selection {
    fn atom(atoms: &Atoms) -> Atom;
}

pub struct Primary;

impl Selection for Primary {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.primary
    }
}

pub struct Clipboard;

impl Selection for Clipboard {
    fn atom(atoms: &Atoms) -> Atom {
        atoms.clipboard
    }
}

pub struct X11ClipboardContext<S>(X11Clipboard, PhantomData<S>)
where
    S: Selection;

impl<S> ClipboardProvider for X11ClipboardContext<S>
where
    S: Selection,
{
    fn get_contents(&mut self) -> Result<String> {
        Ok(String::from_utf8(self.0.load(
            S::atom(&self.0.getter.atoms),
            self.0.getter.atoms.utf8_string,
            self.0.getter.atoms.property,
            Duration::from_secs(5),
        )?)?)
    }

    fn set_contents(&mut self, contents: String) -> Result<()> {
        Ok(self.0.store(
            S::atom(&self.0.setter.atoms),
            self.0.setter.atoms.utf8_string,
            contents,
        )?)
    }
}

impl X11ClipboardContext<Clipboard> {
    pub fn new() -> Result<Self> {
        Ok(X11ClipboardContext(
            X11Clipboard::new().context("Failed to create X11 clipboard")?,
            PhantomData,
        ))
    }
}

impl X11ClipboardContext<Primary> {
    pub fn new() -> Result<Self> {
        Ok(X11ClipboardContext(
            X11Clipboard::new().context("Failed to create primary clipboard")?,
            PhantomData,
        ))
    }
}
