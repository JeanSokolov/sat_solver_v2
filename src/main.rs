use std::{
    f64::INFINITY,
    fs::File,
    io::{stdin, BufRead, BufReader},
};

const DEFAULT_PATH: &'static str = "./KI.txt";

fn parse_file() -> std::io::Result<Vec<String>> {
    let mut args = std::env::args();
    let file_path = args.nth(1).unwrap_or_else(|| DEFAULT_PATH.to_string());
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut tmp = Vec::new();
    for lines in reader.lines() {
        let tmpstr = lines.unwrap();
        if !tmpstr.contains("//") {
            tmp.push(tmpstr);
        }
    }
    return Ok(tmp);
}

fn transform_equation(
    most_significant_constraint: &mut Vec<f64>,
    current_index: usize,
) -> Vec<f64> {
    let len = most_significant_constraint.len();
    let mut transformed_equation: Vec<f64> = Vec::new();
    for mut values in most_significant_constraint.clone() {
        values = values / most_significant_constraint[current_index] * -1.0;
        transformed_equation.push(values);
    }

    return transformed_equation;
}

fn generate_matrix_objective_fn(objective_function: &String, number_of_constraints: usize) -> Vec<f64> {
    let mut objective_fn: Vec<f64> = Vec::new();
    let variables: Vec<&str> = objective_function.split('+').collect();
    for s in variables.into_iter().skip(1) {
        objective_fn.push(
            s.trim().split('*').collect::<Vec<&str>>()[0]
                .parse::<f64>()
                .unwrap(),
        );
    }
    // add slack-variables to objective fn
    for i in 0..number_of_constraints{
        objective_fn.push(-0.0);
    }

    return objective_fn;
}

fn generate_matrix_constraints(file_content: &mut Vec<String>) -> Vec<Vec<f64>> {
    let mut constraints: Vec<Vec<f64>> = Vec::new();
    let number_of_constraints = file_content.len();
    for (index,s) in file_content.iter().enumerate() {
        let tmp_vec_str: Vec<&str> = s.split('+').collect::<Vec<&str>>();
        let mut tmp_vec_f64: Vec<f64> = Vec::new();
        let mut amount_variables:usize = 0;
        // grabbing lhs
        for t in tmp_vec_str.clone().into_iter().skip(1) {
            tmp_vec_f64.push(
                t.trim().split('*').collect::<Vec<&str>>()[0]
                    .parse::<f64>()
                    .unwrap(),
            );
            amount_variables = tmp_vec_f64.len();
        }
        // adding slack variables
        // ToDo: add R1,2,3 aka artificial vars etc.
        for i in 0..number_of_constraints{
            tmp_vec_f64.push(0.0);
        }
        // slack variables as -1, as code only needs to cover minimization problems
        tmp_vec_f64[index+amount_variables] = -1.0;

        // grabbing rhs
        tmp_vec_f64.push(
            tmp_vec_str
                .clone()
                .last()
                .unwrap()
                .trim()
                .split(">=")
                .collect::<Vec<&str>>()[1]
                .replace(";", "")
                .replace(" ", "")
                .parse::<f64>()
                .unwrap(),
        );
        constraints.push(tmp_vec_f64);
    }

    return constraints;
}

fn is_calc_done(mut objective_fn: Vec<f64>) -> bool{
    let max_factor = (objective_fn
        .iter()
        .max_by(|(a), (b)| a.total_cmp(b)))
    .unwrap();
    if max_factor.le(&0.0){
        return true;
    }else{
        return false;
    }
}
// !is_calc_done(objective_fn[0..objective_fn.len()-constraints.len()].to_vec()) 

