use crate::core::ast::*;
use crate::Language;

pub struct CodeGenerator {
    indent_level: usize,
    language: Language,
}

impl CodeGenerator {
    pub fn new(language: Language) -> Self {
        Self {
            indent_level: 0,
            language,
        }
    }

    pub fn generate(&self, ast: &ASTNode) -> String {
        match ast {
            ASTNode::Program(nodes) => {
                let mut code = self.generate_header();
                for node in nodes {
                    code.push_str(&self.generate_node(node));
                }
                code.push_str(&self.generate_footer());
                code
            }
            _ => self.generate_node(ast),
        }
    }

    fn generate_header(&self) -> String {
        match self.language {
            Language::C => "#include <stdio.h>\n#include <stdlib.h>\n\n".to_string(),
            Language::Cpp => "#include <iostream>\n#include <vector>\nusing namespace std;\n\n".to_string(),
            Language::CSharp => "using System;\n\n".to_string(),
            Language::ObjectiveC => "#import <Foundation/Foundation.h>\n\n".to_string(),
            Language::ObjectiveCpp => "#import <Foundation/Foundation.h>\n#include <iostream>\n\n".to_string(),
            Language::Python => "#!/usr/bin/env python3\n# -*- coding: utf-8 -*-\n\n".to_string(),
            Language::Java => "public class Main {\n\n".to_string(),
            Language::JavaScript => "// JavaScript\n\n".to_string(),
            Language::TypeScript => "// TypeScript\n\n".to_string(),
            Language::Rust => "// Rust\n\n".to_string(),
            Language::Swift => "import Foundation\n\n".to_string(),
            Language::Ruby => "#!/usr/bin/env ruby\n\n".to_string(),
            Language::Go => "package main\n\nimport \"fmt\"\n\n".to_string(),
            Language::Kotlin => "// Kotlin\n\n".to_string(),
            Language::Php => "<?php\n\n".to_string(),
            Language::R => "# R\n\n".to_string(),
            Language::Scala => "// Scala\n\n".to_string(),
            Language::Dart => "// Dart\n\n".to_string(),
            Language::Haskell => "module Main where\n\n".to_string(),
            Language::Elixir => "# Elixir\n\n".to_string(),
            Language::FSharp => "module Main\n\n".to_string(),
            Language::Sql => "-- SQL\n\n".to_string(),
            Language::Matlab => "% MATLAB\n\n".to_string(),
            Language::D => "module main;\n\n".to_string(),
            Language::Assembly => "; Assembly (x86/x64/ARM)\n\n".to_string(),
            Language::WebAssemblyWat => ";; WebAssembly text format (WAT)\n(module\n".to_string(),
            Language::PseudocodeC => "# PSEUDOCODI ORIENTAT A C (URV)\n\n".to_string(),
            Language::PseudocodeJava => "# PSEUDOCÓDIGO ORIENTADO A JAVA\n\n".to_string(),
            Language::PseudocodePython => "# PSEUDOCÓDIGO ORIENTADO A PYTHON\n\n".to_string(),
        }
    }

    fn generate_footer(&self) -> String {
        match self.language {
            Language::Java => "\n}\n".to_string(),
            Language::WebAssemblyWat => ")\n".to_string(),
            _ => String::new(),
        }
    }

