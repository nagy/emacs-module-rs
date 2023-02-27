use crate::{symbol, Env, Value, Result};

pub use {user_ptr::Transfer, vector::Vector};

mod integer;
mod float;
mod string;

mod user_ptr;
mod vector;

// XXX: More accurate would be `CloneFromLisp` or `Decode`, but ...
/// Converting Lisp [`Value`] into a Rust type.
///
/// # Implementation
///
/// The lifetime parameter is put on the trait itself, instead of the method. This allows it to be
/// implemented for [`Value`] itself.
///
/// [`Value`]: struct.Value.html
pub trait FromLisp<'e>: Sized {
    fn from_lisp(value: Value<'e>) -> Result<Self>;
}

// XXX: More accurate would be `CloneToLisp`, `Encode`, but ...
/// Converting a Rust type into Lisp [`Value`].
///
/// # Implementation
///
/// The lifetime parameter is put on the trait itself, instead of the method. This allows the impl
/// for [`Value`] to simply return the input, instead of having to create a new [`Value`].
///
/// [`Value`]: struct.Value.html
pub trait IntoLisp<'e> {
    fn into_lisp(self, env: &'e Env) -> Result<Value<'e>>;
}

impl<'e> FromLisp<'e> for Value<'e> {
    #[inline(always)]
    fn from_lisp(value: Value<'e>) -> Result<Value<'_>> {
        Ok(value)
    }
}

impl<'e> IntoLisp<'e> for Value<'e> {
    #[inline(always)]
    fn into_lisp(self, _: &'e Env) -> Result<Value<'_>> {
        Ok(self)
    }
}

impl<'e, T: FromLisp<'e>> FromLisp<'e> for Option<T> {
    fn from_lisp(value: Value<'e>) -> Result<Self> {
        if value.is_not_nil() {
            Ok(Some(<T as FromLisp>::from_lisp(value)?))
        } else {
            Ok(None)
        }
    }
}

impl<'e, T: IntoLisp<'e>> IntoLisp<'e> for Option<T> {
    fn into_lisp(self, env: &'e Env) -> Result<Value<'_>> {
        match self {
            Some(t) => t.into_lisp(env),
            None => symbol::nil.into_lisp(env),
        }
    }
}

impl IntoLisp<'_> for () {
    fn into_lisp(self, env: &Env) -> Result<Value<'_>> {
        symbol::nil.into_lisp(env)
    }
}

impl IntoLisp<'_> for bool {
    fn into_lisp(self, env: &Env) -> Result<Value<'_>> {
        if self {
            symbol::t.into_lisp(env)
        } else {
            symbol::nil.into_lisp(env)
        }
    }
}

impl<'e, T: IntoLisp<'e> + Copy> IntoLisp<'e> for Vec<T> {
    fn into_lisp(self, env: &'e Env) -> Result<Value<'e>> {
        let w = env.make_vector(self.len(), symbol::nil)?;
        for i in 0..self.len() {
            w.set(i, self[i])?
        }
        Ok(w.value())
    }
}
