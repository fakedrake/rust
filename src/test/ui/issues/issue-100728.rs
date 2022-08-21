// run-pass
#![feature(contravariant_traits)]
type F1<'arg> = Box<dyn FnOnce(&'arg u32)>;
fn contravar_once<'small, 'large: 'small>(f: F1<'small>) -> F1<'large> {
    f
}

type F2<'arg> = Box<dyn Fn(&'arg u16)>;
fn contravar<'small, 'large: 'small>(f: F2<'small>) -> F2<'large> {
    f
}

type F3<'arg> = Box<dyn FnMut(&'arg u8)>;
fn contravar_mut<'small, 'large: 'small>(f: F3<'small>) -> F3<'large> {
    f
}

fn main() {}
