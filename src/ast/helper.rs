use super::{SymbolInfo, SymbolKey};
use crate::ast::DEFAULT_OFFSET;
use crate::ast::types::{ASTNode, Type, VarDec};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

static SYMBOL_TABLE: Lazy<Mutex<HashMap<SymbolKey, SymbolInfo>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FUNC_SYMBOL_TABLE: Lazy<Mutex<HashMap<String, HashMap<SymbolKey, SymbolInfo>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static GLOBAL_SCOPE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static CURRENT_FUNC: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

impl ASTNode {
    pub fn print_tree(&self, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        match self {
            ASTNode::Specifier(ty) => {
                format!("{}Specifier: {}", indent_str, Self::print_type(ty))
            }
            ASTNode::Program { items } => {
                let items_str: Vec<String> = items
                    .iter()
                    .map(|item| item.print_tree(indent + 1))
                    .collect();
                format!("{}Program:\n{}", indent_str, items_str.join("\n"))
            }
            ASTNode::FuncDef {
                name,
                params,
                ret_type,
                body,
            } => {
                let params_str: Vec<String> = params
                    .iter()
                    .map(|param| format!("{}  {}", indent_str, Self::print_vardec(param)))
                    .collect();
                format!(
                    "{}FuncDef: {}\n{}  ReturnType: {}\n{}  Parameters:\n{}\n{}  Body:\n{}",
                    indent_str,
                    name,
                    indent_str,
                    Self::print_type(ret_type),
                    indent_str,
                    params_str.join("\n"),
                    indent_str,
                    body.print_tree(indent + 1)
                )
            }
            ASTNode::VarDecl(vars) => {
                let vars_str: Vec<String> = vars
                    .iter()
                    .map(|var| format!("{}  {}", indent_str, Self::print_vardec(var)))
                    .collect();
                format!("{}VarDecl:\n{}", indent_str, vars_str.join("\n"))
            }
            ASTNode::Block { stmts } => {
                let stmts_str: Vec<String> = stmts
                    .iter()
                    .map(|stmt| stmt.print_tree(indent + 1))
                    .collect();
                format!("{}Block:\n{}", indent_str, stmts_str.join("\n"))
            }
            ASTNode::If {
                cond,
                then_br,
                else_br,
            } => {
                let else_str = match else_br {
                    Some(else_branch) => format!(
                        "\n{}  Else:\n{}",
                        indent_str,
                        else_branch.print_tree(indent + 2)
                    ),
                    None => String::new(),
                };
                format!(
                    "{}If:\n{}  Condition:\n{}\n{}  Then:\n{}{}",
                    indent_str,
                    indent_str,
                    cond.print_tree(indent + 2),
                    indent_str,
                    then_br.print_tree(indent + 2),
                    else_str
                )
            }
            ASTNode::While { cond, body } => {
                format!(
                    "{}While:\n{}  Condition:\n{}\n{}  Body:\n{}",
                    indent_str,
                    indent_str,
                    cond.print_tree(indent + 2),
                    indent_str,
                    body.print_tree(indent + 2)
                )
            }
            ASTNode::Return { expr } => match expr {
                Some(e) => format!("{}Return:\n{}", indent_str, e.print_tree(indent + 1)),
                None => format!("{}Return: None", indent_str),
            },
            ASTNode::BinaryOp { op, lhs, rhs } => {
                let op_str = op.as_ref().map_or("None".to_string(), |s| s.clone());
                let rhs_str = match rhs {
                    Some(r) => r.print_tree(indent + 2),
                    None => format!("{}  None", indent_str),
                };
                format!(
                    "{}BinaryOp: {}\n{}  Left:\n{}\n{}  Right:\n{}",
                    indent_str,
                    op_str,
                    indent_str,
                    lhs.print_tree(indent + 2),
                    indent_str,
                    rhs_str
                )
            }
            ASTNode::UnaryOp { op, expr } => {
                format!(
                    "{}UnaryOp: {}\n{}",
                    indent_str,
                    op,
                    expr.print_tree(indent + 1)
                )
            }
            ASTNode::Call { func, args } => {
                let args_str: Vec<String> =
                    args.iter().map(|arg| arg.print_tree(indent + 2)).collect();
                format!(
                    "{}Call: {}\n{}  Arguments:\n{}",
                    indent_str,
                    func,
                    indent_str,
                    args_str.join("\n")
                )
            }
            ASTNode::Literal(value) => {
                format!("{}Literal: {:?}", indent_str, value)
            }
            ASTNode::Ident(name) => {
                format!("{}Ident: {:?}", indent_str, name)
            }
        }
    }

    fn print_type(ty: &Type) -> String {
        match ty {
            Type::BaseType(name) => name.clone(),
            Type::CustomizedType(st) => {
                let fields: Vec<String> = st.field.iter().map(|f| Self::print_vardec(f)).collect();
                format!("struct {} {{\n  {}\n}}", st.id, fields.join("\n  "))
            }
        }
    }

