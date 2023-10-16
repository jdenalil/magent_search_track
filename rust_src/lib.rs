use pyo3::prelude::*;


// env constants
#[allow(dead_code)]
enum Actions {
    NoAction = 0, 
    MoveLeft = 1,
    MoveRight = 2, 
    MoveDown = 3, 
    MoveUp = 4,
}

#[allow(dead_code)]
enum Observations {
    SelfVelX = 0, 
    SelfVelY = 1, 
    SelfPosX = 2,
    SelfPosY = 3,
    LandmarkOneRelPositionX = 4,
    LandmarkOneRelPositionY = 5,
    LandmarkTwoRelPositionX = 6,
    LandmarkTwoRelPositionY = 7,
    LandmarkThreeRelPositionX = 8,
    LandmarkThreeRelPositionY = 9,
    OtherAgentOneRelPositionX = 10,
    OtherAgentOneRelPositionY = 11,
    OtherAgentTwoRelPositionX = 12,
    OtherAgentTwoRelPositionY = 13,
    CommunicationOne = 14,
    CommunicationTwo = 15,
    CommunicationThree = 16,
    CommunicationFour = 17,
}

#[pyfunction]
fn towards_landmark(_py: Python, observations: Vec<f64>, target: i32) -> PyResult<i32> {
    // turn the target assignment into the actual target location
    let offset: i32 = target * 2;
    let x_rel: f64 = observations[(Observations::LandmarkOneRelPositionX as i32 + offset) as usize];
    let y_rel: f64 = observations[(Observations::LandmarkOneRelPositionY as i32 + offset) as usize];
    let x_rel_abs: f64 = x_rel.abs();
    let y_rel_abs: f64 = y_rel.abs();

    // figure out if we are further from the target in the x or y direction
    // and which direction we should actually go
    if x_rel_abs < y_rel_abs {
        if y_rel > 0.0 {
            Ok(Actions::MoveUp as i32)
        } else {
            Ok(Actions::MoveDown as i32)
        }
    } else {
        if x_rel > 0.0 {
            Ok(Actions::MoveRight as i32)
        } else {
            Ok(Actions::MoveLeft as i32)
        }
    }
}

#[pyfunction]
fn return_one() -> PyResult<i32> {
    return Ok(1)
}


/// A Python module implemented in Rust.
#[pymodule]
fn magent_autonomy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(return_one, m)?)?;
    m.add_function(wrap_pyfunction!(towards_landmark, m)?)?;
    Ok(())
}
