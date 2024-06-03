
pub mod config;
pub(crate) mod config_ui;
#[macro_export]
macro_rules! impl_choice_populate {
    ($t:ident) => {
        #[allow(unused_imports)]
        use fltk::{prelude::*, *};
        impl GUIChoice for $t {
            fn populate_choice(control: &mut fltk::menu::Choice) {
                for x in $t::iter() {
                    control.add_choice(&format!("{:}", x));
                }
            }
            fn from_choice(control: &mut fltk::menu::Choice) -> $t {
                match control.choice() {
                    Some(v) => {
                        let x = v.to_string();
                        for e in $t::iter() {
                            if format!("{:}", e) == x {
                                return e;
                            }
                        }
                        panic!("Unknown choice \"{}\"", v);
                    },
                    None => $t::default()
                }
            }
            fn select_choice(&self, control: &mut fltk::menu::Choice) {
                let m = *self as i32;
                control.set_value(m);
            }
        }
    }
}
#[macro_export]
macro_rules! fltk_set_lazystatic {
    ($ui:ident, $control_name:ident, $cfg:ident, $field:ident, $resetfunc:ident) => {
        $ui.$control_name.set_callback(move |tb| {
            if let Ok(mut x) = $cfg.write() {
                x.$field = tb.value();
            }
            $resetfunc();
        })
    }
}
/// Set the value of a property that is inside a property like `Option<T>` on `$cfg`
///
/// An example where you have a control, where you want to set the value of a sub-property where the
/// property can be Optional. This is useful for setting the `RemoteNode.url` property.
/// ```rust
/// pub struct Data {
///     pub remote: Option<RemoteNode>
/// }
/// pub struct RemoteNode {
///     pub url: String, // Very useful for setting this!
///     pub token: Option<String>
/// }
/// ```
///
/// `remote` will be set to `Some` when `None`, and `RemoteNode.url` will be set to the value of
/// `$ui.$control.value()`.
#[macro_export]
macro_rules! fltk_set_lazystatic_option {
    ($ui:ident, $control:ident, $cfg_outer:ident, $cfg:ident, $cfg_type:ident, $field:ident, $resetfunc:ident) => {
        $ui.$control.set_callback(move |tb| {
            if let Ok(mut x) = $cfg.write() {
                let mut cfg = x.clone().$cfg_outer.unwrap_or($cfg_type::default());
                cfg.$field = tb.value();
                x.$cfg_outer = Some(cfg);
            }
            $resetfunc();
        })
    }
}
/// Set the value of a property that is like `Option<T>` that is inside a property like
/// `Option<T>` on `$cfg`
///
/// An example where you have a control, where if it has length it will set the value to Some,
/// otherwise it will be None when empty. This is useful for setting the `RemoteNode.token`
/// property.
/// ```rust
/// pub struct Data {
///     pub remote: Option<RemoteNode>
/// }
/// pub struct RemoteNode {
///     pub url: String,
///     pub token: Option<String>
/// }
/// ```
///
/// `remote` will be set to `Some` when `None` in the `$cfg` provided, and `RemoteNode.token` will be
/// set to `Some` when the length of `$url.$control.value()` is greater than zero.
#[macro_export]
macro_rules! fltk_set_lazystatic_option_withvalue {
    ($ui:ident, $control:ident, $cfg_outer:ident, $cfg:ident, $cfg_type:ident, $field:ident, $resetfunc:ident) => {
        $ui.$control.set_callback(move |tb| {
            if let Ok(mut x) = $cfg.write() {
                let mut cfg = x.clone().$cfg_outer.unwrap_or($cfg_type::default());
                cfg.$field = match tb.value().to_string().len() < 1 {
                    true => None,
                    false => Some(tb.value())
                };
                x.$cfg_outer = Some(cfg);
            }
            $resetfunc();
        })
    }
}
#[macro_export]
macro_rules! fltk_choice_set_lazystatic {
    ($ui:ident, $control:ident, $cfg:ident, $field:ident, $field_type:ident, $resetfunc:ident) => {
        $ui.$control.set_callback(move |cb| {
            if let Ok(mut x) = $cfg.write() {
                x.$field = $field_type::from_choice(cb);
            }
            $resetfunc();
        })
    }
}
#[macro_export]
macro_rules! fltk_choice_set_lazystatic_option {
    ($ui:ident, $control:ident, $cfg_outer:ident, $cfg:ident, $cfg_type:ident, $field:ident, $field_type:ident, $resetfunc:ident) => {
        $ui.$control.set_callback(move |cb| {
            if let Ok(mut x) = $cfg.write() {
                let mut cfg = x.clone().$cfg_outer.unwrap_or($cfg_type::default());
                cfg.$field = $field_type::from_choice(cb);
                x.$cfg_outer = Some(cfg);
            }
            $resetfunc();
        })
    }
}