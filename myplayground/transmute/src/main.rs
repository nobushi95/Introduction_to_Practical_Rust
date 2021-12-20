fn main() {
    let pt1 = Box::new(10); // スタックにおかれた？10がヒープにおかれ、pt1が指す
    // Boxポインタと*mutポインタはどちらも同じビット幅なのでtransumuteできる
    let p2: *mut i32 = unsafe { std::mem::transmute(pt1) }; // 生ポインタにキャスト

    let f1 = 5.6789+3_f32; // 5678.9
    let i1 = f1 as i32;
    println!("i1: {}", i1);
    let i2: i32 = unsafe { std::mem::transmute(f1) }; // ビット幅が同じならそのビット並びを維持して型変換する
    println!("i2: {}", i2);
}
