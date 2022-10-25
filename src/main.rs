//Documentation: https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#read-lines-of-strings-from-a-file

//A simple screenfetch program written in Rust. 
use std::process::Stdio;
use std::process::Command;
extern crate term;

fn main(){
    let mut leelofetch = term::stdout().unwrap();
    leelofetch.attr(term::Attr::Bold).unwrap();
    println!("leelofetch");
    
    println!("----------");
//Hostname

    let hostname = Command::new("hostname")
        .output()
        .expect("Failed");
        leelofetch.fg(term::color::GREEN).unwrap();
        print!("Hostname:");
        leelofetch.fg(term::color::WHITE).unwrap();
        print!("          {}",String::from_utf8_lossy(&hostname.stdout));

//Kernel
    let kernel = Command::new("uname")  
        .arg("-r")
        .output() 
        .expect("Failed");
     leelofetch.fg(term::color::GREEN).unwrap();
     print!("Kernel:");
     leelofetch.fg(term::color::WHITE).unwrap(); 
     print!("            {}", String::from_utf8_lossy(&kernel.stdout));

//OS
    let osin = Command::new("uname")
        .arg("-a")
        .stdout(Stdio::piped()).spawn().unwrap();
        
    let osout = Command::new("awk")
        .arg("/Linux/{print $2}")
        .stdin(osin.stdout.unwrap())
        .output()
        .expect("awk failed");
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("OS:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("                {}", String::from_utf8_lossy(&osout.stdout));



//Packages
  
    let pkgs = Command::new("dpkg")
        .arg("--list")
        .stdout(Stdio::piped()).spawn().unwrap();

    let pkgs2 = Command::new("wc")
        .arg("-l")
        .stdin(pkgs.stdout.unwrap())
        .output()
        .unwrap();
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("Packages:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("          {}", String::from_utf8_lossy(&pkgs2.stdout));


//Time and Date
    let date = Command::new("date")
        .output()
        .expect("date failed");
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("Time and Date:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("     {}", String::from_utf8_lossy(&date.stdout));

//Uptime
    let up = Command::new("uptime")
        .arg("-p")
        .output() 
        .expect("Up time failed");
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("Uptime:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("            {}", String::from_utf8_lossy(&up.stdout));

//CPU
    let cpu = Command::new("awk")
        .arg("/model name/{print $4,$5,$6,$7,$8,$9,$10,$11,$12 ;exit}")
        .arg("/proc/cpuinfo")
        .output()
        .expect("CPU Failed");
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("CPU:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("               {}", String::from_utf8_lossy(&cpu.stdout));


//GPU    
    let gpuin = Command::new("lspci")
        .stdout(Stdio::piped()).spawn().unwrap();

    let gpuout = Command::new("awk")
        .arg("/VGA compatible controller/{print $5,$6,$7,$8,$9,$10;exit}")
        .stdin(gpuin.stdout.unwrap())
        .output()
        .unwrap();
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("GPU:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("               {}", String::from_utf8_lossy(&gpuout.stdout));

//RAM
    let ramin = Command::new("free")
        .arg("-m")
        .stdout(Stdio::piped()).spawn().unwrap();
    let ramout = Command::new("awk")
        .arg("/Mem:/{print $2}")
        .stdin(ramin.stdout.unwrap())
        .output()
        .unwrap();
    let ram_string = "Mb:";
    leelofetch.fg(term::color::GREEN).unwrap();
    print!("RAM:");
    leelofetch.fg(term::color::WHITE).unwrap();
    print!("               {}{}",ram_string, String::from_utf8_lossy(&ramout.stdout));

//Ethernet Card
    let netin = Command::new("lspci")
        .stdout(Stdio::piped()).spawn().unwrap();

    let eth_netout = Command::new("awk")
        .arg("/Ethernet controller:/{print $4, $5, $6, $7, $8, $9, $10, $11, $12; exit}")
        .stdin(netin.stdout.unwrap())
        .output()
        .unwrap(); 
    let eth_convert = String::from_utf8_lossy(&eth_netout.stdout);
    
    if eth_convert.len() != 0 {
        leelofetch.fg(term::color::GREEN).unwrap();
        print!("Ethernet:");
        leelofetch.fg(term::color::WHITE).unwrap();
        print!("          {}",String::from_utf8_lossy(&eth_netout.stdout));
    }
//Wifi Card
    let wfin = Command::new("lspci")
        .stdout(Stdio::piped()).spawn().unwrap();
    let wireless_netout = Command::new("awk")
        .arg("/Network controller:/{print $4, $5, $6, $7, $8, $9, $10, $11, $12; exit}")
        .stdin(wfin.stdout.unwrap())
        .output()
        .unwrap();
    let wfin_convert = String::from_utf8_lossy(&wireless_netout.stdout);
    
    if wfin_convert.len() != 0 {
        leelofetch.fg(term::color::GREEN).unwrap();
        print!("Wireless:");
        leelofetch.fg(term::color::WHITE).unwrap();
        print!("          {}",String::from_utf8_lossy(&wireless_netout.stdout));
    }
}
