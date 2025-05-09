use super::parse::CSTNode;
use super::types::State;
use crate::lexer::lexer::Token;
use crate::utils::types::PhraseType;

pub fn look_forward(tokens: &Vec<Token>, start: usize, len: usize) -> Vec<Token> {
    if start >= tokens.len() {
        vec![]
    } else {
        let end = usize::min(start + len, tokens.len());
        tokens[start..end].to_vec()
    }
}

pub fn error_handler(state: &Vec<State>, look: &Token) {
    // 分析异常原因
    println!(
        "current state {:?} unexpected {:?}",
        state.last().unwrap(),
        look.value
    );
}

pub fn is_type_keyword(value: &String) -> bool {
    matches!(
        value.as_str(),
        "int" | "uint" | "float" | "double" | "char" | "String"
    )
}

pub fn is_control_keyword(value: &String) -> bool {
    matches!(
        value.as_str(),
        "if" | "else" | "while" | "for" | "return" | "break" | "continue"
    )
}

/// 把 token_type 映射到 action_table 的列号
pub fn term_index(tok: &Token) -> usize {
    use PhraseType::*;
    match tok.types {
        Operator if tok.value == "!" => 0,
        Operator if tok.value == "!=" => 1,
        Operator if tok.value == "&&" => 2,
        Separator if tok.value == "(" => 3,
        Separator if tok.value == ")" => 4,
        Operator if tok.value == "*" => 5,
        Operator if tok.value == "+" => 6,
        Separator if tok.value == "," => 7,
        Operator if tok.value == "-" => 8,
        Operator if tok.value == "/" => 9,
        Separator if tok.value == ";" => 10,
        Operator if tok.value == "<" => 11,
        Operator if tok.value == "<=" => 12,
        Operator if tok.value == "=" => 13,
        Operator if tok.value == "==" => 14,
        Operator if tok.value == ">" => 15,
        Operator if tok.value == ">=" => 16,
        Keyword if is_type_keyword(&tok.value) => 17,
        Separator if tok.value == "[" => 18,
        Separator if tok.value == "]" => 19,
        Keyword if tok.value == "else" => 20,
        Identifier if tok.value == "false" => 21,
        Identifier if tok.value != "true" => 22,
        Keyword if tok.value == "if" => 23,
        Float | Bool | Char | Hex | Dec | Oct => 24,
        Keyword if tok.value == "return" => 25,
        Keyword if tok.value == "struct" => 26,
        Identifier if tok.value == "true" => 27,
        Keyword if tok.value == "while" => 28,
        Separator if tok.value == "{" => 29,
        Operator if tok.value == "||" => 30,
        Separator if tok.value == "}" => 31,
        _ => 32, // end of file
    }
}

impl CSTNode {
    fn name(&self) -> &str {
        match self {
            CSTNode::Assign { .. } => "Assign",
            CSTNode::AssignPrime { .. } => "AssignPrime",
            CSTNode::UnMatchedStmt { .. } => "UnMatchedStmt",
            CSTNode::StructSpecifier { .. } => "StructSpecifier",
            CSTNode::Specifier { .. } => "Specifier",
            CSTNode::ParaDec { .. } => "ParaDec",
            CSTNode::MatchedStmt { .. } => "MatchedStmt",
            CSTNode::StmtList { .. } => "StmtList",
            CSTNode::Stmt { .. } => "Stmt",
            CSTNode::NormalStmt { .. } => "NormalStmt",
            CSTNode::Program(_) => "Program",
            CSTNode::Expression(_) => "Expression",
            CSTNode::FunCall { .. } => "FunCall",
            CSTNode::FunDec { .. } => "FunDec",
            CSTNode::VarList { .. } => "VarList",
            CSTNode::VarDec { .. } => "VarDec",
            CSTNode::LogicalOrPrime { .. } => "LogicalOrPrime",
            CSTNode::LogicalOr { .. } => "LogicalOr",
            CSTNode::LogicalAnd { .. } => "LogicalAnd",
            CSTNode::LogicalAndPrime { .. } => "LogicalAndPrime",
            CSTNode::ExtDefList { .. } => "ExtDefList",
            CSTNode::ExtDef { .. } => "ExtDef",
            CSTNode::ExtDecList { .. } => "ExtDecList",
            CSTNode::DefList { .. } => "DefList",
            CSTNode::Def { .. } => "Def",
            CSTNode::DecList { .. } => "DecList",
            CSTNode::Dec { .. } => "Dec",
            CSTNode::CompSt { .. } => "CompSt",
            CSTNode::Arguments { .. } => "Arguments",
            CSTNode::ArgumentsTail { .. } => "ArgumentsTail",
            CSTNode::Equality { .. } => "Equality",
            CSTNode::EqualityPrime { .. } => "EqualityPrime",
            CSTNode::Comparison { .. } => "Comparison",
            CSTNode::ComparisonPrime { .. } => "ComparisonPrime",
            CSTNode::Term { .. } => "Term",
            CSTNode::TermPrime { .. } => "TermPrime",
            CSTNode::Factor { .. } => "Factor",
            CSTNode::FactorPrime { .. } => "FactorPrime",
            CSTNode::Unary { .. } => "Unary",
            CSTNode::Primary { .. } => "Primary",
        }
    }
    pub fn print_tree(&self) {
        self.traverse_tree(0, true);
    }

