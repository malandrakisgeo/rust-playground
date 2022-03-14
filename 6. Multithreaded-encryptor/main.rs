use std::{fs, io, process};
use std::fs::File;
use std::io::{Read};
use std::path::Path;
use std::sync::{Arc, Barrier};

use clap::{Arg, Command};
use threadpool::ThreadPool;
use walkdir::WalkDir;


fn encrypt_file(
    filepath: &str,
    pass: &str,
) -> Result<(), io::Error> {

    println!("The filepath: {}", filepath);

    let text_vec = String::from_utf8_lossy(&fs::read(filepath).unwrap()).as_bytes().to_vec();
    println!("File read complete:");

    let mut encryptionPassword: String;
    if pass.len() < 32 {
        let length = 32 - pass.len();
        let strr = (0..length).map(|_| "0").collect::<String>().to_owned();
        encryptionPassword = format!("{}{}", pass, strr);
    }else if pass.len() > 32 {
        encryptionPassword = String::from(&pass[..32]);
    }else {
        encryptionPassword = String::from(pass);
    }

    println!("encryptionPassword: {:?}",&encryptionPassword);

    let ciphertext = enc_file::encrypt_aes(text_vec, &encryptionPassword.as_str()).unwrap();
    //let ciphertext = String::from_utf8_lossy(&enc_file::encrypt_aes(text_vec, &encryptionPassword.as_str()).unwrap()).as_bytes().to_vec();

    let enc = ".encrypted";
    let dist = format!("{}{}", filepath, enc);

    fs::write(&dist, &ciphertext)?;
    println!("wrote");
    drop(&dist);



    let mut buffer = String::new();
    let mut file  = File::open(&dist)?;
    //File::open(&dist).unwrap().read_to_string(&mut buffer)?;
    //let decrypted = enc_file::decrypt_aes(buffer.into_bytes().to_vec(), encryptionPassword.as_str());
    //let mut buffer= Vec::new();
    file.read_to_string(&mut buffer);
    println!("file opened");
    println!("e: {:?}",&buffer );

    let decrypted = enc_file::decrypt_aes(buffer.into_bytes(), encryptionPassword.as_str()).unwrap();
    println!("Decrypted: {:?}", &decrypted );

    /*
    LEITOURGIKOS SYNDYASMOS:
        let ciphertext = encrypt_aes(text_vec, encryptionPassword.as_str()).unwrap();
    fs::write(&dist, &ciphertext);

 */

    Ok(())

}


fn main() {
    let matches = Command::new("Multithreaded encryptor by GEORGE MALANDRAKIS")
        .version("0.1.0")
        .arg(Arg::new("password")
            .short("p".parse().unwrap())
            .long("password")
            .takes_value(true)
            .help("Encryption password"))
        .arg(Arg::new("path")
            .short("D".parse().unwrap())
            .long("path") //Double quotes needed!
            .takes_value(true)
            .help("Path to files"))
        .get_matches();

    let pass = matches.value_of("password").unwrap_or_else(
        || { "null" }
    ); //htan owned

    let path = matches.value_of("path").unwrap_or_else(
        || { process::exit(1); }
    ); //htan owned

    println!("The password: {}", pass);
    println!("The path: {}", path);


    let thread_pool = ThreadPool::new(num_cpus::get()); //As many threads as virtual CPUs.

    let barrier = Arc::new(Barrier::new(thread_pool.max_count() +1));

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir())
    {
        let filepath = entry.path().to_owned();
        let a3 = pass.to_owned();
        let barrier = barrier.clone();
        thread_pool.execute(move || {
            encrypt_file(filepath.to_str().unwrap(), a3.as_str()).unwrap();
            barrier.wait();
        });
    }

    barrier.wait();
}

