use super::helper::error_handler;
use super::types::{Action, NonTerm, State};
use crate::lexer::lexer::Token;
use crate::utils::types::PhraseType;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum CSTNode {
    Assign {
        logical_or: Box<CSTNode>,
        assign_prime: Option<Box<CSTNode>>,
    },
    AssignPrime {
        op: String,
        logical_or: Box<CSTNode>,
        assign_prime: Option<Box<CSTNode>>,
    },
    UnMatchedStmt {
        while_stmt: Option<String>,
        if_stmt: Option<String>,
        else_stmt: Option<String>,
        lp: Option<String>,
        rp: Option<String>,
        expression: Option<Box<CSTNode>>,
        matched_stmt: Option<Box<CSTNode>>,
        unmatched_stmt: Option<Box<CSTNode>>,
        stmt: Option<Box<CSTNode>>,
    },
    StructSpecifier {
        struct_type: String,
        id: Option<String>,
        lc: Option<String>,
        rc: Option<String>,
        def_list: Option<Box<CSTNode>>,
    },
    Specifier {
        specifier_type: Option<String>,
        struct_specifier: Option<Box<CSTNode>>,
    },
    ParaDec {
        spec: Box<CSTNode>,
        var_dec: Box<CSTNode>,
    },
    MatchedStmt {
        normal_stmt: Option<Box<CSTNode>>,
        while_stmt: Option<String>,
        if_stmt: Option<String>,
        else_stmt: Option<String>,
        lp: Option<String>,
        rp: Option<String>,
        expression: Option<Box<CSTNode>>,
        matched_stmt_fore: Option<Box<CSTNode>>,
        matched_stmt_back: Option<Box<CSTNode>>,
    },
    StmtList {
        stmt: Box<CSTNode>,
        stmt_list: Option<Box<CSTNode>>,
    },
    Stmt {
        unmatched_stmt: Option<Box<CSTNode>>,
        matched_stmt: Option<Box<CSTNode>>,
    },
    NormalStmt {
        sepa: Option<String>,
        expression: Option<Box<CSTNode>>,
        compst: Option<Box<CSTNode>>,
        return_stmt: Option<String>,
    },
    Program(Box<CSTNode>),
    Expression(Box<CSTNode>),
    FunCall {
        id: String,
        lp: String,
        arguments: Option<Box<CSTNode>>,
        rp: String,
    },
    FunDec {
        id: String,
        lp: String,
        var_list: Option<Box<CSTNode>>,
        rp: String,
    },
    VarList {
        para_dec: Box<CSTNode>,
        sepa: Option<String>,
        var_list: Option<Box<CSTNode>>,
    },
    VarDec {
        id: Option<String>,
        var_dec: Option<Box<CSTNode>>,
        lt: Option<String>,
        rt: Option<String>,
        literal: Option<String>,
    },
    LogicalOrPrime {
        op: String,
        logical_and: Box<CSTNode>,
        logical_or_prime: Option<Box<CSTNode>>,
    },
    LogicalOr {
        logical_and: Box<CSTNode>,
        logical_or_prime: Option<Box<CSTNode>>,
    },
    LogicalAnd {
        equality: Box<CSTNode>,
        logical_and_prime: Option<Box<CSTNode>>,
    },
    LogicalAndPrime {
        op: String,
        equality: Box<CSTNode>,
        logical_and_prime: Option<Box<CSTNode>>,
    },
    ExtDefList {
        ext_def: Box<CSTNode>,
        ext_def_list: Option<Box<CSTNode>>,
    },
    ExtDef {
        spec: Box<CSTNode>,
        ext_dec_list: Option<Box<CSTNode>>,
        fun_dec: Option<Box<CSTNode>>,
        compst: Option<Box<CSTNode>>,
        sepa: Option<String>,
    },
    ExtDecList {
        var_dec: Box<CSTNode>,
        sepa: Option<String>,
        ext_dec_list: Option<Box<CSTNode>>,
    },
    DefList {
        def: Box<CSTNode>,
        def_list: Option<Box<CSTNode>>,
    },
    Def {
        spec: Box<CSTNode>,
        dec_list: Box<CSTNode>,
        sepa: String,
    },
    DecList {
        dec: Box<CSTNode>,
        sepa: Option<String>,
        dec_list: Option<Box<CSTNode>>,
    },
    Dec {
        var_dec: Box<CSTNode>,
        op: Option<String>,
        expression: Option<Box<CSTNode>>,
    },
    CompSt {
        lc: String,
        def_list: Option<Box<CSTNode>>,
        stmt_list: Box<CSTNode>,
        rc: String,
    },
    Arguments {
        arguments_tail: Option<Box<CSTNode>>,
        expression: Box<CSTNode>,
    },
    ArgumentsTail {
        separator: String,
        expression: Box<CSTNode>,
    },
    Equality {
        comparison: Box<CSTNode>,
        equality_prime: Option<Box<CSTNode>>,
    },
    EqualityPrime {
        op: String,
        comparison: Box<CSTNode>,
        equality_prime: Option<Box<CSTNode>>,
    },
    ComparisonPrime {
        op: String,
        term: Box<CSTNode>,
        comparison_prime: Option<Box<CSTNode>>,
    },
    Comparison {
        term: Box<CSTNode>,
        comparison_prime: Option<Box<CSTNode>>,
    },
    Term {
        factor: Box<CSTNode>,
        term_prime: Option<Box<CSTNode>>,
    },
    TermPrime {
        op: String,
        factor: Box<CSTNode>,
        term_prime: Option<Box<CSTNode>>,
    },
    Factor {
        unary: Box<CSTNode>,
        factor_prime: Option<Box<CSTNode>>,
    },
    FactorPrime {
        op: String,
        unary: Box<CSTNode>,
        factor_prime: Option<Box<CSTNode>>,
    },
    Unary {
        op: Option<String>,
        unary: Box<CSTNode>,
    },
    Primary {
        symbol: Option<Token>,
        lp: Option<String>,
        rp: Option<String>,
        expression: Option<Box<CSTNode>>,
        fun_call: Option<Box<CSTNode>>,
    },
}

