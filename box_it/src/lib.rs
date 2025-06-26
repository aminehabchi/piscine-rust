pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let parts = s.split(' ');
    let mut arr: Vec<u32> = Vec::new();

    for part in parts {
        let mut part_str = part.to_string();

        let last = part_str.pop();
        if last == Some('k') {
            arr.push(part_str.parse::<u32>().unwrap_or(0) * 1000);
        } else {
            if let Some(c) = last {
                part_str.push(c);
            }
            arr.push(part_str.parse::<u32>().unwrap_or(0));
        }
    }

    Box::new(arr)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    let mut arr: Vec<u32> = Vec::new();
    for val in a.iter() {
        arr.push(*val);
    }
    arr
}
