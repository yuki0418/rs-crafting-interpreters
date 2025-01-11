use std::{
    fs::{self, OpenOptions},
    io::Write,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }
    let output_dir = &args[1];

    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   : Expr left, Token operator, Expr right".to_string(),
            "Grouping : Expr expression".to_string(),
            "Literal  : String value".to_string(),
            "Unary    : Token operator, Expr right".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> std::io::Result<()> {
    fs::create_dir_all(output_dir)?;
    let path: String = format!("{output_dir}/{base_name}.rs");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(format!("struct {base_name} {{}}\n").as_bytes())?;

    for type_ in types {
        let class_name = type_.split(":").next().unwrap().trim();
        let fields = type_.split(":").nth(1).unwrap().trim();
        define_type(&mut file, class_name, fields)?;
    }

    Ok(())
}

fn define_type(file: &mut fs::File, class_name: &str, field_list: &str) -> std::io::Result<()> {
    file.write_all(format!("struct {class_name} {{\n").as_bytes())?;

    // Store parameters in fields.
    let fields: Vec<&str> = field_list.split(", ").collect();
    for field in fields {
        let type_ = field.split(" ").next().unwrap();
        let name = field.split(" ").nth(1).unwrap();
        file.write_all(format!("\t\t{name}: {type_},\n").as_bytes())?;
    }

    file.write_all("}\n".to_string().as_bytes())?;

    Ok(())
}
