#[test]
fn nth_byte() {
    use utils::get_nth_byte;

    let number = 0x01_02_03_04_05_06_07_08u64;
    println!("numbers: {:#x}", number);
    for i in 0..8 {
        println!("i = {}, get_nth_byte() = {}", i, get_nth_byte(number,i as usize));
        assert_eq!(get_nth_byte(number, i as usize), (i+1) as u8);
    }
    
}
