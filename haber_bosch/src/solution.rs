use ode_solvers::{Vector3, System, Dop853};

// Constants:
/*
E_a = 27.1
A = 2.73 * 10^{-2}
T = 400.0
R = 8.31446261815324
b = 2.0
*/

type State = Vector3<f64>;
const GAS_CONSTANT: f64 = 8.31446261815324;

// A derived debug trait adding functionality to HaberBoschModel structure
#[derive(Debug)]
pub struct HaberBoschModel {
    pub rf: f64,
    pub rr: f64,
}

// implementation of the System<V> trait to communicate about the system of equations with the ode solver.
impl System<State> for HaberBoschModel {
    fn system(&self, _x: f64, y: &State, dy: &mut State) {
        dy[0] = -1. * self.rf * y[0] * y[1] + 1. * self.rr * y[2].powi(2);
        dy[1] = -3. * self.rf * y[0] * y[1] + 3. * self.rr * y[2].powi(2);
        dy[2] = 2. * self.rf * y[0] * y[1] - 2. * self.rr * y[2].powi(2);
    }
}

// a function signature consists of: fn keyword, identifier, list of arguments (identifier: type) and an optional return type after ->
pub fn arrhenius(ea: f64, a : f64, temp: f64) -> f64 {
     a * (-ea / (GAS_CONSTANT * temp)).exp()
}

pub fn solution_main() {
    // here we use variables to store different types.
    let ea = 27.1;
    let power = -2;
    let exp_f = 2.73 * 10.0f64.powi(power);

    // we call the arrhenius function to calculate the forward reaction rate at 400°
    let rate_f = arrhenius(ea, exp_f, 400.0);
    println!("RF: {}", rate_f);

    // we generate and output the model using the Debug trait and {:?} placeholder
    let model = HaberBoschModel { rf: rate_f, rr: rate_f * 2.0 };
    println!("Model: {:?}", model);

    // for one part of nitrogen use three parts of hydrogen.
    let y0 = State::new(1./4., 3./4., 0.);

    let mut stepper = Dop853::new(model, 0.0, 200.0, 0.1, y0, 10e-8, 10e-12);
    let res = stepper.integrate();
    match res {
        Ok(state) => println!("{}", state),
        Err(e) => println!("Error: {}", e),
    }

    // also we start with a sum of 1.0 we end around 0.81
    // thats ok, due to the underlying units, for a plot of we should normalize.
    let y_out = stepper.y_out();
    println!("{:?}", y_out.iter().map(|x| x[0] + x[1] + x[2]).collect::<Vec<f64>>());
}