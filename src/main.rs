fn main() {
    let a: usize = 5;
    print!("{}", (a as f32 / 2.).ceil() as usize);
    let a: usize = 4;
    print!("{}", (a as f32 / 2.).ceil() as usize);
}

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut ls: Vec<_> = heights
        .into_iter()
        .map(|x| -x)
        .zip(names.into_iter())
        .collect();
    ls.sort_unstable();
    let (_, names): (Vec<i32>, Vec<String>) = ls.into_iter().unzip();
    names
}