    fn generate_node(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Program(nodes) => nodes
                .iter()
                .map(|n| self.generate_node(n))
                .collect::<Vec<_>>()
                .join(""),
            ASTNode::VarDeclaration {
                name,
                data_type,
                value,
            } => self.generate_var_declaration(name, data_type, value),
            ASTNode::FunctionDef {
                name,
                params,
                return_type,
                body,
            } => self.generate_function_def(name, params, return_type, body),
            ASTNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => self.generate_if_statement(condition, then_branch, else_branch),
            ASTNode::WhileLoop { condition, body } => self.generate_while_loop(condition, body),
            ASTNode::ForLoop {
                variable,
                iterable,
                body,
            } => self.generate_for_loop(variable, iterable, body),
            ASTNode::Block(statements) => self.generate_block(statements),
            ASTNode::Assignment { target, value } => self.generate_assignment(target, value),
            ASTNode::BinaryOp { op, left, right } => self.generate_binary_op(op, left, right),
            ASTNode::UnaryOp { op, operand } => self.generate_unary_op(op, operand),
            ASTNode::FunctionCall { name, args } => self.generate_function_call(name, args),
            ASTNode::MethodCall {
                object,
                method,
                args,
            } => self.generate_method_call(object, method, args),
            ASTNode::ReturnStatement(value) => self.generate_return(value),
            ASTNode::IntegerLiteral(n) => n.to_string(),
            ASTNode::FloatLiteral(f) => f.to_string(),
            ASTNode::StringLiteral(s) => self.generate_string_literal(s),
            ASTNode::BooleanLiteral(b) => self.boolean_literal(*b),
            ASTNode::Identifier(name) => name.clone(),
            ASTNode::ArrayLiteral(elements) => self.generate_array_literal(elements),
            ASTNode::ArrayAccess { array, index } => {
                format!("{}[{}]", self.generate_node(array), self.generate_node(index))
            }
            ASTNode::NullLiteral => self.null_literal().to_string(),
            ASTNode::Comment(c) => format!("{}{}{}\n", self.indent(), self.comment_prefix(), c),
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn is_brace_style(&self) -> bool {
        matches!(
            self.language,
            Language::C
                | Language::Cpp
                | Language::CSharp
                | Language::ObjectiveC
                | Language::ObjectiveCpp
                | Language::Go
                | Language::Kotlin
                | Language::Php
                | Language::Java
                | Language::JavaScript
                | Language::TypeScript
                | Language::Scala
                | Language::Dart
                | Language::Rust
                | Language::D
                | Language::Swift
        )
    }

    fn comment_prefix(&self) -> &'static str {
        match self.language {
            Language::Python
            | Language::Ruby
            | Language::R
            | Language::Elixir
            | Language::PseudocodeC
            | Language::PseudocodeJava
            | Language::PseudocodePython => "# ",
            Language::Sql | Language::Haskell => "-- ",
            Language::Matlab => "% ",
            Language::Assembly => "; ",
            Language::WebAssemblyWat => ";; ",
            _ => "// ",
        }
    }

