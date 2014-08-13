use std::sync::Arc;

fn main() {
    let numbers = vec![1i, 2i, 3i];

    let (tx, rx) = channel();
    tx.send(numbers);

    spawn(proc(){
        let numbers = rx.recv();
        println!("{}", numbers[0]);
    });

    main2();
}

fn main2() {
    let numbers = vec![1i, 2i, 3i];
    let numbers = Arc::new(numbers);

    for num in range(0u, 3) {
        let (tx, rx) = channel();
        tx.send(numbers.clone());

        spawn(proc(){
            let numbers = rx.recv();
            println!("{:d}", (*numbers)[num as uint]);
        })
    }
}
