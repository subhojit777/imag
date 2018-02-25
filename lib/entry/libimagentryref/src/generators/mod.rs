//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

//! Default generators
//!
//! This module provides a number of default `UniqueRefPathGenerator`s
//! which can be used for generating references.
//!
//! These generators are _NOT_ domain specific. So there won't be a "UniqueMailRefPathGenerator" in
//! here, for example.
//!
//! All these generators use "ref" as collection name.
//! They can be overridden using the `make_unique_ref_path_generator!()` convenience macro.
//!
//! # Note
//!
//! You must enable the appropriate crate feature to use any of the provided generators. With the
//! `generators` feature, you only get the convenience macro `make_unique_ref_path_generator!()`.
//!

/// A convenience macro for wrapping a generator in a new one, reusing the functionality from the
/// underlying generator
///
/// The UniqueRefPathGenerator must be in scope.
///
/// The macro creates a new struct `$name` over `$underlying` and changes the collection name to
/// `$collectionname`.
/// If passed, the new implementation is used (defaults to the implementation from the underlying
/// generator).
/// If passed, the new postprocessing is used (defaults to not changing the StoreId)
///
#[macro_export]
macro_rules! make_unique_ref_path_generator {
    (
        $name:ident
        over $underlying:ty
        => with error $errtype:ty
        => with collection name $collectionname:expr
    ) => {
        struct $name;

        impl $crate::refstore::UniqueRefPathGenerator for $name {
            type Error = $errtype;

            fn collection() -> &'static str {
                $collectionname
            }

            fn unique_hash<A: AsRef<Path>>(path: A) -> Result<String, Self::Error> {
                $underlying::unique_hash(path)
            }

            fn postprocess_storeid(sid: ::libimagstore::storeid::StoreId)
                -> Result<::libimagstore::storeid::StoreId, Self::Error>
            {
                Ok(sid)
            }
        }
    };

    (
        $name:ident
        over $underlying:ty
        => with error $errtype:ty
        => with collection name $collectionname:expr
        => $impl:expr
    ) => {
        struct $name;

        impl $crate::refstore::UniqueRefPathGenerator for $name {
            type Error = $errtype;

            fn collection() -> &'static str {
                $collectionname
            }

            fn unique_hash<A: AsRef<Path>>(path: A) -> Result<String, Self::Error> {
                debug!("Making unique hash for path: {:?}", path.as_ref());
                $impl(path)
            }

            fn postprocess_storeid(sid: ::libimagstore::storeid::StoreId)
                -> Result<::libimagstore::storeid::StoreId, Self::Error>
            {
                Ok(sid)
            }
        }
    };

    (
        pub $name:ident
        over $underlying:ty
        => with error $errtype:ty
        => with collection name $collectionname:expr
        => $impl:expr
    ) => {
        make_unique_ref_path_generator!(
            pub $name
            over $underlying
            => with error $errtype
            => with collection name $collectionname
            => $impl => |sid| { Ok(sid) }
            );
    };

    (
        pub $name:ident
        over $underlying:ty
        => with error $errtype:ty
        => with collection name $collectionname:expr
        => $impl:expr
        => $postproc:expr
    ) => {
        pub struct $name;

        impl $crate::refstore::UniqueRefPathGenerator for $name {
            type Error = $errtype;

            fn collection() -> &'static str {
                $collectionname
            }

            fn unique_hash<A: AsRef<Path>>(path: A) -> ::std::result::Result<String, Self::Error> {
                debug!("Making unique hash for path: {:?}", path.as_ref());
                $impl(path)
            }

            fn postprocess_storeid(sid: ::libimagstore::storeid::StoreId)
                -> ::std::result::Result<::libimagstore::storeid::StoreId, Self::Error>
            {
                $postproc(sid)
            }
        }
    };
}


#[cfg(any(
        feature = "generators-sha1",
        feature = "generators-sha224",
        feature = "generators-sha256",
        feature = "generators-sha384",
        feature = "generators-sha512",
        feature = "generators-sha3",
        ))]
mod base;

