use std::fmt::{Debug, Write};
use convert_case::{Case, Casing};

use pest_meta::ast::RuleType;
use pest_meta::optimizer::{OptimizedExpr, OptimizedRule};
use pest_meta::parse_and_optimize;
use yggdrasil_ir::grammar::GrammarInfo;
use yggdrasil_ir::rule::GrammarRule;
use crate::utils::Buffer;


pub struct PestConverter {}


impl Default for PestConverter {
    fn default() -> Self {
        Self {}
    }
}

impl PestConverter {
    pub fn parse_pest(&self, text: &str) -> String {
        use bnf::Grammar;

        let input =
            "<postal-address> ::= <name-part> <street-address> <zip-part>

        <name-part> ::= <personal-part> <last-name> <opt-suffix-part> <EOL>
                        | <personal-part> <name-part>

    <personal-part> ::= <initial> '.' | <first-name>

    <street-address> ::= <house-num> <street-name> <opt-apt-num> <EOL>

        <zip-part> ::= <town-name> ',' <state-code> <ZIP-code> <EOL>

    <opt-suffix-part> ::= 'Sr.' | 'Jr.' | <roman-numeral> | ''
        <opt-apt-num> ::= <apt-num> | ''";

        let grammar: Result<Grammar, _> = input.parse();
        match grammar {
            Ok(g) => println!("{:#?}", g),
            Err(e) => println!("Failed to make grammar from String: {}", e),
        }
        let (_, rules) = parse_and_optimize(text).unwrap();
        let mut buffer = Buffer::new(self);
        for rule in rules {
            rule.build_ygg(&mut buffer).unwrap();
        }
        buffer.finish()
    }
}

trait FromPest {
    fn build_ygg(&self, f: &mut Buffer<PestConverter>) -> std::fmt::Result;
    fn is_single(&self) -> bool {
        false
    }
}

impl<'i> FromPest for OptimizedRule {
    fn build_ygg(&self, f: &mut Buffer<PestConverter>) -> std::fmt::Result {
        match self.ty {
            RuleType::Atomic => {
                f.write_str("atomic ")?
            }
            RuleType::CompoundAtomic => {
                f.write_str("atomic ")?
            }
            RuleType::Silent => {
                f.write_str("ignore ")?
            }
            _ => {}
        };
        writeln!(f, "class {} {{", self.name.to_case(Case::Pascal))?;
        self.expr.build_ygg(f)?;
        f.write_str("}\n")
    }
}

impl<'i> FromPest for OptimizedExpr {
    fn build_ygg(&self, f: &mut Buffer<PestConverter>) -> std::fmt::Result {
        match self {
            OptimizedExpr::Str(s) => {
                writeln!(f, " {:?}", s)?
            }
            OptimizedExpr::Insens(s) => {
                writeln!(f, " @insensitive({:?})", s)?
            }
            OptimizedExpr::Range(min, max) => {
                writeln!(f, " [{min}-{max}]")?
            }
            OptimizedExpr::Ident(s) => {
                writeln!(f, " {}", s.to_case(Case::Pascal))?
            }
            OptimizedExpr::PeekSlice(a, b) => {
                writeln!(f, "@peek({}, {})", a, b.unwrap_or(i32::MAX))?
            }
            OptimizedExpr::PosPred(a) => {
                f.write_str("&(")?;
                a.build_ygg(f)?;
                f.write_str(")")?;
            }
            OptimizedExpr::NegPred(a) => {
                f.write_str("!(")?;
                a.build_ygg(f)?;
                f.write_str(")")?;
            }
            OptimizedExpr::Seq(a, b) => {
                a.build_ygg(f)?;
                b.build_ygg(f)?;
            }
            OptimizedExpr::Choice(a, b) => {
                a.build_ygg(f)?;
                f.write_str(" | ")?;
                b.build_ygg(f)?;
            }
            OptimizedExpr::Opt(a) => {
                f.write_str("(")?;
                a.build_ygg(f)?;
                f.write_str(")?")?;
            }
            OptimizedExpr::Rep(a) => {
                f.write_str("(")?;
                a.build_ygg(f)?;
                f.write_str(")*")?;
            }
            OptimizedExpr::RepOnce(a) => {
                f.write_str("(")?;
                a.build_ygg(f)?;
                f.write_str(")+")?;
            }
            OptimizedExpr::Skip(a) => {
                f.write_str("@skip(")?;
                for (index, skip) in a.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    f.write_str(skip)?;
                }
                f.write_str(")")?
            }
            OptimizedExpr::Push(a) => {
                f.write_str("@push(")?;
                a.build_ygg(f)?;
                f.write_str(")")?
            }
            OptimizedExpr::NodeTag(a, b) => {
                f.write_str(b)?;
                f.write_str(":(")?;
                a.build_ygg(f)?;
                f.write_str(")")?;
            }
            OptimizedExpr::RestoreOnErr(a) => {
                f.write_str("@restore(")?;
                a.build_ygg(f)?;
                f.write_str(")")?
            }
        }
        Ok(())
    }
}