    /// 深度优先、前序遍历并打印，depth 是当前深度，is_last 表示是否父节点最后一个孩子
    fn traverse_tree(&self, depth: usize, is_last: bool) {
        // 构造前缀
        let mut prefix = String::new();
        if depth > 0 {
            for _ in 0..depth - 1 {
                prefix.push_str("│   ");
            }
            prefix.push_str(if is_last { "└── " } else { "├── " });
        }
        // 打印当前节点（Debug 格式）
        let name = self.name();
        println!("{}{}", prefix, name);

        // 取所有子节点
        let children = self.children();
        let count = children.len();
        // 递归遍历子节点
        for (i, child) in children.into_iter().enumerate() {
            child.traverse_tree(depth + 1, i + 1 == count);
        }
    }

    fn children(&self) -> Vec<&CSTNode> {
        let mut v: Vec<&CSTNode> = Vec::new();
        match self {
            CSTNode::Assign {
                logical_or,
                assign_prime,
            } => {
                v.push(logical_or);
                if let Some(ap) = assign_prime {
                    v.push(ap);
                }
            }
            CSTNode::AssignPrime {
                logical_or,
                assign_prime,
                ..
            } => {
                v.push(logical_or);
                if let Some(ap) = assign_prime {
                    v.push(ap);
                }
            }
            CSTNode::UnMatchedStmt {
                expression,
                matched_stmt,
                unmatched_stmt,
                stmt,
                ..
            } => {
                if let Some(e) = expression {
                    v.push(e);
                }
                if let Some(ms) = matched_stmt {
                    v.push(ms);
                }
                if let Some(ums) = unmatched_stmt {
                    v.push(ums);
                }
                if let Some(s) = stmt {
                    v.push(s);
                }
            }
            CSTNode::StructSpecifier { def_list, .. } => {
                if let Some(d) = def_list {
                    v.push(d);
                }
            }
            CSTNode::Specifier {
                struct_specifier, ..
            } => {
                if let Some(s) = struct_specifier {
                    v.push(s);
                }
            }
            CSTNode::ParaDec { spec, var_dec } => {
                v.push(spec);
                v.push(var_dec);
            }
            CSTNode::MatchedStmt {
                normal_stmt,
                expression,
                matched_stmt_fore,
                matched_stmt_back,
                ..
            } => {
                if let Some(n) = normal_stmt {
                    v.push(n);
                }
                if let Some(e) = expression {
                    v.push(e);
                }
                if let Some(f) = matched_stmt_fore {
                    v.push(f);
                }
                if let Some(b) = matched_stmt_back {
                    v.push(b);
                }
            }
            CSTNode::StmtList { stmt, stmt_list } => {
                v.push(stmt);
                if let Some(sl) = stmt_list {
                    v.push(sl);
                }
            }
            CSTNode::Stmt {
                unmatched_stmt,
                matched_stmt,
            } => {
                if let Some(us) = unmatched_stmt {
                    v.push(us);
                }
                if let Some(ms) = matched_stmt {
                    v.push(ms);
                }
            }
            CSTNode::NormalStmt {
                expression, compst, ..
            } => {
                if let Some(e) = expression {
                    v.push(e);
                }
                if let Some(c) = compst {
                    v.push(c);
                }
            }
            CSTNode::Program(node) | CSTNode::Expression(node) => {
                v.push(node);
            }
            CSTNode::FunCall { arguments, .. }
            | CSTNode::FunDec {
                var_list: arguments,
                ..
            } => {
                if let Some(a) = arguments {
                    v.push(a);
                }
            }
            CSTNode::VarList {
                para_dec, var_list, ..
            }
            | CSTNode::ExtDecList {
                var_dec: para_dec,
                ext_dec_list: var_list,
                ..
            } => {
                v.push(para_dec);
                if let Some(rest) = var_list {
                    v.push(rest);
                }
            }
            CSTNode::VarDec { var_dec, .. } => {
                if let Some(vd) = var_dec {
                    v.push(vd);
                }
            }
            CSTNode::LogicalOr {
                logical_and,
                logical_or_prime,
            }
            | CSTNode::LogicalAnd {
                equality: logical_and,
                logical_and_prime: logical_or_prime,
            } => {
                v.push(logical_and);
                if let Some(p) = logical_or_prime {
                    v.push(p);
                }
            }
            CSTNode::LogicalOrPrime {
                logical_and,
                logical_or_prime,
                ..
            }
            | CSTNode::LogicalAndPrime {
                equality: logical_and,
                logical_and_prime: logical_or_prime,
                ..
            } => {
                v.push(logical_and);
                if let Some(p) = logical_or_prime {
                    v.push(p);
                }
            }
            CSTNode::ExtDefList {
                ext_def,
                ext_def_list,
            } => {
                v.push(ext_def);
                if let Some(edl) = ext_def_list {
                    v.push(edl);
                }
            }
            CSTNode::ExtDef {
                spec,
                ext_dec_list,
                fun_dec,
                compst,
                ..
            } => {
                v.push(spec);
                if let Some(xdl) = ext_dec_list {
                    v.push(xdl);
                }
                if let Some(fd) = fun_dec {
                    v.push(fd);
                }
                if let Some(c) = compst {
                    v.push(c);
                }
            }
            CSTNode::DefList { def, def_list } => {
                v.push(def);
                if let Some(dl) = def_list {
                    v.push(dl);
                }
            }
            CSTNode::Def { spec, dec_list, .. } => {
                v.push(spec);
                v.push(dec_list);
            }
            CSTNode::DecList { dec, dec_list, .. } => {
                v.push(dec);
                if let Some(dl) = dec_list {
                    v.push(dl);
                }
            }
            CSTNode::Dec {
                var_dec,
                expression,
                ..
            } => {
                v.push(var_dec);
                if let Some(e) = expression {
                    v.push(e);
                }
            }
            CSTNode::CompSt {
                def_list,
                stmt_list,
                ..
            } => {
                if let Some(dl) = def_list {
                    v.push(dl);
                }
                v.push(stmt_list);
            }
            CSTNode::Arguments {
                arguments_tail,
                expression,
            } => {
                v.push(expression);
                if let Some(at) = arguments_tail {
                    v.push(at);
                }
            }
            CSTNode::ArgumentsTail { expression, .. } => {
                v.push(expression);
            }
            CSTNode::Equality {
                comparison,
                equality_prime,
            }
            | CSTNode::Comparison {
                term: comparison,
                comparison_prime: equality_prime,
            } => {
                v.push(comparison);
                if let Some(p) = equality_prime {
                    v.push(p);
                }
            }
            CSTNode::EqualityPrime {
                comparison,
                equality_prime,
                ..
            }
            | CSTNode::ComparisonPrime {
                term: comparison,
                comparison_prime: equality_prime,
                ..
            } => {
                v.push(comparison);
                if let Some(p) = equality_prime {
                    v.push(p);
                }
            }
            CSTNode::Term { factor, term_prime }
            | CSTNode::Factor {
                unary: factor,
                factor_prime: term_prime,
            } => {
                v.push(factor);
                if let Some(p) = term_prime {
                    v.push(p);
                }
            }
            CSTNode::TermPrime {
                factor, term_prime, ..
            }
            | CSTNode::FactorPrime {
                unary: factor,
                factor_prime: term_prime,
                ..
            } => {
                v.push(factor);
                if let Some(p) = term_prime {
                    v.push(p);
                }
            }
            CSTNode::Unary { unary, .. } => {
                v.push(unary);
            }
            CSTNode::Primary {
                expression,
                fun_call,
                ..
            } => {
                if let Some(e) = expression {
                    v.push(e);
                }
                if let Some(fc) = fun_call {
                    v.push(fc);
                }
            }
        }
        v
    }
}
