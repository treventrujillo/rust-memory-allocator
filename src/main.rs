mod allocator;

fn main() {
    let len = 4096;
    let address = allocator::alloc(len);

    println!("Address acquired {:#?}\n", address);

    let dealloc_result = allocator::dealloc(address, len);

    if 0 == dealloc_result {
        println!("Deallocated {:?} bytes at address {:#?}", len, address);
    }
    if -1 == dealloc_result {
        eprintln!(
            "Failed to deallocate {:?} bytes at address {:#?}",
            len, address
        )
    }
}
