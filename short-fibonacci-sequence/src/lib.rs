pub fn create_empty() -> Vec<u8> {
     Vec::new()
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut sequence = vec![1, 1];
    for i in 2..5 {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }
    sequence
}
