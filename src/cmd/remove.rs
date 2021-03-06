use serde_json::Error;
use config::Config;

pub fn execute(conf: &mut Config, task_id: &str) -> Result<(), Error> {
    println!("Removing task with id {:?}", task_id);
    conf.user_data.rm_task(task_id.parse().unwrap()).unwrap();
    conf.save().unwrap();
    Ok(())
}
