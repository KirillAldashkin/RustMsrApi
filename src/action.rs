//! Actions that are supported by the service API. 
//! These are passed to the client implementation.

use core::fmt;

use alloc::vec::Vec;

use crate::model::*;

/// Action that can be performed by the service.
/// These are passed to the implementations.
pub trait Action {
    /// Data returned by that action, not accounting for errors.
    #[cfg(not(feature = "serde"))]
    type Data;
    /// Data returned by that action, not accounting for errors.
    #[cfg(feature = "serde")]
    type Data: for<'a> serde::Deserialize<'a>;

    /// Formats URL path for the current action.
    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result;
}

/// Retrieve all albums.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetAlbums();

impl Action for GetAlbums {
    type Data = Vec<AlbumInfo>;
    
    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result {
        write!(fmt, "albums")
    }
}

/// Get basic info on a given album.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetAlbumData(pub Cid);

impl Action for GetAlbumData {
    type Data = AlbumData;

    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result {
        write!(fmt, "album/{}/data", self.0.0)
    }
}

/// Get extended info on a given album.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetAlbumDetail(pub Cid);

impl Action for GetAlbumDetail {
    type Data = AlbumDetail;

    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result {
        write!(fmt, "album/{}/detail", self.0.0)
    }
}

/// Retrieve all songs.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetSongs();

impl Action for GetSongs {
    type Data = SongsInfo;

    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result {
        write!(fmt, "songs")
    }
}

/// Get info on a given song.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GetSong(pub Cid);

impl Action for GetSong {
    type Data = SongData;

    fn path(&self, fmt: &mut impl fmt::Write) -> fmt::Result {
        write!(fmt, "song/{}", self.0.0)
    }
}