/// Helper macro for generating implementations for the various Sha algorithms
macro_rules! make_sha_mod {
    {
        $modname:ident,
        $hashname:ident,
        $hashingimpl:expr
    } => {
        pub mod $modname {
            use std::path::Path;
            use std::fs::OpenOptions;
            use std::io::Read;

            use error::RefError as RE;
            use error::RefErrorKind as REK;

            use crypto::digest::Digest;
            make_unique_ref_path_generator! (
                pub $hashname
                over generators::base::Base
                => with error RE
                => with collection name "ref"
                => |path| {
                    OpenOptions::new()
                        .read(true)
                        .write(false)
                        .create(false)
                        .open(path)
                        .map_err(RE::from)
                        .and_then(|mut file| {
                            let mut buffer = String::new();
                            let _ = file.read_to_string(&mut buffer)?;
                            $hashingimpl(buffer)
                        })
                }
            );

            impl $hashname {

                /// Function which can be used by a wrapping UniqueRefPathGenerator to hash only N bytes.
                pub fn hash_n_bytes<A: AsRef<Path>>(path: A, n: usize) -> Result<String, RE> {
                    debug!("Opening '{}' for hashing", path.as_ref().display());
                    OpenOptions::new()
                        .read(true)
                        .write(false)
                        .create(false)
                        .open(path)
                        .map_err(RE::from)
                        .and_then(|mut file| {
                            let mut buffer = vec![0; n];
                            debug!("Allocated {} bytes", buffer.capacity());

                            match file.read_exact(&mut buffer) {
                                Ok(_)  => { /* yay */ Ok(()) },
                                Err(e) => if e.kind() == ::std::io::ErrorKind::UnexpectedEof {
                                    debug!("Ignoring unexpected EOF before {} bytes were read", n);
                                    Ok(())
                                } else {
                                    Err(e)
                                }
                            }?;

                            let buffer = String::from_utf8(buffer)?;
                            $hashingimpl(buffer)
                        })
                }

                /// Hash the path, not the content behind the path
                pub fn hash_path<A: AsRef<Path>>(path: A) -> Result<String, RE> {
                    path.as_ref()
                        .to_str()
                        .map(String::from)
                        .ok_or_else(|| RE::from(REK::PathUTF8Error))
                        .and_then(|buffer| $hashingimpl(buffer))
                }

            }

        }
    }
}

#[cfg(feature = "generators-sha1")]
make_sha_mod! {
    sha1, Sha1, |buffer: String| {
        let mut hasher = ::crypto::sha1::Sha1::new();

        trace!("Hashing: '{:?}'", buffer);
        hasher.input_str(&buffer);
        let res = hasher.result_str();
        trace!("Hash => '{:?}'", res);

        Ok(String::from(res))
    }
}

#[cfg(feature = "generators-sha224")]
make_sha_mod! {
    sha224, Sha224, |buffer: String| {
        let mut hasher = ::crypto::sha2::Sha224::new();
        hasher.input_str(&buffer);
        Ok(String::from(hasher.result_str()))
    }
}

#[cfg(feature = "generators-sha256")]
make_sha_mod! {
    sha256, Sha256, |buffer: String| {
        let mut hasher = ::crypto::sha2::Sha256::new();
        hasher.input_str(&buffer);
        Ok(String::from(hasher.result_str()))
    }
}

#[cfg(feature = "generators-sha384")]
make_sha_mod! {
    sha384, Sha384, |buffer: String| {
        let mut hasher = ::crypto::sha2::Sha384::new();
        hasher.input_str(&buffer);
        Ok(String::from(hasher.result_str()))
    }
}

#[cfg(feature = "generators-sha512")]
make_sha_mod! {
    sha512, Sha512, |buffer: String| {
        let mut hasher = ::crypto::sha2::Sha512::new();
        hasher.input_str(&buffer);
        Ok(String::from(hasher.result_str()))
    }
}

#[cfg(feature = "generators-sha3")]
make_sha_mod! {
    sha3, Sha3, |buffer: String| {
        let mut hasher = ::crypto::sha3::Sha3::sha3_256();
        hasher.input_str(&buffer);
        Ok(String::from(hasher.result_str()))
    }
}

