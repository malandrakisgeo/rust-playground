use sysinfo::SystemExt;
use std::os::raw::c_char;
use std::io::{BufReader, Read};
use std::fs::File;
use rand::{distributions::Alphanumeric, Rng};
use rand::distributions::Standard; // 0.8


//    static system = sysinfo::System::new();


fn main() {

    unsafe{
        //wipeWithRandomData();
        wipeWithZeroes();
    }

}

unsafe fn wipeWithZeroes(){
    let mut vec = Vec::new();

    loop {

        //file.read(&mut buffer);
        //let val = buffer;
        let val = [2; 1024];
        vec.push(val);
    }

}


unsafe fn wipeWithRandomData(){

    //let file = libc::fopen(&urandomPath, &mode);

    let mut file = File::open("/dev/random").unwrap();
    let mut buffer = [0; 1024];
    let mut vec = Vec::new();

    //Random String.
    let randomString: String = rand::thread_rng()
        .sample_iter(&Standard)
        .take(10000)
        .map(char::from)
        .collect();

//Rythmoi xelwnas apo th Kina
    loop {
        file.read(&mut buffer);
        let val = buffer;
        vec.push(val);

    }
}

/*
    1. Diavase gia tis static metavlhtes
    2. c_char & string
    3. Rust unsafe blocks?
    4. c_void?
    5. Box. Isws to antistoixo calloc ths Rust.
    6. the trait `From<&str>` is not implemented for `i8`

 */