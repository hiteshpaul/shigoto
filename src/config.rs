use std::fs::{ OpenOptions};
use std::fs;
use std::io::Error;
use std::env;
use std::path::PathBuf;
use serde;
use serde_json;
use chrono::prelude::*;


pub trait Show {
    fn show(&self) -> Result<(), Error>;
}
impl Show for UserData {
    fn show(&self) -> Result<(), Error> {
        for task in self.tasks.iter() {
            println!("{id}: {name}, {priority}",
                     id=task.id,
                     name=task.name,
                     priority=task.priority);
        }
    Ok(())
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub priority: i8,
    pub created_on: DateTime<Utc>, 
    pub due: DateTime<Utc>,
    pub name: String,
    pub is_completed: bool,
    pub tags: Vec<String>,
}


impl Task {
    pub fn with_default(name: &str) -> Task {
        Task {
            id: 1,
            priority: 2,
            created_on: Utc::now(),
            due: Utc::now(),
            name: name.to_string(),
            is_completed: false,
            tags: {
                let mut v = Vec::new();
                v.push(String::from("hello"));
                v
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub tasks: Vec<Task>,
}


impl UserData {
    pub fn new() -> UserData {
        let tasks = Vec::new();
        UserData {
            tasks
        }
    }
}

pub struct Config {
    pub data_file: PathBuf,
    pub user_data: UserData,
}

impl Config {
    pub fn new() -> serde::export::Result<Config, Box<Error>> {
        let data_path: PathBuf = env::var("XDG_DATA_HOME")
            .map(|p| PathBuf::from(p).join("shigoto"))
            .unwrap_or_else(|_| {
                let home = env::home_dir().expect("No Home directory");
                home.join(".local").join("share").join("shigoto")
            });
        if !data_path.exists() {
            fs::create_dir_all(&data_path)
                .expect("Cannot create data_dir");
        }
        let data_file = data_path.join("data.json");

        if !data_file.exists() {
            fs::File::create(&data_file).expect("Failed to create file");
            return Ok(Config {
                data_file: data_file,
                user_data: UserData::new()
            })
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&data_file)?;
        let user_data: UserData = match serde_json::from_reader(file) {
            Ok(r) => r,
            Err(_) => UserData::new(),
        };
        Ok(Config { data_file: data_file, user_data: user_data })
    }
}

impl Config {
    fn save(self) -> Result<(), Error> {
        let j = serde_json::to_string(&self.user_data)?;
        let f = OpenOptions::new()
            .write(true)
            .open(&self.data_file)?;
        serde_json::to_writer(f, &j)?;
        Ok(())
    }
}