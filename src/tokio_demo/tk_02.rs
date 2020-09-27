#[test]
fn test_tk_task1() {
    use tokio::task;

    let result = task::block_in_place(|| {
        // do some compute-heavy work or call synchronous code
        "blocking completed"
    });
    assert_eq!(result, "blocking completed");
}