    fn print_vardec(var: &VarDec) -> String {
        let init_str = match &var.init {
            Some(init) => format!(" = {}", init.print_tree(0).trim_start()),
            None => String::new(),
        };
        format!(
            "{}: {}{}",
            var.var_name,
            Self::print_type(&var.var_type),
            init_str
        )
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_tree(0))
    }
}

pub fn build_symbol_table(
    func_name: String,
    symbol: &VarDec,
    scope: usize,
    params: Option<Vec<VarDec>>,
) {
    let key: SymbolKey = (symbol.var_name.clone(), scope);
    let init_ = symbol.init.as_ref().map(|e| e.clone().optimal());
    let var_dec = VarDec {
        var_type: symbol.var_type.clone(),
        var_name: symbol.var_name.clone(),
        init: init_,
    };
    let value: SymbolInfo = (var_dec, scope, params.clone(), DEFAULT_OFFSET);
    if func_name.is_empty() || params.is_some() {
        let sym = look_up_symbol_table(symbol.var_name.clone(), scope);
        if sym.is_some() && sym.unwrap().1 == scope {
            println!("redefine {:?}", symbol);
            unreachable!();
        }
        let mut table = SYMBOL_TABLE.lock().unwrap();
        table.insert(key, value);
    } else {
        if look_up_func_symbol_table(symbol.var_name.clone(), scope, func_name.clone()).is_some() {
            println!("redefine {:?}", symbol);
            unreachable!();
        }
        let mut func_symbol_table = FUNC_SYMBOL_TABLE.lock().unwrap();
        let symbol_table = func_symbol_table
            .entry(func_name.clone())
            .or_insert_with(|| HashMap::new());
        symbol_table.insert(key, value);
    }
}
pub fn look_up_func_symbol_table(
    var_name: String,
    scope: usize,
    func_name: String,
) -> Option<SymbolInfo> {
    let func_symbol_table = FUNC_SYMBOL_TABLE.lock().unwrap();
    let symbol_table = func_symbol_table.get(&func_name);
    if let Some(vlaue) = symbol_table {
        let key = (var_name, scope);
        vlaue.get(&key).cloned()
    } else {
        None
    }
}
pub fn look_up_symbol_table(var_name: String, scope: usize) -> Option<SymbolInfo> {
    let table = SYMBOL_TABLE.lock().unwrap();
    let key: SymbolKey = (var_name, scope);
    table.get(&key).cloned()
}

pub fn travel_symbol_table(
    var_name: &String,
    scope: usize,
    func_name: String,
) -> Option<SymbolInfo> {
    let result: Option<SymbolInfo> = SYMBOL_TABLE
        .lock()
        .unwrap()
        .iter()
        .filter(|((v, s), _)| v.eq(var_name) && *s <= scope)
        .max_by_key(|((_, s), _)| *s)
        .map(|(_, v)| v.clone());
    if result.is_some() {
        return result;
    }
    let mut i = scope;
    loop {
        let ret = look_up_func_symbol_table(var_name.clone(), i, func_name.clone());
        if i == 0 || ret.is_some() {
            return ret;
        }
        i -= 1;
    }
}

pub fn get_current_scope_num() -> usize {
    let scope = GLOBAL_SCOPE.lock().unwrap();
    *scope
}

pub fn update_current_scope_num(next: usize) {
    let mut scope = GLOBAL_SCOPE.lock().unwrap();
    *scope = next;
}

pub fn set_current_func(func_name: String) {
    let mut name = CURRENT_FUNC.lock().unwrap();
    *name = func_name;
}
pub fn get_current_func() -> String {
    let name = CURRENT_FUNC.lock().unwrap();
    name.clone()
}

pub fn check_types(lhs: &ASTNode, rhs: &ASTNode) -> bool {
    let lhs_type = ASTNode::get_ast_type(lhs);
    let rhs_type = ASTNode::get_ast_type(rhs);
    if lhs_type != rhs_type {
        println!("lhs'type {:?}, rhs'type {:?}", lhs_type, rhs_type);
        println!(
            "lhs->{:?} rhs->{:?}",
            lhs.clone().optimal(),
            rhs.clone().optimal()
        );
        print_symbol_table();
        false
    } else {
        true
    }
}

pub fn print_symbol_table() {
    let table = SYMBOL_TABLE.lock().unwrap();
    println!("global");
    for (key, value) in table.iter() {
        println!("{:?} {:?}", key, value);
    }
    let func_table = FUNC_SYMBOL_TABLE.lock().unwrap();
    for symbol_table in func_table.iter() {
        println!("{}", symbol_table.0);
        for (key, value) in symbol_table.1 {
            println!("{:?} {:?}", key, value);
        }
    }
}

pub fn check_entry_func() -> bool {
    let table = SYMBOL_TABLE.lock().unwrap();
    let key: SymbolKey = (String::from("main"), 0);
    table.get(&key).cloned().is_some()
}
