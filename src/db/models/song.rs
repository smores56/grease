use diesel;
use db::pool::DB;
use db::models::*;
use diesel::prelude::*;
use db::schema::songs::dsl::*;

impl Song {
    pub const KEYS: [&'static str; 21] = ["A♭", "A",  "A♯", "B♭", "B",  "B♯", "C♭",
                                          "C",  "C♯", "D♭", "D",  "D♯", "E♭", "E",
                                          "E♯", "F♭", "F",  "F♯", "G♭", "G",  "G♯"];

    pub fn load(given_song_id: i32, conn: &DB) -> Result<Song, String> {
        songs
            .filter(id.eq(given_song_id))
            .first::<Song>(&**conn)
            .optional()
            .expect("error loading song")
            .ok_or(format!("no song exists with the id {}", given_song_id))
    }

    pub fn load_with_data(given_song_id: i32, conn: &DB) -> Result<SongData, String> {
        let song = Song::load(given_song_id, conn)?;
        let (sheets, midis) = File::load_for_song_sorted(given_song_id, conn);
        let (performance_links, other_links) = Link::load_for_song_sorted(given_song_id, conn);
        Ok(SongData { song, sheets, midis, performance_links, other_links })
    }

    pub fn load_all(conn: &DB) -> Vec<Song> {
        songs
            .order(name)
            .load::<Song>(&**conn)
            .expect("error loading songs")
    }

    pub fn load_all_separate_this_semester(conn: &DB) -> (Vec<Song>, Vec<Song>) {
        let mut current_songs = Vec::new();
        let mut other_songs = Vec::new();
        for song in Song::load_all(conn) {
            if song.this_semester {
                current_songs.push(song);
            } else {
                other_songs.push(song);
            }
        }

        (current_songs, other_songs)
    }

    // TODO: figure out what to do with actual file uploading / creation
    pub fn create(new_song: NewSong, files: Vec<NewFile>, conn: &DB) {
        diesel::insert(&new_song)
            .into(songs)
            .execute(&**conn)
            .expect("error adding new attendances");
        File::create_multiple(files, conn);
    }

    pub fn update(given_song_id: i32, updated_song: NewSong, conn: &DB) -> bool {
        diesel::update(songs.find(given_song_id))
            .set(&updated_song)
            .get_result::<Song>(&**conn)
            .is_ok()
    }
}

impl PublicJson for Song {}
