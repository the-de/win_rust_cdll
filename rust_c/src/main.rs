
fn main() {

    let get_u32 = ||{
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        a.trim().parse::<u32>().unwrap_or_default()
    };

    loop {
        let a = get_u32();
        let b = get_u32();
        let ans = unsafe { add(a,b) };
        println!("{}",ans);
        if ans == 100{
            break;
        }
    }
}

extern "C" { 
    fn add(a:u32,b:u32)->u32; 
}

