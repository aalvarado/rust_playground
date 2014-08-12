fn main() {
    let numbers = vec![1i, 2i, 3i];

    let (tx, rx) = channel();
    tx.send(numbers);

    spawn(proc(){
        let numbers = rx.recv();
        println!("{}", numbers[0]);
    });
}