fn solve(mut objective_fn: Vec<f64>, mut constraints: Vec<Vec<f64>>) {
    let mut i: usize = 0;
    let mut cost: f64 = 0.0;
    // max # of iterations = ammount of variable
    while i < objective_fn.len() {
        // find most promising variable
        let mut tmp_obj_fn = objective_fn.clone();
        for n in 0..tmp_obj_fn.len(){
            if tmp_obj_fn[n].eq(&-0.0){
                tmp_obj_fn[n]=-INFINITY;
            }
        }

        let mut index_max_factor = (tmp_obj_fn
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index))
        .unwrap();
        println!("Current variable: x{}", index_max_factor);
        // find most limiting constraint
        let mut lhs: Vec<f64> = Vec::new();
        let mut rhs: Vec<f64> = Vec::new();
        for constraint in constraints.clone() {
            lhs.push(constraint[index_max_factor]);
            rhs.push(constraint.last().unwrap().to_owned());
        }
        let mut n: usize = 0;
        // iterate through the new equations, dividing rhs value by coefficient of lhs summand e.g. 3x=12 to x=4
        // while doing so, check that the variables respect the non-negative constraint
        while n < lhs.len() {
            //println!("Pre Calc {:?}={:?}", lhs[n], rhs[n]);
            if lhs[n].ge(&0.0){
                if rhs[n].ge(&0.0){
                    rhs[n] = rhs[n] / lhs[n];
                    lhs[n] = 1.0;
                }else{
                    rhs[n] = INFINITY;
                    lhs[n] = INFINITY;
                }
            }else{
                if rhs[n].ge(&0.0){
                    rhs[n] = INFINITY;
                    lhs[n] = INFINITY;
                }else{
                    rhs[n] = rhs[n] / lhs[n];
                    lhs[n] = 1.0;
                }
            }
            //println!("Post Calc {:?}={:?}", lhs[n], rhs[n]);
            n += 1;
        }
        let max_val = rhs.iter()
            .max_by(|a, b| a.total_cmp(b))
        .unwrap();
        let min_val = rhs.iter()
            .min_by(|a, b| a.total_cmp(b))
        .unwrap();
        if min_val.eq(max_val){
            println!("optimum found");
            break;
        }
        // determine most restrictive constraint for (current/remaining) most significant variable in objective function
        let most_significant_constraint_index = (rhs
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index))
        .unwrap();
        //println!("{:?}", most_significant_constraint_index);
        let mut most_significant_constraint = &mut constraints[most_significant_constraint_index];

        let x: Vec<f64> = transform_equation(&mut most_significant_constraint, index_max_factor);

        //println!("TRANSFORMED CONSTRAINT VALUES: {:?}", x);
        for j in 0..lhs.len() {
            let mult = constraints[j][index_max_factor];
            /* println!(
                "Constraint {} of iteration {}: {:?}",
                j + 1,
                i,
                constraints[j]
            ); */
            if j.ne(&most_significant_constraint_index) {
                for k in 0..constraints[j].len() {
                    constraints[j][k] += (x[k] * mult);
                    /* println!("X: {:?}", x[k]);
                    println!("SUMFACCON: {:?}", mult); */
                }
                constraints[j][index_max_factor] = 0.0;
            } else {
                for k in 0..constraints[j].len() {
                    constraints[j][k] /= mult;
                }
            }
        }
        // change objective function
        let mult = objective_fn[index_max_factor];
        for k in 0..x.len() {
            if k.ne(&x.len().checked_sub(1).unwrap()){
                objective_fn[k] += (x[k] * mult);
                //println!("{:?}{:?}",x, mult);
            }else{
                cost += (x.last().unwrap() * mult);
            }
        }
        for k in constraints.clone() {
            //println!("NEW CONSTRAINT VALUES: {:?}", k);
        }
        objective_fn[index_max_factor] = 0.0;
        println!("\nNEW OBJECTIVE FUNCTION: {:?}= {cost}", objective_fn);
        i += 1;
    }
    println!("\nDone after {} iterations", i);
    let mut variable_index = Vec::new();
    for n in 0..objective_fn.len()-constraints.len(){
        if objective_fn[n].eq(&0.0){
            variable_index.push(n);
        }
    }
    let mut variable_values = Vec::new();
    for index in variable_index.clone(){
        let mut tmp_vec = Vec::new();
        for k in constraints.clone(){
            tmp_vec.push(k[index]);
        }
        let mut index_max_factor = (tmp_vec
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index))
        .unwrap();
        variable_values.push(constraints[index_max_factor].last().unwrap());
    }
    println!("p = {cost}");
    for i in 0..variable_index.len(){
        println!("x{} = {}", variable_index[i], variable_values[i]);    
    }
}

fn main() {
    // Read file
    let mut file_content = parse_file().unwrap();
    //debug_info(&mut file_content);
    let mut objective_function = &file_content.clone()[0];
    file_content.remove(0);
    let mut tmp = String::new();
    // convert minimization function to maximization function
    if objective_function.contains("min") {
        tmp = objective_function.replace("min:", "max:");
        tmp = tmp.replace("+ ", "+ -");
        objective_function = &tmp;
    }
    // convert string/substrings to 32-bit floats, generating a matrix
    let objective_fn = generate_matrix_objective_fn(objective_function, file_content.len());
    let constraints = generate_matrix_constraints(&mut file_content);

    println!("Objective f: {:?}", objective_fn);
    println!("Constraints: {:?}", constraints);

    // Run simplex algorithm
    solve(objective_fn, constraints);
    //find_most_significant_constraint(objective_function, &mut file_content);

    // Keep program window open, close by pressing Return key
    let mut input = String::new();
    stdin().read_line(&mut input);
}
