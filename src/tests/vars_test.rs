use super::super::funcs::var;
use super::*;
use std::collections::HashMap;

pub fn var_works() {
    // var x str >> something
    let mut statement = "var x str >> something";
    let mut cmd: Vec<&str> = statement.split(" ").collect();

    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_str.get(&"x") {
        Some(&value) => assert_str(value, "something", statement),
        _ => assert_str("fail", "something", statement),
    }

    // var y int >> 50
    statement = "var y int >> 50";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_int.get(&"y") {
        Some(&value) => assert_f64(value, 50.0, statement),
        _ => assert_str("fail", "something", statement),
    }
}

pub fn move_works() {
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    // move int lx x
    let mut statement: &str = "var x int >> 40";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);

    statement = "move int lx x";
    cmd = statement.split(" ").collect();

    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_int.get(&"x") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int rv y
    let mut statement: &str = "var y int >> 30";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);

    statement = "move int rv y";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_int.get(&"y") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str string z
    statement = "var z str >> ok";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);

    statement = "move str string z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_str.get(&"z") {
        Some(&value) => assert_str(value, vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str lxstring z
    statement = "var z str >> ok2";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);

    statement = "move str lxstring z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_str.get(&"z") {
        Some(&value) => assert_str(value, vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var exm string
    vars.string = "hello".to_string();
    statement = "move str var exm string";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_str.get(&"exm") {
        Some(&value) => assert_str(value, vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var lxstr lxstring
    vars.lxstring = "hello2".to_string();
    statement = "move str var lxstr lxstring";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_str.get(&"lxstr") {
        Some(&value) => assert_str(value, vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint lx
    vars.lx = 15.0;
    statement = "move int var testint lx";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_int.get(&"testint") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint rv
    vars.rv = 16.0;
    statement = "move int var testint rv";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, 0, 1);
    match vars.hash_int.get(&"testint") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }
}
