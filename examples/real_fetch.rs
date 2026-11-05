//! Tests models and actions by fetching all released content.

use msrec::{ClientFutureSend, action::*};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    for album in client.msr(GetAlbums()).await.unwrap() {
        let album = client.msr(GetAlbumData(album.cid)).await.unwrap();
        let album = client.msr(GetAlbumDetail(album.cid)).await.unwrap();
        println!("album {} \"{}\"", album.cid, album.name);
    }

    for song in client.msr(GetSongs()).await.unwrap().list {
        let song = client.msr(GetSong(song.cid)).await.unwrap();
        println!("song {} \"{}\"", song.cid, song.name);
    }
}
