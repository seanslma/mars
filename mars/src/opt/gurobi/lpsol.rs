use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

lazy_static! {
    static ref PATTERN_1: Regex =
        Regex::new(r"([a-zA-Z0-9_]+)\|(\d+)").unwrap();
    static ref PATTERN_2: Regex =
        Regex::new(r"([a-zA-Z0-9_]+)\|(\d+)\|(\d+)").unwrap();
    static ref TERM_PATTERN: Regex =
        Regex::new(r"([-+]?\s*[\d\.eE\-\+]*\s*\*?\s*[a-zA-Z_][\w\[\],]*)")
            .unwrap();
    static ref NEWLINE_PATTERN: Regex =
        Regex::new(r"(:\s)|(\s[+\-<>=]=?\s)").unwrap();
    static ref NAME_PATTERN: Regex =
        Regex::new(r"([a-zA-Z_][\w]*)\[(.*?)\]$").unwrap();
}

fn replace_indices(text: &str) -> String {
    let replaced = PATTERN_2.replace_all(text, "$1[$2,$3]");
    PATTERN_1.replace_all(&replaced, "$1[$2]").into_owned()
}

fn replace_with_line(caps: &regex::Captures, original_string: &str) -> String {
    if let Some(m) = caps.get(1) {
        // Matched :<space>
        return format!("{}\n  ", m.as_str());
    } else if let Some(m) = caps.get(2) {
        // Matched operator
        let full_match = m.as_str();
        let start = m.start();
        // Check if the 2 characters before match are 2 spaces
        if start >= 2 && &original_string[start - 2..start] == "  " {
            return full_match.to_string(); // leave it unchanged
        } else {
            return format!("\n  {}", full_match);
        }
    } else {
        // Fallback for any other match (e.g., the whole pattern if no specific group matched)
        return caps.get(0).unwrap().as_str().to_string();
    }
}

fn replace_with_lines(lines: &[String]) -> Vec<String> {
    let mut bounds_found = false;
    let mut before_bounds = Vec::new();
    let mut after_bounds = Vec::new();

    for line in lines {
        if !bounds_found && line.starts_with("Bounds") {
            bounds_found = true;
        }
        if bounds_found {
            after_bounds.push(line.clone());
        } else {
            before_bounds.push(line.clone());
        }
    }

    let before_string = before_bounds.join("\n");
    let modified_before = NEWLINE_PATTERN.replace_all(
        &before_string,
        |caps: &regex::Captures| {
            replace_with_line(caps, &before_string)
        },
    );

    let mut final_content = modified_before.into_owned();
    final_content.push_str("\n");
    final_content.push_str(&after_bounds.join("\n"));
    final_content.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum NameInd {
    Int(i32),
    Str(String),
}

fn parse_name(name: &str) -> (String, Vec<NameInd>) {
    if let Some(caps) = NAME_PATTERN.captures(name) {
        let base = caps.get(1).unwrap().as_str().to_string();
        let ind_str = caps.get(2).unwrap().as_str();
        let idx: Vec<NameInd> = ind_str
            .split(',')
            .map(|s| {
                let trimmed = s.trim();
                if let Ok(num) = trimmed.parse::<i32>() {
                    NameInd::Int(num)
                } else {
                    NameInd::Str(trimmed.to_string())
                }
            })
            .collect();
        (base, idx)
    } else {
        (name.to_string(), vec![])
    }
}

/// Custom comparison for sorting constraint/variable names with indices
fn var_compare(v1: &str, v2: &str) -> Ordering {
    let (base1, idx1) = parse_name(v1);
    let (base2, idx2) = parse_name(v2);
    if base1 != base2 {
        return base1.cmp(&base2);
    }
    idx1.cmp(&idx2)
}

/// Extract terms like coeff * var or just var from an expression
fn extract_terms(expr: &str) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let terms: Vec<&str> =
        TERM_PATTERN.find_iter(expr).map(|m| m.as_str()).collect();
    let mut parsed_terms = Vec::new();
    let mut parsed_quadratic_terms = Vec::new();
    let mut temp_term: Option<(String, String)> = None;

    for term in terms {
        let coeff_var = term.trim();
        if coeff_var.contains('*') {
            if let Some((var1, coeff)) = temp_term.take() {
                let parts: Vec<&str> = coeff_var.splitn(2, '*').collect();
                let var2 = parts[1].trim().to_string();
                let mut vars = vec![var1, var2];
                vars.sort();
                parsed_quadratic_terms
                    .push((format!("{} * {}", vars[0], vars[1]), coeff));
            }
        } else {
            let coe_var: Vec<&str> = coeff_var.rsplitn(2, ' ').collect();
            let (coeff, var) = if coe_var.len() == 1 {
                ("+".to_string(), coe_var[0].to_string())
            } else {
                let mut coeff = coe_var[1].to_string();
                let var = coe_var[0].to_string();
                if !coeff.starts_with('+') && !coeff.starts_with('-') {
                    coeff = format!("+ {}", coeff);
                }
                (coeff, var)
            };
            temp_term = Some((var.clone(), coeff.clone()));
            parsed_terms.push((var, coeff));
        }
    }
    (parsed_terms, parsed_quadratic_terms)
}

