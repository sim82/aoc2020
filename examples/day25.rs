fn main() {
    let pk_card = 12090988;
    let pk_door = 240583;
    // let pk_card = 5764801;
    // let pk_door = 17807724;

    let mut v = 1u64;
    let s = 7;

    let mut loops_door = 0;
    let mut loops_card = 0;

    for i in 1.. {
        v = (v * s) % 20201227;

        if pk_card == v && loops_card == 0 {
            println!("card loops: {}", i);
            loops_card = i;
        }
        if pk_door == v && loops_door == 0 {
            println!("door loops: {}", i);
            loops_door = i;
        }
        if loops_card != 0 && loops_door != 0 {
            break;
        }
    }

    let enc_door = (0..loops_door).fold(1, |a, _| (a * pk_card) % 20201227);
    let enc_card = (0..loops_card).fold(1, |a, _| (a * pk_door) % 20201227);

    println!("enc card: {} door: {}", enc_card, enc_door);
}