pub fn parse(tokens: &Vec<Token>) -> Vec<CSTNode> {
    let mut index: usize = 0;
    let mut state: Vec<State> = vec![State::S0];
    let mut sym: Vec<Token> = vec![];
    let mut cst: Vec<CSTNode> = vec![];
    loop {
        let look = tokens.get(index);
        let col = if let Some(tok) = look {
            super::helper::term_index(tok)
        } else {
            32
        };
        let st = state.last().unwrap().to_index();
        let action = &super::constant::ACTION[st][col];
        let token = tokens.get(index).cloned().unwrap_or(Token {
            pos: 0,
            types: PhraseType::Separator,
            value: String::from("$"),
        });
        match action {
            Action::Shift(ns) => {
                let symbol = token.clone();
                // println!(
                //     "shift push {:?} {:?} {:?}",
                //     state.last().unwrap(),
                //     *ns,
                //     look
                // );
                state.push(*ns);
                sym.push(symbol);
                index += 1;
            }
            Action::Reduce(rule) => {
                let (lhs, rhs_len) = match rule {
                    0 => (NonTerm::Accept, 0),
                    1 => {
                        let arguments_tail = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::Arguments {
                            arguments_tail: Some(Box::new(arguments_tail)),
                            expression: Box::new(expression),
                        };
                        cst.push(node);
                        (NonTerm::Arguments, 2)
                    }
                    2 => {
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::Arguments {
                            arguments_tail: None,
                            expression: Box::new(expression),
                        };
                        cst.push(node);
                        (NonTerm::Arguments, 1)
                    }
                    3 => {
                        let sepa = sym.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::ArgumentsTail {
                            separator: sepa.value,
                            expression: Box::new(expression),
                        };
                        cst.push(node);
                        (NonTerm::ArgumentsTail, 2)
                    }
                    4 => {
                        let rc = sym.pop().unwrap();
                        let lc = sym.pop().unwrap();
                        let stmt_list = cst.pop().unwrap();
                        let def_list = cst.pop().unwrap();
                        let node = CSTNode::CompSt {
                            lc: lc.value,
                            def_list: Some(Box::new(def_list)),
                            stmt_list: Box::new(stmt_list),
                            rc: rc.value,
                        };
                        cst.push(node);
                        (NonTerm::CompSt, 4)
                    }
                    5 => {
                        let rc = sym.pop().unwrap();
                        let lc = sym.pop().unwrap();
                        let stmt_list = cst.pop().unwrap();
                        let node = CSTNode::CompSt {
                            lc: lc.value,
                            def_list: None,
                            stmt_list: Box::new(stmt_list),
                            rc: rc.value,
                        };
                        cst.push(node);
                        (NonTerm::CompSt, 3)
                    }
                    6 => {
                        let var_dec = cst.pop().unwrap();
                        let node = CSTNode::Dec {
                            var_dec: Box::new(var_dec),
                            op: None,
                            expression: None,
                        };
                        cst.push(node);
                        (NonTerm::Dec, 1)
                    }
                    7 => {
                        let expr = cst.pop().unwrap();
                        let var_dec = cst.pop().unwrap();
                        let op = sym.pop().unwrap();
                        let node = CSTNode::Dec {
                            var_dec: Box::new(var_dec),
                            op: Some(op.value),
                            expression: Some(Box::new(expr)),
                        };
                        cst.push(node);
                        (NonTerm::Dec, 3)
                    }
                    8 => {
                        let dec = cst.pop().unwrap();
                        let node = CSTNode::DecList {
                            dec: Box::new(dec),
                            sepa: None,
                            dec_list: None,
                        };
                        cst.push(node);
                        (NonTerm::DecList, 1)
                    }
                    9 => {
                        let dec_list = cst.pop().unwrap();
                        let dec = cst.pop().unwrap();
                        let sepa = sym.pop().unwrap();
                        let node = CSTNode::DecList {
                            dec: Box::new(dec),
                            sepa: Some(sepa.value),
                            dec_list: Some(Box::new(dec_list)),
                        };
                        cst.push(node);
                        (NonTerm::DecList, 3)
                    }
                    10 => {
                        let sepa = sym.pop().unwrap();
                        let dec_list = cst.pop().unwrap();
                        let spec = cst.pop().unwrap();
                        let node = CSTNode::Def {
                            spec: Box::new(spec),
                            dec_list: Box::new(dec_list),
                            sepa: sepa.value,
                        };
                        cst.push(node);
                        (NonTerm::Def, 3)
                    }
                    11 => {
                        let def_list = cst.pop().unwrap();
                        let def = cst.pop().unwrap();
                        let node = CSTNode::DefList {
                            def: Box::new(def),
                            def_list: Some(Box::new(def_list)),
                        };
                        cst.push(node);
                        (NonTerm::DefList, 2)
                    }
                    12 => {
                        let def = cst.pop().unwrap();
                        let node = CSTNode::DefList {
                            def: Box::new(def),
                            def_list: None,
                        };
                        cst.push(node);
                        (NonTerm::DefList, 1)
                    }
                    13 => {
                        let var_dec = cst.pop().unwrap();
                        let node = CSTNode::ExtDecList {
                            var_dec: Box::new(var_dec),
                            sepa: None,
                            ext_dec_list: None,
                        };
                        cst.push(node);
                        (NonTerm::ExtDecList, 1)
                    }
                    14 => {
                        let ext_dec_list = cst.pop().unwrap();
                        let var_dec = cst.pop().unwrap();
                        let sepa = sym.pop().unwrap();
                        let node = CSTNode::ExtDecList {
                            var_dec: Box::new(var_dec),
                            sepa: Some(sepa.value),
                            ext_dec_list: Some(Box::new(ext_dec_list)),
                        };
                        cst.push(node);
                        (NonTerm::ExtDecList, 3)
                    }
                    15 => {
                        let ext_dec_list = cst.pop().unwrap();
                        let spec = cst.pop().unwrap();
                        let sepa = sym.pop().unwrap();
                        let node = CSTNode::ExtDef {
                            spec: Box::new(spec),
                            ext_dec_list: Some(Box::new(ext_dec_list)),
                            fun_dec: None,
                            compst: None,
                            sepa: Some(sepa.value),
                        };
                        cst.push(node);
                        (NonTerm::ExtDef, 3)
                    }
                    16 => {
                        let spec = cst.pop().unwrap();
                        let sepa = sym.pop().unwrap();
                        let node = CSTNode::ExtDef {
                            spec: Box::new(spec),
                            ext_dec_list: None,
                            fun_dec: None,
                            compst: None,
                            sepa: Some(sepa.value),
                        };
                        cst.push(node);
                        (NonTerm::ExtDef, 2)
                    }
                    17 => {
                        let compst = cst.pop().unwrap();
                        let fun_dec = cst.pop().unwrap();
                        let spec = cst.pop().unwrap();
                        let node = CSTNode::ExtDef {
                            spec: Box::new(spec),
                            ext_dec_list: None,
                            fun_dec: Some(Box::new(fun_dec)),
                            compst: Some(Box::new(compst)),
                            sepa: None,
                        };
                        cst.push(node);
                        (NonTerm::ExtDef, 3)
                    }
                    18 => {
                        let ext_def_list = cst.pop().unwrap();
                        let ext_def = cst.pop().unwrap();
                        let node = CSTNode::ExtDefList {
                            ext_def: Box::new(ext_def),
                            ext_def_list: Some(Box::new(ext_def_list)),
                        };
                        cst.push(node);
                        (NonTerm::ExtDefList, 2)
                    }
                    19 => {
                        let ext_def = cst.pop().unwrap();
                        let node = CSTNode::ExtDefList {
                            ext_def: Box::new(ext_def),
                            ext_def_list: None,
                        };
                        cst.push(node);
                        (NonTerm::ExtDefList, 1)
                    }
                    20 => {
                        let arguments = cst.pop().unwrap();
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let id = sym.pop().unwrap();
                        let node = CSTNode::FunCall {
                            id: id.value,
                            lp: lp.value,
                            arguments: Some(Box::new(arguments)),
                            rp: rp.value,
                        };
                        cst.push(node);
                        (NonTerm::FunCall, 4)
                    }
                    21 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let id = sym.pop().unwrap();
                        let node = CSTNode::FunCall {
                            id: id.value,
                            lp: lp.value,
                            arguments: None,
                            rp: rp.value,
                        };
                        cst.push(node);
                        (NonTerm::FunCall, 3)
                    }
                    22 => {
                        let var_list = cst.pop().unwrap();
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let id = sym.pop().unwrap();
                        let node = CSTNode::FunDec {
                            id: id.value,
                            lp: lp.value,
                            var_list: Some(Box::new(var_list)),
                            rp: rp.value,
                        };
                        cst.push(node);
                        (NonTerm::FunDec, 4)
                    }
                    23 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let id = sym.pop().unwrap();
                        let node = CSTNode::FunDec {
                            id: id.value,
                            lp: lp.value,
                            var_list: None,
                            rp: rp.value,
                        };
                        cst.push(node);
                        (NonTerm::FunDec, 3)
                    }
                    24 => {
                        let normal_stmt = cst.pop().unwrap();
                        let node = CSTNode::MatchedStmt {
                            normal_stmt: Some(Box::new(normal_stmt)),
                            while_stmt: None,
                            if_stmt: None,
                            else_stmt: None,
                            expression: None,
                            matched_stmt_fore: None,
                            matched_stmt_back: None,
                            lp: None,
                            rp: None,
                        };
                        cst.push(node);
                        (NonTerm::MatchedStmt, 1)
                    }
                    25 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let while_stmt = sym.pop().unwrap();
                        let matched_stmt = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::MatchedStmt {
                            normal_stmt: None,
                            while_stmt: Some(while_stmt.value),
                            lp: Some(lp.value),
                            rp: Some(rp.value),
                            expression: Some(Box::new(expression)),
                            matched_stmt_fore: Some(Box::new(matched_stmt)),
                            matched_stmt_back: None,
                            if_stmt: None,
                            else_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::MatchedStmt, 5)
                    }
                    26 => {
                        let else_stmt = sym.pop().unwrap();
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let if_stmt = sym.pop().unwrap();
                        let matched_stmt_back = cst.pop().unwrap();
                        let matched_stmt_fore = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::MatchedStmt {
                            normal_stmt: None,
                            while_stmt: None,
                            lp: Some(lp.value),
                            rp: Some(rp.value),
                            expression: Some(Box::new(expression)),
                            matched_stmt_fore: Some(Box::new(matched_stmt_fore)),
                            matched_stmt_back: Some(Box::new(matched_stmt_back)),
                            if_stmt: Some(if_stmt.value),
                            else_stmt: Some(else_stmt.value),
                        };
                        cst.push(node);
                        (NonTerm::MatchedStmt, 7)
                    }
                    27 => {
                        let sepa = sym.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::NormalStmt {
                            expression: Some(Box::new(expression)),
                            sepa: Some(sepa.value),
                            compst: None,
                            return_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::NormalStmt, 2)
                    }
                    28 => {
                        let compst = cst.pop().unwrap();
                        let node = CSTNode::NormalStmt {
                            compst: Some(Box::new(compst)),
                            return_stmt: None,
                            sepa: None,
                            expression: None,
                        };
                        cst.push(node);
                        (NonTerm::NormalStmt, 1)
                    }
                    29 => {
                        let sepa = sym.pop().unwrap();
                        let return_stmt = sym.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::NormalStmt {
                            return_stmt: Some(return_stmt.value),
                            expression: Some(Box::new(expression)),
                            sepa: Some(sepa.value),
                            compst: None,
                        };
                        cst.push(node);
                        (NonTerm::NormalStmt, 3)
                    }
                    30 => {
                        let var_dec = cst.pop().unwrap();
                        let spec = cst.pop().unwrap();
                        let node = CSTNode::ParaDec {
                            var_dec: Box::new(var_dec),
                            spec: Box::new(spec),
                        };
                        cst.push(node);
                        (NonTerm::ParaDec, 2)
                    }
                    31 => {
                        let ext_def_list = cst.pop().unwrap();
                        let node = CSTNode::Program(Box::new(ext_def_list));
                        cst.push(node);
                        (NonTerm::Program, 1)
                    }
                    32 => {
                        let specifier_type = sym.pop().unwrap();
                        let node = CSTNode::Specifier {
                            specifier_type: Some(specifier_type.value),
                            struct_specifier: None,
                        };
                        cst.push(node);
                        (NonTerm::Specifier, 1)
                    }
                    33 => {
                        let struct_specifier = cst.pop().unwrap();
                        let node = CSTNode::Specifier {
                            struct_specifier: Some(Box::new(struct_specifier)),
                            specifier_type: None,
                        };
                        cst.push(node);
                        (NonTerm::Specifier, 1)
                    }
                    34 => {
                        let matched_stmt = cst.pop().unwrap();
                        let node = CSTNode::Stmt {
                            matched_stmt: Some(Box::new(matched_stmt)),
                            unmatched_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::Stmt, 1)
                    }
                    35 => {
                        let unmatched_stmt = cst.pop().unwrap();
                        let node = CSTNode::Stmt {
                            unmatched_stmt: Some(Box::new(unmatched_stmt)),
                            matched_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::Stmt, 1)
                    }
                    36 => {
                        let stmt_list = cst.pop().unwrap();
                        let stmt = cst.pop().unwrap();
                        let node = CSTNode::StmtList {
                            stmt: Box::new(stmt),
                            stmt_list: Some(Box::new(stmt_list)),
                        };
                        cst.push(node);
                        (NonTerm::StmtList, 2)
                    }
                    37 => {
                        let stmt = cst.pop().unwrap();
                        let node = CSTNode::StmtList {
                            stmt: Box::new(stmt),
                            stmt_list: None,
                        };
                        cst.push(node);
                        (NonTerm::StmtList, 1)
                    }
                    38 => {
                        let rc = sym.pop().unwrap();
                        let lc = sym.pop().unwrap();
                        let id = sym.pop().unwrap();
                        let struct_type = sym.pop().unwrap();
                        let def_list = cst.pop().unwrap();
                        let node = CSTNode::StructSpecifier {
                            struct_type: struct_type.value,
                            id: Some(id.value),
                            lc: Some(lc.value),
                            rc: Some(rc.value),
                            def_list: Some(Box::new(def_list)),
                        };
                        cst.push(node);
                        (NonTerm::StructSpecifier, 5)
                    }
                    39 => {
                        let rc = sym.pop().unwrap();
                        let lc = sym.pop().unwrap();
                        let struct_type = sym.pop().unwrap();
                        let def_list = cst.pop().unwrap();
                        let node = CSTNode::StructSpecifier {
                            struct_type: struct_type.value,
                            id: None,
                            lc: Some(lc.value),
                            rc: Some(rc.value),
                            def_list: Some(Box::new(def_list)),
                        };
                        cst.push(node);
                        (NonTerm::StructSpecifier, 4)
                    }
                    40 => {
                        let id = sym.pop().unwrap();
                        let struct_type = sym.pop().unwrap();
                        let node = CSTNode::StructSpecifier {
                            struct_type: struct_type.value,
                            id: Some(id.value),
                            lc: None,
                            rc: None,
                            def_list: None,
                        };
                        cst.push(node);
                        (NonTerm::StructSpecifier, 2)
                    }
                    41 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let if_stmt = sym.pop().unwrap();
                        let stmt = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::UnMatchedStmt {
                            while_stmt: None,
                            lp: Some(lp.value),
                            expression: Some(Box::new(expression)),
                            rp: Some(rp.value),
                            unmatched_stmt: None,
                            if_stmt: Some(if_stmt.value),
                            else_stmt: None,
                            stmt: Some(Box::new(stmt)),
                            matched_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::UnMatchedStmt, 5)
                    }
                    42 => {
                        let else_stmt = sym.pop().unwrap();
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let if_stmt = sym.pop().unwrap();
                        let unmatched_stmt = cst.pop().unwrap();
                        let matched_stmt = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::UnMatchedStmt {
                            while_stmt: None,
                            lp: Some(lp.value),
                            expression: Some(Box::new(expression)),
                            rp: Some(rp.value),
                            unmatched_stmt: Some(Box::new(unmatched_stmt)),
                            if_stmt: Some(if_stmt.value),
                            else_stmt: Some(else_stmt.value),
                            stmt: None,
                            matched_stmt: Some(Box::new(matched_stmt)),
                        };
                        cst.push(node);
                        (NonTerm::UnMatchedStmt, 7)
                    }
                    43 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let while_stmt = sym.pop().unwrap();
                        let unmatched_stmt = cst.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::UnMatchedStmt {
                            while_stmt: Some(while_stmt.value),
                            lp: Some(lp.value),
                            expression: Some(Box::new(expression)),
                            rp: Some(rp.value),
                            unmatched_stmt: Some(Box::new(unmatched_stmt)),
                            if_stmt: None,
                            else_stmt: None,
                            stmt: None,
                            matched_stmt: None,
                        };
                        cst.push(node);
                        (NonTerm::UnMatchedStmt, 5)
                    }
                    44 => {
                        let id = sym.pop().unwrap();
                        let node = CSTNode::VarDec {
                            id: Some(id.value),
                            var_dec: None,
                            lt: None,
                            rt: None,
                            literal: None,
                        };
                        cst.push(node);
                        (NonTerm::VarDec, 1)
                    }
                    45 => {
                        let rt = sym.pop().unwrap();
                        let literal = sym.pop().unwrap();
                        let lt = sym.pop().unwrap();
                        let var_dec = cst.pop().unwrap();
                        let node = CSTNode::VarDec {
                            id: None,
                            var_dec: Some(Box::new(var_dec)),
                            lt: Some(lt.value),
                            rt: Some(rt.value),
                            literal: Some(literal.value),
                        };
                        cst.push(node);
                        (NonTerm::VarDec, 4)
                    }
                    46 => {
                        let var_list = cst.pop().unwrap();
                        let para_dec = cst.pop().unwrap();
                        let sepa = sym.pop().unwrap();
                        let node = CSTNode::VarList {
                            para_dec: Box::new(para_dec),
                            sepa: Some(sepa.value),
                            var_list: Some(Box::new(var_list)),
                        };
                        cst.push(node);
                        (NonTerm::VarList, 3)
                    }
                    47 => {
                        let para_dec = cst.pop().unwrap();
                        let node = CSTNode::VarList {
                            para_dec: Box::new(para_dec),
                            sepa: None,
                            var_list: None,
                        };
                        cst.push(node);
                        (NonTerm::VarList, 1)
                    }
                    48 => {
                        let logical_or = cst.pop().unwrap();
                        let node = CSTNode::Assign {
                            logical_or: Box::new(logical_or),
                            assign_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::Assign, 1)
                    }
                    49 => {
                        let assign_prime = cst.pop().unwrap();
                        let logical_or = cst.pop().unwrap();
                        let node = CSTNode::Assign {
                            logical_or: Box::new(logical_or),
                            assign_prime: Some(Box::new(assign_prime)),
                        };
                        cst.push(node);
                        (NonTerm::Assign, 2)
                    }
                    50 => {
                        let assign_op = sym.pop().unwrap();
                        let assign_prime = cst.pop().unwrap();
                        let logical_or = cst.pop().unwrap();
                        let node = CSTNode::AssignPrime {
                            op: assign_op.value,
                            logical_or: Box::new(logical_or),
                            assign_prime: Some(Box::new(assign_prime)),
                        };
                        cst.push(node);
                        (NonTerm::AssignPrime, 3)
                    }
                    51 => {
                        let assign_op = sym.pop().unwrap();
                        let logical_or = cst.pop().unwrap();
                        let node = CSTNode::AssignPrime {
                            op: assign_op.value,
                            logical_or: Box::new(logical_or),
                            assign_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::AssignPrime, 2)
                    }
                    52 => {
                        let comparison_prime = cst.pop().unwrap();
                        let term = cst.pop().unwrap();
                        let node = CSTNode::Comparison {
                            term: Box::new(term),
                            comparison_prime: Some(Box::new(comparison_prime)),
                        };
                        cst.push(node);
                        (NonTerm::Comparison, 2)
                    }
                    53 => {
                        let term = cst.pop().unwrap();
                        let node = CSTNode::Comparison {
                            term: Box::new(term),
                            comparison_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::Comparison, 1)
                    }
                    54..58 => {
                        let op = sym.pop().unwrap();
                        let comparison_prime = cst.pop().unwrap();
                        let term = cst.pop().unwrap();
                        let node = CSTNode::ComparisonPrime {
                            op: op.value,
                            term: Box::new(term),
                            comparison_prime: Some(Box::new(comparison_prime)),
                        };
                        cst.push(node);
                        (NonTerm::ComparisonPrime, 3)
                    }
                    58..62 => {
                        let op = sym.pop().unwrap();
                        let term = cst.pop().unwrap();
                        let node = CSTNode::ComparisonPrime {
                            op: op.value,
                            term: Box::new(term),
                            comparison_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::ComparisonPrime, 2)
                    }
                    62 => {
                        let equality_prime = cst.pop().unwrap();
                        let comparison = cst.pop().unwrap();
                        let node = CSTNode::Equality {
                            comparison: Box::new(comparison),
                            equality_prime: Some(Box::new(equality_prime)),
                        };
                        cst.push(node);
                        (NonTerm::Equality, 2)
                    }
                    63 => {
                        let comparison = cst.pop().unwrap();
                        let node = CSTNode::Equality {
                            comparison: Box::new(comparison),
                            equality_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::Equality, 1)
                    }
                    64 | 65 => {
                        let op = sym.pop().unwrap();
                        let equality_prime = cst.pop().unwrap();
                        let comparison = cst.pop().unwrap();
                        let node = CSTNode::EqualityPrime {
                            op: op.value,
                            comparison: Box::new(comparison),
                            equality_prime: Some(Box::new(equality_prime)),
                        };
                        cst.push(node);
                        (NonTerm::EqualityPrime, 3)
                    }
                    66 | 67 => {
                        let op = sym.pop().unwrap();
                        let comparison = cst.pop().unwrap();
                        let node = CSTNode::EqualityPrime {
                            op: op.value,
                            comparison: Box::new(comparison),
                            equality_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::EqualityPrime, 2)
                    }
                    68 => {
                        let assign = cst.pop().unwrap();
                        let node = CSTNode::Expression(Box::new(assign));
                        cst.push(node);
                        (NonTerm::Expression, 1)
                    }
                    69 => {
                        let factor_prime = cst.pop().unwrap();
                        let unary = cst.pop().unwrap();
                        let node = CSTNode::Factor {
                            unary: Box::new(unary),
                            factor_prime: Some(Box::new(factor_prime)),
                        };
                        cst.push(node);
                        (NonTerm::Factor, 2)
                    }
                    70 => {
                        let unary = cst.pop().unwrap();
                        let node = CSTNode::Factor {
                            unary: Box::new(unary),
                            factor_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::Factor, 1)
                    }
                    71 | 72 => {
                        let op = sym.pop().unwrap();
                        let factor_prime = cst.pop().unwrap();
                        let unary = cst.pop().unwrap();
                        let node = CSTNode::FactorPrime {
                            op: op.value,
                            unary: Box::new(unary),
                            factor_prime: Some(Box::new(factor_prime)),
                        };
                        cst.push(node);
                        (NonTerm::FactorPrime, 3)
                    }
                    73 | 74 => {
                        let op = sym.pop().unwrap();
                        let unary = cst.pop().unwrap();
                        let node = CSTNode::FactorPrime {
                            op: op.value,
                            unary: Box::new(unary),
                            factor_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::FactorPrime, 2)
                    }
                    75 => {
                        let logical_and_prime = cst.pop().unwrap();
                        let equality = cst.pop().unwrap();
                        let node = CSTNode::LogicalAnd {
                            equality: Box::new(equality),
                            logical_and_prime: Some(Box::new(logical_and_prime)),
                        };
                        cst.push(node);
                        (NonTerm::LogicalAnd, 2)
                    }
                    76 => {
                        let equality = cst.pop().unwrap();
                        let node = CSTNode::LogicalAnd {
                            equality: Box::new(equality),
                            logical_and_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::LogicalAnd, 1)
                    }
                    77 => {
                        let op = sym.pop().unwrap();
                        let logical_and_prime = cst.pop().unwrap();
                        let equality = cst.pop().unwrap();
                        let node = CSTNode::LogicalAndPrime {
                            op: op.value,
                            equality: Box::new(equality),
                            logical_and_prime: Some(Box::new(logical_and_prime)),
                        };
                        cst.push(node);
                        (NonTerm::LogicalAndPrime, 3)
                    }
                    78 => {
                        let op = sym.pop().unwrap();
                        let equality = cst.pop().unwrap();
                        let node = CSTNode::LogicalAndPrime {
                            op: op.value,
                            equality: Box::new(equality),
                            logical_and_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::LogicalAndPrime, 2)
                    }
                    79 => {
                        let logical_or_prime = cst.pop().unwrap();
                        let logical_and = cst.pop().unwrap();
                        let node = CSTNode::LogicalOr {
                            logical_and: Box::new(logical_and),
                            logical_or_prime: Some(Box::new(logical_or_prime)),
                        };
                        cst.push(node);
                        (NonTerm::LogicalOr, 2)
                    }
                    80 => {
                        let logical_and = cst.pop().unwrap();
                        let node = CSTNode::LogicalOr {
                            logical_and: Box::new(logical_and),
                            logical_or_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::LogicalOr, 1)
                    }
                    81 => {
                        let op = sym.pop().unwrap();
                        let logical_or_prime = cst.pop().unwrap();
                        let logical_and = cst.pop().unwrap();
                        let node = CSTNode::LogicalOrPrime {
                            op: op.value,
                            logical_and: Box::new(logical_and),
                            logical_or_prime: Some(Box::new(logical_or_prime)),
                        };
                        cst.push(node);
                        (NonTerm::LogicalOrPrime, 3)
                    }
                    82 => {
                        let op = sym.pop().unwrap();
                        let logical_and = cst.pop().unwrap();
                        let node = CSTNode::LogicalOrPrime {
                            op: op.value,
                            logical_and: Box::new(logical_and),
                            logical_or_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::LogicalOrPrime, 2)
                    }
                    83..87 => {
                        let symbol = sym.pop().unwrap();
                        let node = CSTNode::Primary {
                            symbol: Some(symbol),
                            fun_call: None,
                            expression: None,
                            lp: None,
                            rp: None,
                        };
                        cst.push(node);
                        (NonTerm::Primary, 1)
                    }
                    87 => {
                        let rp = sym.pop().unwrap();
                        let lp = sym.pop().unwrap();
                        let expression = cst.pop().unwrap();
                        let node = CSTNode::Primary {
                            symbol: None,
                            lp: Some(lp.value),
                            rp: Some(rp.value),
                            expression: Some(Box::new(expression)),
                            fun_call: None,
                        };
                        cst.push(node);
                        (NonTerm::Primary, 3)
                    }
                    88 => {
                        let fun_call = cst.pop().unwrap();
                        let node = CSTNode::Primary {
                            symbol: None,
                            lp: None,
                            rp: None,
                            expression: None,
                            fun_call: Some(Box::new(fun_call)),
                        };
                        cst.push(node);
                        (NonTerm::Primary, 1)
                    }
                    89 => {
                        let term_prime = cst.pop().unwrap();
                        let factor = cst.pop().unwrap();
                        let node = CSTNode::Term {
                            factor: Box::new(factor),
                            term_prime: Some(Box::new(term_prime)),
                        };
                        cst.push(node);
                        (NonTerm::Term, 2)
                    }
                    90 => {
                        let factor = cst.pop().unwrap();
                        let node = CSTNode::Term {
                            factor: Box::new(factor),
                            term_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::Term, 1)
                    }
                    91 | 92 => {
                        let term_prime = cst.pop().unwrap();
                        let factor = cst.pop().unwrap();
                        let op = sym.pop().unwrap();
                        let node = CSTNode::TermPrime {
                            op: op.value,
                            factor: Box::new(factor),
                            term_prime: Some(Box::new(term_prime)),
                        };
                        cst.push(node);
                        (NonTerm::TermPrime, 3)
                    }
                    93 | 94 => {
                        let factor = cst.pop().unwrap();
                        let op = sym.pop().unwrap();
                        let node = CSTNode::TermPrime {
                            op: op.value,
                            factor: Box::new(factor),
                            term_prime: None,
                        };
                        cst.push(node);
                        (NonTerm::TermPrime, 2)
                    }
                    96 => {
                        let primary = cst.pop().unwrap();
                        let node = CSTNode::Unary {
                            op: None,
                            unary: Box::new(primary),
                        };
                        cst.push(node);
                        (NonTerm::Unary, 1)
                    }
                    95 | 97 => {
                        let op = sym.pop().unwrap();
                        let unary = cst.pop().unwrap();
                        let node = CSTNode::Unary {
                            op: Some(op.value),
                            unary: Box::new(unary),
                        };
                        cst.push(node);
                        (NonTerm::Unary, 2)
                    }
                    _ => unreachable!(),
                };
                for _ in 0..rhs_len {
                    // let last_state = state.last().unwrap();
                    // println!("pop state {:?}", last_state);
                    state.pop();
                }
                // 根据 GOTO 表推进
                let st2 = state.last().unwrap().to_index();
                if let Some(tgt) = &super::constant::GOTO[st2][lhs.to_index()] {
                    // println!("reduce push {:?} {:?} {:?}", st2, *tgt, look);
                    state.push(*tgt);
                }
            }
            Action::Accept => {
                // 最后一个符号就是 E
                break;
            }

            Action::Error => {
                let tok = look.unwrap();
                error_handler(&state, tok);
                break;
                // skip error token
                //     loop {
                //         let look = tokens.get(index);
                //         match look {
                //             Some(token) => {
                //                 if token.types == PhraseType::Separator {
                //                     index += 1;
                //                     break;
                //                 } else {
                //                     index += 1;
                //                 }
                //             }
                //             None => break,
                //         }
                //     }
                //     if let Some(token) = tokens.get(index) {
                //         println!("start over token {:?}", token);
                //         state.clear();
                //         state.push(State::S0);
                //         continue;
                //     } else {
                //         break;
                //     }
            }
        }
    }
    cst
}
