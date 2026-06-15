use alloc::{format, string::String};
use ini::Ini;

use crate::fs::*;

pub mod ini;

pub trait IniDeserialize: Sized {
    fn from_ini_section(ini: &Ini, base: &str) -> Self;

    /// Deserialize `Self` from the top level of `ini`.
    fn from_ini(ini: &Ini) -> Self {
        Self::from_ini_section(ini, "")
    }
}

pub trait IniSerialize {
    fn to_ini_section(&self, ini: &mut Ini, base: &str);

    /// Serialize `self` into an [`Ini`].
    fn to_ini(&self) -> Ini {
        let mut ini = Ini::default();
        self.to_ini_section(&mut ini, "");
        ini
    }
}

/// Loads the given config file if it exists, and creates it if not.
///
/// # Notes
/// This function stores and loads the config from the savedata directory as
/// that is unfortunately the only way the application can write anything to disk.
///
/// # Example
/// ```ignore
/// #[derive(Default, IniDeserialize, IniSerialize)]
/// struct MyConfig {
///     pub value: i32,
/// }
///
/// let config: MyConfig = forge::config::load("my_config.ini");
/// ```
pub fn load<T: IniDeserialize + IniSerialize + Default>(filename: &str) -> Result<T, String> {
    let _mount = savedata::ScopedMount::new();

    let path = format!("{}:/{}", savedata::mount_point(), filename);
    if file_exists(&path) {
        let ini = ini::Reader::read(&path)?;
        Ok(T::from_ini(&ini))
    } else {
        let config = T::default();
        ini::Writer::write(&path, &config.to_ini())?;
        Ok(config)
    }
}

/// Do not use
#[doc(hidden)]
pub mod __private {
    use core::{marker::PhantomData, str::FromStr};

    use alloc::{
        format,
        string::{String, ToString},
    };

    use super::{IniDeserialize, IniSerialize, ini::Ini};

    /// Join a parent section path with a segment using `.` as the separator,
    /// skipping the separator when either side is empty.
    pub fn join(base: &str, segment: &str) -> String {
        if base.is_empty() {
            String::from(segment)
        } else if segment.is_empty() {
            String::from(base)
        } else {
            format!("{base}.{segment}")
        }
    }

    /// Type-level probe used to pick the right read strategy for a field via
    /// autoref-based specialization. `Probe<T>` (no autoref) resolves to the
    /// nested-struct impl when `T: IniDeserialize`; otherwise `&Probe<T>`
    /// (one autoref) resolves to the scalar impl when `T: FromStr + Default`.
    pub struct Probe<T>(pub PhantomData<T>);

    /// Higher-priority impl: nested struct fields (`T: IniDeserialize`).
    pub trait IniNestedField {
        type Out;
        fn __ini_read(&self, ini: &Ini, section: &str, key: &str) -> Self::Out;
    }

    impl<T: IniDeserialize> IniNestedField for Probe<T> {
        type Out = T;

        fn __ini_read(&self, ini: &Ini, section: &str, key: &str) -> T {
            T::from_ini_section(ini, &join(section, key))
        }
    }

    /// Lower-priority impl: scalar fields (`T: FromStr + Default`).
    pub trait IniScalarField {
        type Out;
        fn __ini_read(&self, ini: &Ini, section: &str, key: &str) -> Self::Out;
    }

    impl<T: FromStr + Default> IniScalarField for &Probe<T> {
        type Out = T;

        fn __ini_read(&self, ini: &Ini, section: &str, key: &str) -> T {
            ini.get_as::<T>(section, key, T::default())
        }
    }

    /// Higher-priority impl: nested struct fields (`T: IniSerialize`).
    pub trait IniNestedWrite {
        type In;
        fn __ini_write(&self, value: &Self::In, ini: &mut Ini, section: &str, key: &str);
    }

    impl<T: IniSerialize> IniNestedWrite for Probe<T> {
        type In = T;

        fn __ini_write(&self, value: &T, ini: &mut Ini, section: &str, key: &str) {
            value.to_ini_section(ini, &join(section, key));
        }
    }

    /// Lower-priority impl: scalar fields (`T: ToString`).
    pub trait IniScalarWrite {
        type In;
        fn __ini_write(&self, value: &Self::In, ini: &mut Ini, section: &str, key: &str);
    }

    impl<T: ToString> IniScalarWrite for &Probe<T> {
        type In = T;

        fn __ini_write(&self, value: &T, ini: &mut Ini, section: &str, key: &str) {
            ini.set(section, key, value.to_string());
        }
    }
}
