fn stringify(t: impl ToString) -> String {
    t.to_string()
}

fn main() {
    stringify("a");
    stringify(1i32);
    stringify(1u32);
    stringify(1i64);
    stringify(1u64);
}