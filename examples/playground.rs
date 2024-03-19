use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // This does not compile because v is still owned by main
    //
    // task::spawn(async {
    //     println!("Here's a vec: {:?}", v);
    // });
    //
    //  error[E0373]: async block may outlive the current function, but
    //  it borrows `v`, which is owned by the current function
    //  --> src/main.rs:7:23
    //   |
    // 7 |       task::spawn(async {
    //   |  _______________________^
    // 8 | |         println!("Here's a vec: {:?}", v);
    //   | |                                        - `v` is borrowed here
    // 9 | |     });
    //   | |_____^ may outlive borrowed value `v`
    //   |
    // note: function requires argument type to outlive `'static`
    //  --> src/main.rs:7:17
    //   |
    // 7 |       task::spawn(async {
    //   |  _________________^
    // 8 | |         println!("Here's a vector: {:?}", v);
    // 9 | |     });
    //   | |_____^
    // help: to force the async block to take ownership of `v` (and any other
    //       referenced variables), use the `move` keyword
    //   |
    // 7 |     task::spawn(async move {
    // 8 |         println!("Here's a vec: {:?}", v);
    // 9 |     });
    //   |

    // "note: function requires argument type to outlive `'static`"
    //  ^-- that is interresting
    // Note that the error message talks about the argument type outliving the
    // 'static lifetime. This terminology can be rather confusing because the
    // 'static lifetime lasts until the end of the program, so if it outlives it,
    // don't you have a memory leak? The explanation is that it is the type, not the
    // value that must outlive the 'static lifetime, and the value may be destroyed
    // before its type is no longer valid.

    // When we say that a value is 'static, all that means is that it would not be
    // incorrect to keep that value around forever. This is important because the
    // compiler is unable to reason about how long a newly spawned task stays
    // around. We have to make sure that the task is allowed to live forever, so
    // that Tokio can make the task run as long as it needs to.

    // The article that the info-box earlier links to uses the terminology
    // "bounded by 'static" rather than "its type outlives 'static" or "the value is
    // 'static" to refer to T: 'static. These all mean the same thing, but are
    // different from "annotated with 'static" as in &'static T.

    // This works because we move v to the async block
    //
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });

    // This obviously doesn't work as we just moved v to the async block
    //
    // println!("Here's a vec: {:?}", v);

    // If a single piece of data must be accessible from more than one task
    // concurrently, then it must be shared using synchronization primitives
    // such as Arc.
}
