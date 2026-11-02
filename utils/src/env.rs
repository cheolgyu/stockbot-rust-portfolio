use std::error::Error;

pub fn envload() {
    load_env_file();
}

#[cfg(not(debug_assertions))]
pub fn envload_batch_plan() -> Result<(), Box<dyn Error>> {
    // envload_batch_plan 는 dockr run 에서 주입하므로 필요없음
    // 디버그 시에는 파일에서 가져오는걸로.
    Ok(())
}
#[cfg(debug_assertions)]
pub fn envload_batch_plan() -> Result<(), Box<dyn Error>> {
    dotenvy::from_filename(".env.batch_plan-init")?;
    Ok(())
}

#[cfg(debug_assertions)]
fn load_env_file() {
    use std::env;

    println!("Debugging enabled");

    if are_you_on_linux() {
        //dotenvy::from_filename(".env.dev.docker").ok().unwrap();
    } else {
        println!("load .env");
        //dotenvy::from_filename(".env.dev.windos").ok().unwrap();
        dotenvy::from_filename(".env").ok().unwrap();
    }
    println!("===========================================");
    println!("LOAD ENV_NAME: {:?}", env::var("ENV_NAME"));
    println!("===========================================");
}

#[cfg(not(debug_assertions))]
fn load_env_file() {
    println!("load .env.prod");
    dotenvy::from_filename(".env.prod").ok().unwrap();
}

#[cfg(target_os = "linux")]
fn are_you_on_linux() -> bool {
    println!("You are running linux!");
    true
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() -> bool {
    println!("You are *not* running linux!");
    false
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        envload();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