fn format_expression(terms: &[(String, String)]) -> String {
    let mut sorted_terms = terms.to_vec();
    sorted_terms.sort_by(|a, b| var_compare(&a.0, &b.0));
    sorted_terms
        .iter()
        .map(|(var, coeff)| format!("  {} {}", coeff, var))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_constraint(
    name: Option<&str>,
    lhs_terms: &[String],
    rhs: &str,
) -> Option<(String, String)> {
    if let Some(name) = name {
        if !lhs_terms.is_empty() {
            let expr = lhs_terms.join(" ");
            let (terms, _) = extract_terms(&expr);
            let sorted_expr = format_expression(&terms);
            let (parsed_name, _) = parse_name(name);
            let body = format!(" {}:\n{}\n  {}", name, sorted_expr, rhs);
            return Some((parsed_name, body));
        }
    }
    None
}

pub fn process_lp_file(input_file: &Path) -> io::Result<()> {
    let mut before_constraints = Vec::new();
    let mut parsed_constraints: Vec<(String, String)> = Vec::new();
    let mut parsed_bounds: Vec<((String, Vec<NameInd>), String)> = Vec::new();
    let mut after_constraints = Vec::new();

    let mut in_objective = 0;
    let mut in_constraints = 0;
    let mut in_bounds = 0;
    let mut in_generals = 0;
    let mut done_sections = false;

    let mut objective_expr = String::new();
    let mut constraint_name: Option<String> = None;
    let mut constraint_buffer = Vec::new();

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    let modified_lines = replace_with_lines(&lines);

    for line in &modified_lines {
        let lowercase_line = replace_indices(&line).to_lowercase();
        let stripped = lowercase_line.trim();

        // determine which section we are in
        if stripped.starts_with("maximize")
            || stripped.starts_with("minimize")
        {
            in_objective = 1;
        } else if stripped.starts_with("subject to") {
            in_constraints = 1;
        } else if stripped.starts_with("bound") {
            in_bounds = 1;
        } else if stripped.starts_with("general") {
            in_generals = 1;
        }

        if in_objective > 0 {
            if in_objective == 1 {
                in_objective += 1;
                before_constraints.push(line.clone()); // Keep title line
            } else if [
                "subject to", "bound", "integer", "binar", "general", "end"
            ]
                .iter()
                .any(|s| stripped.starts_with(s))
            {
                in_objective = 0;
                // Process objective expression
                let (terms, quadratic_terms) = extract_terms(&objective_expr);
                let sorted_expr = format_expression(&terms);
                before_constraints.push(sorted_expr);
                if !quadratic_terms.is_empty() {
                    let sorted_quadratic_expr =
                        format_expression(&quadratic_terms);
                    before_constraints.push(
                        "  + [\n".to_string()
                            + &sorted_quadratic_expr
                            + "\n  ] / 2\n",
                    );
                }
            } else {
                objective_expr.push_str(&stripped);
                objective_expr.push(' ');
            }
        }

        if in_constraints > 0 {
            if in_constraints == 1 {
                in_constraints += 1;
                before_constraints.push(line.clone()); // Keep title line
            } else if ["bound", "integer", "binar", "general", "end"]
                .iter()
                .any(|s| stripped.starts_with(s))
            {
                in_constraints = 0;
                // Process constraints
                parsed_constraints.sort_by(|a, b| a.0.cmp(&b.0));
                let sorted_constraints: Vec<String> =
                    parsed_constraints.iter().map(|c| c.1.clone()).collect();
                println!("Sorted {} constraints", sorted_constraints.len());
                // before_constraints.extend(sorted_constraints); //xxx
            } else {
                if stripped.contains(':') {
                    // Sort constraint name
                    let parts: Vec<&str> = stripped.splitn(2, ':').collect();
                    constraint_name = Some(parts[0].trim().to_string());
                    let rest = parts.get(1).map(|s| s.trim()).unwrap_or("");
                    constraint_buffer = if !rest.is_empty() {
                        vec![rest.to_string()]
                    } else {
                        Vec::new()
                    };
                    // constraint_rhs_term = None;
                } else {
                    if !stripped.contains('=') {
                        // LHS terms
                        constraint_buffer.push(stripped.to_string());
                    } else {
                        // (LHS terms +) RHS
                        let parts: Vec<&str> = stripped.splitn(2, '=').collect();
                        let term = parts[0];
                        let rhs = parts[1];
                        let rhs_term: String;
                        if term.is_empty() {
                            rhs_term = stripped.to_string();
                        } else {
                            let sign = term.chars().last().unwrap();
                            if sign == '>' || sign == '<' {
                                if term.len() > 1 {
                                    constraint_buffer.push(
                                        term[..term.len() - 1].to_string()
                                    );
                                }
                                rhs_term = format!("{}={}", sign, rhs);
                            } else {
                                rhs_term = format!("={}", rhs);
                            }
                        }
                        // Add constraint
                        if let Some(constraint) = format_constraint(
                            constraint_name.as_deref(),
                            &constraint_buffer,
                            &rhs_term,
                        ) {
                            parsed_constraints.push(constraint);
                        }
                    }
                }
            }
        }

        if in_bounds > 0 {
            if in_bounds == 1 {
                in_bounds += 1;
                after_constraints.push(line.clone()); // Keep title line
            } else if ["integer", "binar", "general", "end"]
                .iter()
                .any(|s| stripped.starts_with(s))
            {
                in_bounds = 0;
                // Process bounds
                parsed_bounds.sort_by(|a, b| a.0.cmp(&b.0));
                let sorted_bounds: Vec<String> =
                    parsed_bounds.iter().map(|c| c.1.clone()).collect();
                println!("Sorted {} bounds", sorted_bounds.len());
                after_constraints.extend(sorted_bounds);
            } else {
                let var_name: String;
                if stripped.contains("=") {
                    let var_bounds: Vec<&str> = stripped.split('=').collect();
                    if var_bounds.len() == 2 {
                        var_name = var_bounds[0].trim_end_matches('<').trim().to_string()
                    } else {
                        var_name = var_bounds[1].trim_end_matches('<').trim().to_string()
                    };
                } else {
                    // free variables like `var_name free`
                    let var_bounds: Vec<&str> = stripped.split(' ').collect();
                    var_name = var_bounds[0].trim().to_string();
                };
                parsed_bounds.push((parse_name(&var_name), lowercase_line.clone()));
            }
        }

        if in_generals > 0 {
            if in_generals == 1 {
                in_generals += 1;
                after_constraints.push(line.clone()); // Keep title line
            } else if ["integer", "binar", "general", "end"]
                .iter()
                .any(|s| stripped.starts_with(s))
            {
                in_generals = 0;
                done_sections = true;
                // Process bounds
                println!("Processed generals");
            } else {
                after_constraints.push(lowercase_line.clone());
            }
        }

        let in_sections =
            in_objective > 0 || in_constraints > 0 || in_bounds > 0 || in_generals > 0;
        if !in_sections {
            if !done_sections {
                before_constraints.push(line.clone());
            } else {
                after_constraints.push(line.clone());
            }
        }
    }

    // Write to output
    let output_file = format!("{}.sorted.lp", input_file.display());
    // don't use LineWriter, it's slow
    let mut writer = BufWriter::new(File::create(&output_file)?);
    for line in &before_constraints {
        writeln!(&mut writer, "{}", line)?;
    }
    for constraint in &parsed_constraints {
        writeln!(&mut writer, "{}", constraint.1)?;
    }
    for line in &after_constraints {
        writeln!(&mut writer, "{}", line)?;
    }

    println!("LP formatted to: {}", output_file);
    Ok(())
}

pub fn process_sol_file(input_file: &Path) -> io::Result<()> {
    let mut is_first_line = true;
    let mut first_line: Option<Vec<String>> = None;
    let mut parsed_sols: Vec<((String, Vec<NameInd>), String)> = Vec::new();

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    for line in &lines {
        let lowercase_line = replace_indices(&line.to_lowercase());
        let stripped = lowercase_line.trim();

        if is_first_line && stripped.contains("# objective value = ") {
            is_first_line = false;
            first_line = Some(vec![
                "name,value".to_string(),
                stripped
                    .replace("# objective value = ", "obj_val,")
                    .trim()
                    .to_string(),
            ]);
        } else if let Some((var_name, sol_val)) = stripped.split_once(' ') {
            parsed_sols.push((
                parse_name(var_name.trim()),
                format!("\"{}\",{}", var_name.trim(), sol_val),
            ));
        }
    }

    parsed_sols.sort_by(|a, b| a.0.cmp(&b.0));
    let sorted_sols: Vec<String> =
        parsed_sols.iter().map(|c| c.1.clone()).collect();
    println!("Sorted solution of {} variables", sorted_sols.len());

    let output_file = format!("{}.sorted.csv", input_file.display());
    let mut writer = BufWriter::new(File::create(&output_file)?);
    if let Some(fl) = first_line {
        for l in fl {
            writeln!(&mut writer, "{}", l)?;
        }
    }
    for sol in &sorted_sols {
        writeln!(&mut writer, "{}", sol)?;
    }

    println!("SOL formatted to: {}", output_file);
    Ok(())
}
