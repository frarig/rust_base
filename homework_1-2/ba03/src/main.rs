fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    insertion_sort(args).iter().for_each(|x| println!("{}", x));
}

fn insertion_sort(mut args: Vec<String>) -> Vec<String> {
    for i in 1..args.len() {
        let mut j = i;

        while j > 0 && args[j - 1] > args[j] {
            args.swap(j - 1, j);
            j -= 1;
        }
    }
    args
}

#[test]
fn test_insertion_sort() {
    assert_eq!(
        insertion_sort(vec!["a".into(), "b".into(), "c".into()]),
        vec!["a", "b", "c"]);

    assert_eq!(
        insertion_sort(vec!["e".into(), "d".into(), "b".into(), "a".into(), "c".into()]),
        vec!["a", "b", "c", "d", "e"]);

    assert_eq!(
        insertion_sort(vec!["A".into(), "a".into(), "A".into(), "a".into()]),
        vec!["A", "A", "a", "a"]);

    assert_eq!(
        insertion_sort(vec!["world".into(), "Hello".into()]),
        vec!["Hello", "world"]);

    assert_eq!(
        insertion_sort(vec!["World".into(), "hello".into()]),
        vec!["World", "hello"]);

    assert_eq!(
        insertion_sort(vec!["hello,".into(), "world,".into(), "this".into(), "is".into(), "a".into(), "program".into()]),
        vec!["a", "hello,", "is", "program", "this", "world,"]);
}