    fn null_literal(&self) -> &'static str {
        match self.language {
            Language::C | Language::Cpp | Language::ObjectiveC | Language::ObjectiveCpp => "NULL",
            Language::Go => "nil",
            Language::Python | Language::PseudocodePython => "None",
            Language::Ruby => "nil",
            Language::R => "NA",
            Language::Swift => "nil",
            Language::Rust => "None",
            Language::Haskell => "Nothing",
            Language::Elixir => "nil",
            Language::Sql => "NULL",
            Language::Matlab => "[]",
            Language::Assembly | Language::WebAssemblyWat => "0",
            _ => "null",
        }
    }

    fn boolean_literal(&self, value: bool) -> String {
        match self.language {
            Language::C | Language::Cpp | Language::ObjectiveC | Language::ObjectiveCpp => {
                if value { "1" } else { "0" }.to_string()
            }
            Language::Python | Language::PseudocodePython | Language::Haskell => {
                if value { "True" } else { "False" }.to_string()
            }
            Language::R | Language::Sql => {
                if value { "TRUE" } else { "FALSE" }.to_string()
            }
            Language::Assembly | Language::WebAssemblyWat => {
                if value { "1" } else { "0" }.to_string()
            }
            Language::Ruby => {
                if value { "true" } else { "false" }.to_string()
            }
            _ => value.to_string(),
        }
    }

    fn stmt_end(&self) -> &'static str {
        if self.is_brace_style() || matches!(self.language, Language::Sql | Language::Matlab) {
            ";"
        } else {
            ""
        }
    }

    fn generate_var_declaration(
        &self,
        name: &str,
        data_type: &DataType,
        value: &Option<Box<ASTNode>>,
    ) -> String {
        let indent = self.indent();
        let value_str = value
            .as_ref()
            .map(|v| self.generate_node(v))
            .unwrap_or_else(|| self.null_literal().to_string());

        match self.language {
            Language::C | Language::ObjectiveC => format!(
                "{}{} {} = {}{}\n",
                indent,
                self.get_c_family_type(data_type),
                name,
                value_str,
                self.stmt_end()
            ),
            Language::Cpp | Language::ObjectiveCpp | Language::D => {
                format!("{}auto {} = {}{}\n", indent, name, value_str, self.stmt_end())
            }
            Language::Go | Language::CSharp => {
                format!("{}var {} = {}{}\n", indent, name, value_str, self.stmt_end())
            }
            Language::Python | Language::Ruby | Language::R | Language::Haskell | Language::PseudocodePython => {
                format!("{}{} = {}\n", indent, name, value_str)
            }
            Language::Elixir => format!("{}{} = {}\n", indent, name, value_str),
            Language::Java => format!(
                "{}{} {} = {}{}\n",
                indent,
                self.get_java_type(data_type),
                name,
                value_str,
                self.stmt_end()
            ),
            Language::JavaScript | Language::TypeScript => {
                format!("{}let {} = {}{}\n", indent, name, value_str, self.stmt_end())
            }
            Language::Kotlin => format!("{}var {} = {}\n", indent, name, value_str),
            Language::Php => format!("{}${} = {}{}\n", indent, name, value_str, self.stmt_end()),
            Language::Scala => format!("{}var {} = {}\n", indent, name, value_str),
            Language::Dart => format!("{}var {} = {}{}\n", indent, name, value_str, self.stmt_end()),
            Language::FSharp => format!("{}let mutable {} = {}\n", indent, name, value_str),
            Language::Sql => format!(
                "{}DECLARE @{} {} = {}{}\n",
                indent,
                name,
                self.get_sql_type(data_type),
                value_str,
                self.stmt_end()
            ),
            Language::Matlab => format!("{}{} = {}{}\n", indent, name, value_str, self.stmt_end()),
            Language::Assembly => {
                format!("{}; var {}\n{}mov {}, {}\n", indent, name, indent, name, value_str)
            }
            Language::WebAssemblyWat => format!(
                "{}(local ${} i32)\n{}(local.set ${} {})\n",
                indent, name, indent, name, value_str
            ),
            Language::Rust => format!("{}let mut {} = {}{}\n", indent, name, value_str, self.stmt_end()),
            Language::Swift => format!("{}var {} = {}\n", indent, name, value_str),
            Language::PseudocodeC | Language::PseudocodeJava => {
                format!("{}{} ← {}\n", indent, name, value_str)
            }
        }
    }

    fn generate_function_def(
        &self,
        name: &str,
        params: &[Parameter],
        return_type: &DataType,
        body: &ASTNode,
    ) -> String {
        let mut result = String::new();
        let params_csv = params
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        match self.language {
            Language::C | Language::ObjectiveC => {
                result.push_str(&format!(
                    "{} {}({}) ",
                    self.get_c_family_type(return_type),
                    name,
                    self.c_like_params(params)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Cpp | Language::ObjectiveCpp => {
                result.push_str(&format!("auto {}({}) ", name, params_csv));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Go => {
                result.push_str(&format!(
                    "func {}({}) {} ",
                    name,
                    self.go_params(params),
                    self.get_go_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::CSharp => {
                result.push_str(&format!(
                    "static {} {}({}) ",
                    self.get_csharp_type(return_type),
                    name,
                    self.csharp_params(params)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Python | Language::PseudocodePython => {
                result.push_str(&format!("def {}({}):\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
            }
            Language::Ruby => {
                result.push_str(&format!("def {}({})\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("end\n");
            }
            Language::Kotlin => {
                result.push_str(&format!(
                    "fun {}({}): {} ",
                    name,
                    self.kotlin_params(params),
                    self.get_kotlin_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Php => {
                result.push_str(&format!("function {}({}) ", name, self.php_params(params)));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Java => {
                result.push_str(&format!(
                    "static {} {}({}) ",
                    self.get_java_type(return_type),
                    name,
                    self.java_params(params)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::JavaScript => {
                result.push_str(&format!("function {}({}) ", name, params_csv));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::TypeScript => {
                result.push_str(&format!(
                    "function {}({}): {} ",
                    name,
                    self.ts_params(params),
                    self.get_ts_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Scala => {
                result.push_str(&format!(
                    "def {}({}): {} = ",
                    name,
                    self.scala_params(params),
                    self.get_scala_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Dart => {
                result.push_str(&format!(
                    "{} {}({}) ",
                    self.get_dart_type(return_type),
                    name,
                    params_csv
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Rust => {
                result.push_str(&format!(
                    "fn {}({}) -> {} ",
                    name,
                    self.rust_params(params),
                    self.get_rust_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Swift => {
                result.push_str(&format!(
                    "func {}({}) -> {} ",
                    name,
                    self.swift_params(params),
                    self.get_swift_type(return_type)
                ));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::R => {
                result.push_str(&format!("{} <- function({}) ", name, params_csv));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Haskell => {
                result.push_str(&format!("{} {} =\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
            }
            Language::Elixir => {
                result.push_str(&format!("def {}({}) do\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("end\n");
            }
            Language::FSharp => {
                result.push_str(&format!("let {} {} =\n", name, params_csv.replace(", ", " ")));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
            }
            Language::Sql => {
                result.push_str(&format!("CREATE PROCEDURE {}\nAS\nBEGIN\n", name));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("END;\n");
            }
            Language::Matlab => {
                result.push_str(&format!("function out = {}({})\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("end\n");
            }
            Language::D => {
                result.push_str(&format!("auto {}({}) ", name, params_csv));
                result.push_str(&self.generate_statement_or_block(body));
            }
            Language::Assembly => {
                result.push_str(&format!("{}:\n", name));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("ret\n");
            }
            Language::WebAssemblyWat => {
                result.push_str(&format!("  (func ${} ", name));
                for p in params {
                    result.push_str(&format!("(param ${} i32) ", p.name));
                }
                result.push_str("(result i32)\n");
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 2;
                result.push_str(&nested.generate_node(body));
                result.push_str("  )\n");
            }
            Language::PseudocodeC => {
                result.push_str(&format!("FUNCIÓ {}({})\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("FI FUNCIÓ\n");
            }
            Language::PseudocodeJava => {
                result.push_str(&format!("MÉTODO {}({})\n", name, params_csv));
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str("FIN MÉTODO\n");
            }
        }

        result
    }

    fn generate_if_statement(
        &self,
        condition: &ASTNode,
        then_branch: &ASTNode,
        else_branch: &Option<Box<ASTNode>>,
    ) -> String {
        let cond = self.generate_node(condition);
        let indent = self.indent();

        if self.is_brace_style() {
            let mut result = format!("{}if ({}) ", indent, cond);
            result.push_str(&self.generate_statement_or_block(then_branch));
            if let Some(else_br) = else_branch {
                result.push_str(&format!("{}else ", indent));
                result.push_str(&self.generate_statement_or_block(else_br));
            }
            return result;
        }

        match self.language {
            Language::PseudocodeC => {
                let mut result = format!("{}SI {} ALESHORES\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(then_branch));
                if let Some(else_br) = else_branch {
                    result.push_str(&format!("{}SINÓ\n", indent));
                    result.push_str(&nested.generate_node(else_br));
                }
                result.push_str(&format!("{}FI SI\n", indent));
                result
            }
            Language::PseudocodeJava => {
                let mut result = format!("{}SI {} ENTONCES\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(then_branch));
                if let Some(else_br) = else_branch {
                    result.push_str(&format!("{}SINO\n", indent));
                    result.push_str(&nested.generate_node(else_br));
                }
                result.push_str(&format!("{}FIN SI\n", indent));
                result
            }
            Language::Python | Language::PseudocodePython => {
                let mut result = format!("{}if {}:\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(then_branch));
                if let Some(else_br) = else_branch {
                    result.push_str(&format!("{}else:\n", indent));
                    result.push_str(&nested.generate_node(else_br));
                }
                result
            }
            Language::Ruby => {
                let mut result = format!("{}if {}\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(then_branch));
                if let Some(else_br) = else_branch {
                    result.push_str(&format!("{}else\n", indent));
                    result.push_str(&nested.generate_node(else_br));
                }
                result.push_str(&format!("{}end\n", indent));
                result
            }
            _ => format!("{}# if {}\n", indent, cond),
        }
    }

    fn generate_while_loop(&self, condition: &ASTNode, body: &ASTNode) -> String {
        let cond = self.generate_node(condition);
        let indent = self.indent();

        if self.is_brace_style() {
            return format!(
                "{}while ({}) {}",
                indent,
                cond,
                self.generate_statement_or_block(body)
            );
        }

        match self.language {
            Language::PseudocodeC => {
                let mut result = format!("{}MENTRE {} FER\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str(&format!("{}FI MENTRE\n", indent));
                result
            }
            Language::PseudocodeJava => {
                let mut result = format!("{}MIENTRAS {} HACER\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str(&format!("{}FIN MIENTRAS\n", indent));
                result
            }
            Language::Python | Language::PseudocodePython => {
                let mut result = format!("{}while {}:\n", indent, cond);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result
            }
            _ => format!("{}# while {}\n", indent, cond),
        }
    }

    fn generate_for_loop(&self, variable: &str, iterable: &ASTNode, body: &ASTNode) -> String {
        let it = self.generate_node(iterable);
        let indent = self.indent();

        match self.language {
            Language::PseudocodeC => {
                let mut result = format!("{}PER {} EN {} FER\n", indent, variable, it);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str(&format!("{}FI PER\n", indent));
                result
            }
            Language::PseudocodeJava => {
                let mut result = format!("{}PARA {} EN {} HACER\n", indent, variable, it);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str(&format!("{}FIN PARA\n", indent));
                result
            }
            Language::C | Language::Cpp | Language::ObjectiveC | Language::ObjectiveCpp => {
                format!(
                    "{}for (int {} = 0; {} < {}; {}++) {}",
                    indent,
                    variable,
                    variable,
                    it,
                    variable,
                    self.generate_statement_or_block(body)
                )
            }
            Language::Go | Language::D => format!(
                "{}for {} := 0; {} < {}; {}++ {}",
                indent,
                variable,
                variable,
                it,
                variable,
                self.generate_statement_or_block(body)
            ),
            Language::Kotlin => format!(
                "{}for ({} in {}) {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::CSharp => format!(
                "{}foreach (var {} in {}) {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::Java => format!(
                "{}for (var {} : {}) {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::JavaScript | Language::TypeScript | Language::Dart | Language::Php => format!(
                "{}for (const {} of {}) {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::Scala => format!(
                "{}for ({} <- {}) {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::Rust => format!(
                "{}for {} in {} {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::Swift => format!(
                "{}for {} in {} {}",
                indent,
                variable,
                it,
                self.generate_statement_or_block(body)
            ),
            Language::Python | Language::PseudocodePython => {
                let mut result = format!("{}for {} in {}:\n", indent, variable, it);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result
            }
            Language::Ruby => {
                let mut result = format!("{}{}.each do |{}|\n", indent, it, variable);
                let mut nested = Self::new(self.language);
                nested.indent_level = self.indent_level + 1;
                result.push_str(&nested.generate_node(body));
                result.push_str(&format!("{}end\n", indent));
                result
            }
            _ => format!("{}# for {} in {}\n", indent, variable, it),
        }
    }

    fn generate_block(&self, statements: &[Box<ASTNode>]) -> String {
        let mut result = String::new();

        if self.is_brace_style() {
            result.push_str("{\n");
            let mut nested = Self::new(self.language);
            nested.indent_level = self.indent_level + 1;
            for stmt in statements {
                result.push_str(&nested.generate_node(stmt));
            }
            result.push_str(&format!("{}}}\n", self.indent()));
            return result;
        }

        let mut nested = Self::new(self.language);
        nested.indent_level = self.indent_level + 1;
        for stmt in statements {
            result.push_str(&nested.generate_node(stmt));
        }
        result
    }

    fn generate_statement_or_block(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Block(_) => self.generate_node(node),
            _ => {
                if self.is_brace_style() {
                    let mut result = String::from("{\n");
                    let mut nested = Self::new(self.language);
                    nested.indent_level = self.indent_level + 1;
                    result.push_str(&nested.generate_node(node));
                    result.push_str(&format!("{}}}\n", self.indent()));
                    result
                } else {
                    let mut nested = Self::new(self.language);
                    nested.indent_level = self.indent_level + 1;
                    nested.generate_node(node)
                }
            }
        }
    }

    fn generate_assignment(&self, target: &str, value: &ASTNode) -> String {
        let indent = self.indent();
        if matches!(self.language, Language::PseudocodeC | Language::PseudocodeJava) {
            return format!("{}{} ← {}\n", indent, target, self.generate_node(value));
        }
        if matches!(self.language, Language::R) {
            return format!("{}{} <- {}\n", indent, target, self.generate_node(value));
        }
        if matches!(self.language, Language::Php) {
            return format!(
                "{}${} = {}{}\n",
                indent,
                target,
                self.generate_node(value),
                self.stmt_end()
            );
        }
        if matches!(self.language, Language::Assembly) {
            return format!("{}mov {}, {}\n", indent, target, self.generate_node(value));
        }
        if matches!(self.language, Language::WebAssemblyWat) {
            return format!("{}(local.set ${} {})\n", indent, target, self.generate_node(value));
        }
        format!(
            "{}{} = {}{}\n",
            indent,
            target,
            self.generate_node(value),
            self.stmt_end()
        )
    }

    fn generate_binary_op(&self, op: &BinaryOperator, left: &ASTNode, right: &ASTNode) -> String {
        format!("({} {} {})", self.generate_node(left), op, self.generate_node(right))
    }

    fn generate_unary_op(&self, op: &UnaryOperator, operand: &ASTNode) -> String {
        format!("({}{})", op, self.generate_node(operand))
    }

    fn generate_function_call(&self, name: &str, args: &[Box<ASTNode>]) -> String {
        let args_str = args
            .iter()
            .map(|arg| self.generate_node(arg))
            .collect::<Vec<_>>()
            .join(", ");

        if matches!(self.language, Language::PseudocodeC) {
            format!("CRIDAR {}({})", name, args_str)
        } else if matches!(self.language, Language::PseudocodeJava | Language::PseudocodePython) {
            format!("LLAMAR {}({})", name, args_str)
        } else if matches!(self.language, Language::Sql) {
            format!("EXEC {} {}", name, args_str)
        } else if matches!(self.language, Language::Assembly) {
            format!("call {}", name)
        } else if matches!(self.language, Language::WebAssemblyWat) {
            format!("(call ${})", name)
        } else {
            format!("{}({})", name, args_str)
        }
    }

    fn generate_method_call(&self, object: &ASTNode, method: &str, args: &[Box<ASTNode>]) -> String {
        let args_str = args
            .iter()
            .map(|arg| self.generate_node(arg))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}.{}({})", self.generate_node(object), method, args_str)
    }

    fn generate_return(&self, value: &Option<Box<ASTNode>>) -> String {
        let indent = self.indent();

        match self.language {
            Language::PseudocodeC | Language::PseudocodeJava | Language::PseudocodePython => {
                if let Some(v) = value {
                    format!("{}RETORNAR {}\n", indent, self.generate_node(v))
                } else {
                    format!("{}RETORNAR\n", indent)
                }
            }
            _ => {
                if let Some(v) = value {
                    format!("{}return {}{}\n", indent, self.generate_node(v), self.stmt_end())
                } else {
                    format!("{}return{}\n", indent, self.stmt_end())
                }
            }
        }
    }

    fn generate_string_literal(&self, s: &str) -> String {
        format!("\"{}\"", s)
    }

    fn generate_array_literal(&self, elements: &[Box<ASTNode>]) -> String {
        let elem_str = elements
            .iter()
            .map(|e| self.generate_node(e))
            .collect::<Vec<_>>()
            .join(", ");

        match self.language {
            Language::C | Language::Cpp | Language::ObjectiveC | Language::ObjectiveCpp => {
                format!("{{ {} }}", elem_str)
            }
            Language::WebAssemblyWat => format!(";; array [{}]", elem_str),
            _ => format!("[{}]", elem_str),
        }
    }

    fn go_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{} {}", p.name, self.get_go_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn kotlin_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.get_kotlin_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn php_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("${}", p.name))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn scala_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.get_scala_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn c_like_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{} {}", self.get_c_family_type(&p.data_type), p.name))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn java_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{} {}", self.get_java_type(&p.data_type), p.name))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn csharp_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{} {}", self.get_csharp_type(&p.data_type), p.name))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn ts_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.get_ts_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn rust_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.get_rust_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn swift_params(&self, params: &[Parameter]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", p.name, self.get_swift_type(&p.data_type)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn get_c_family_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "int",
            DataType::Float => "float",
            DataType::String => "char*",
            DataType::Boolean => "int",
            DataType::Void => "void",
            DataType::Array => "int[]",
            DataType::Any => "int",
        }
    }

    fn get_java_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "int",
            DataType::Float => "double",
            DataType::String => "String",
            DataType::Boolean => "boolean",
            DataType::Void => "void",
            DataType::Array => "int[]",
            DataType::Any => "Object",
        }
    }

    fn get_csharp_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "int",
            DataType::Float => "double",
            DataType::String => "string",
            DataType::Boolean => "bool",
            DataType::Void => "void",
            DataType::Array => "int[]",
            DataType::Any => "object",
        }
    }

    fn get_ts_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer | DataType::Float => "number",
            DataType::String => "string",
            DataType::Boolean => "boolean",
            DataType::Void => "void",
            DataType::Array => "any[]",
            DataType::Any => "any",
        }
    }

    fn get_rust_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "i64",
            DataType::Float => "f64",
            DataType::String => "String",
            DataType::Boolean => "bool",
            DataType::Void => "()",
            DataType::Array => "Vec<i64>",
            DataType::Any => "i64",
        }
    }

    fn get_swift_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "Int",
            DataType::Float => "Double",
            DataType::String => "String",
            DataType::Boolean => "Bool",
            DataType::Void => "Void",
            DataType::Array => "[Int]",
            DataType::Any => "Any",
        }
    }

    fn get_go_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "int",
            DataType::Float => "float64",
            DataType::String => "string",
            DataType::Boolean => "bool",
            DataType::Void => "",
            DataType::Array => "[]int",
            DataType::Any => "any",
        }
    }

    fn get_kotlin_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "Int",
            DataType::Float => "Double",
            DataType::String => "String",
            DataType::Boolean => "Boolean",
            DataType::Void => "Unit",
            DataType::Array => "IntArray",
            DataType::Any => "Any",
        }
    }

    fn get_scala_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "Int",
            DataType::Float => "Double",
            DataType::String => "String",
            DataType::Boolean => "Boolean",
            DataType::Void => "Unit",
            DataType::Array => "Array[Int]",
            DataType::Any => "Any",
        }
    }

    fn get_dart_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "int",
            DataType::Float => "double",
            DataType::String => "String",
            DataType::Boolean => "bool",
            DataType::Void => "void",
            DataType::Array => "List<int>",
            DataType::Any => "dynamic",
        }
    }

    fn get_sql_type(&self, data_type: &DataType) -> &'static str {
        match data_type {
            DataType::Integer => "INT",
            DataType::Float => "FLOAT",
            DataType::String => "NVARCHAR(MAX)",
            DataType::Boolean => "BIT",
            DataType::Void => "INT",
            DataType::Array => "NVARCHAR(MAX)",
            DataType::Any => "SQL_VARIANT",
        }
    }
}

pub fn generate_code(ast: &ASTNode, target_language: Language) -> String {
    let generator = CodeGenerator::new(target_language);
    generator.generate(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic() {
        let ast = ASTNode::IntegerLiteral(42);
        let code = generate_code(&ast, Language::C);
        assert_eq!(code, "42");
    }

    #[test]
    fn test_generate_pseudocode_c_header_cat() {
        let ast = ASTNode::Program(vec![Box::new(ASTNode::IntegerLiteral(1))]);
        let code = generate_code(&ast, Language::PseudocodeC);
        assert!(code.contains("PSEUDOCODI ORIENTAT A C"));
    }
}
