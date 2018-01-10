use diesel;
use db::pool::DB;
use db::models::*;
use diesel::prelude::*;
use db::schema::files::dsl::*;

impl File {
    pub fn load(given_file_id: i32, conn: &DB) -> Result<File, String> {
        files
            .filter(id.eq(given_file_id))
            .first::<File>(&**conn)
            .optional()
            .expect("error loading file")
            .ok_or(format!("no file exists with the id {}", given_file_id))
    }

    pub fn load_for_path(given_file_path: &str, conn: &DB) -> Result<File, String> {
        files
            .filter(path.eq(given_file_path))
            .first::<File>(&**conn)
            .optional()
            .expect("error loading file")
            .ok_or(format!("no file exists with the path {}", given_file_path))
    }

    pub fn load_for_song(given_song_id: i32, conn: &DB) -> Vec<File> {
        files
            .filter(song_id.eq(given_song_id))
            .order(name)
            .load::<File>(&**conn)
            .expect("error loading files")
    }

    pub fn load_for_song_sorted(given_song_id: i32, conn: &DB) -> (Vec<File>, Vec<File>) {
        let mut sheets = Vec::new();
        let mut midis = Vec::new();
        for file in File::load_for_song(given_song_id, conn) {
            if file.is_sheet {
                sheets.push(file);
            } else {
                midis.push(file);
            }
        }

        (sheets, midis)
    }

    // TODO: figure out what to do with actual file uploading / creation
    pub fn create(new_file: NewFile, conn: &DB) {
        diesel::insert(&new_file)
            .into(files)
            .execute(&**conn)
            .expect("error adding new file");
    }

    pub fn create_multiple(new_files: Vec<NewFile>, conn: &DB) {
        diesel::insert(&new_files)
            .into(files)
            .execute(&**conn)
            .expect("error adding new files");
    }

    pub fn update(given_file_id: i32, updated_file: NewFile, conn: &DB) -> bool {
        diesel::update(files.find(given_file_id))
            .set(&updated_file)
            .get_result::<File>(&**conn)
            .is_ok()
    }
}

impl PublicJson for File {}
