mod hw1;

mod utils;
use utils::Error;

#[repr(C)]
enum Task {
    Task1,
    Task2,
    Task3,
}

fn main() -> Result<(), Error> {
    #[cfg(any(
        feature = "hw1",
        feature = "hw1-task-1",
        feature = "hw1-task-2",
        feature = "hw1-task-3"
    ))]
    {
        const HW_1: [fn() -> Result<(), Error>; 3] =
            [hw1::task_1::main, hw1::task_2::main, hw1::task_3::main];

        #[cfg(any(feature = "hw1", feature = "hw1-task-1"))]
        HW_1[Task::Task1 as usize]()?;

        #[cfg(any(feature = "hw1", feature = "hw1-task-2"))]
        HW_1[Task::Task2 as usize]()?;

        #[cfg(any(feature = "hw1", feature = "hw1-task-3"))]
        HW_1[Task::Task3 as usize]()?;
    }

    Ok(())
}
