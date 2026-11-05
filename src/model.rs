//! Various data types used by the service.

use core::{fmt, ops::{Deref, DerefMut}};

use alloc::{string::String, vec::Vec};
#[cfg(feature = "serde")]
use serde::Deserialize;
use url::Url;

/// Error returned by the service.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Error {
    /// Numeric type of this error.
    pub code: u32,
    /// Readable message describing the error.
    pub msg: String,
}

/// Otherwise being a transparent wrapper over [`Result<T, Error>`],
/// handles the way the service encodes errors.
#[repr(transparent)]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Response<T>(pub Result<T, Error>);

impl<T> Deref for Response<T> {
    type Target = Result<T, Error>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Response<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Result<T, Error>> for Response<T> {
    fn from(value: Result<T, Error>) -> Self {
        Self(value)
    }
}

impl<T> From<Response<T>> for Result<T, Error> {
    fn from(value: Response<T>) -> Self {
        value.0
    }
}

#[cfg(feature = "serde")]
mod response_deserialize;

/// ID used by all content.
#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Cid(pub String);

impl fmt::Display for Cid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Common info on an album, returned by [`GetAlbums`](super::action::GetAlbums).
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct AlbumInfo {
    /// ID of this album.
    pub cid: Cid,
    /// Release name of this album.
    pub name: String,
    /// URL of the smaller, square cover.
    #[cfg_attr(feature = "serde", serde(rename = "coverUrl"))]
    pub cover_url: Url,
    /// List of artists participated in this album.
    pub artistes: Vec<String>,
}

/// An album, returned by [`GetAlbumData`](super::action::GetAlbumData).
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct AlbumData {
    /// ID of this album.
    pub cid: Cid,
    /// Release name of this album.
    pub name: String,
    /// Description of this album.
    pub intro: String,
    /// Franchise this album belongs to.
    pub belong: String,
    /// URL of the smaller, square cover.
    #[cfg_attr(feature = "serde", serde(rename = "coverUrl"))]
    pub cover_url: Url,
    /// URL of the bigger cover.
    #[cfg_attr(feature = "serde", serde(rename = "coverDeUrl"))]
    pub cover_de_url: String,
    /// List of artists participated in this album.
    pub artistes: Vec<String>,
}

/// An album, returned by [`GetAlbumDetail`](super::action::GetAlbumDetail).
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct AlbumDetail {
    /// ID of this album.
    pub cid: Cid,
    /// Release name of this album.
    pub name: String,
    /// Description of this album.
    pub intro: String,
    /// Franchise this album belongs to.
    pub belong: String,
    /// URL of the smaller, square cover.
    #[cfg_attr(feature = "serde", serde(rename = "coverUrl"))]
    pub cover_url: Url,
    /// URL of the bigger cover.
    #[cfg_attr(feature = "serde", serde(rename = "coverDeUrl"))]
    pub cover_de_url: String,
    /// Songs included in this album.
    pub songs: Vec<Song>,
}

/// Song reference used in [`AlbumDetail`].
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Song {
    /// ID of this song.
    pub cid: Cid,
    /// Release name of this song.
    pub name: String,
    /// List of artists participated in this song.
    pub artistes: Vec<String>,
}

/// List of songs, returned by [`GetSongs`](super::action::GetSongs)
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct SongsInfo {
    /// All released songs.
    pub list: Vec<SongInfo>,
    /// ID of the song that is to be played automatically.
    pub autoplay: Cid,
}

/// Common info on a song, used in [`SongsInfo`].
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct SongInfo {
    /// ID of this song.
    pub cid: Cid,
    /// Release name of this song.
    pub name: String,
    /// ID of album this song belongs to.
    #[cfg_attr(feature = "serde", serde(rename = "albumCid"))]
    pub album_cid: Cid,
    /// List of artists participated in this song.
    pub artists: Vec<String>,
}

/// A song, returned by [`GetSong`](super::action::GetSong).
#[derive(Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct SongData {
    /// ID of this song.
    pub cid: Cid,
    /// Release name of this song.
    pub name: String,
    /// ID of album this song belongs to.
    #[cfg_attr(feature = "serde", serde(rename = "albumCid"))]
    pub album_cid: Cid,
    /// URL to the music file.
    #[cfg_attr(feature = "serde", serde(rename = "sourceUrl"))]
    pub source_url: Url,
    /// URL to the lyrics, if there is one.
    #[cfg_attr(feature = "serde", serde(rename = "lyricUrl"))]
    pub lyric_url: Option<Url>,

    // TODO are `mvUrl` and `mvCoverUrl` used?

    /// List of artists participated in this song.
    pub artists: Vec<String>,
}
