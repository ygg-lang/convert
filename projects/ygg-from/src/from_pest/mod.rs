use std::fmt::{Debug, Write};

use pest_meta::ast::RuleType;
use pest_meta::optimizer::{OptimizedExpr, OptimizedRule};
use pest_meta::parse_and_optimize;

use yggdrasil_ir::{ChoiceExpression, ConcatExpression, DataKind, ExpressionNode, GrammarInfo, GrammarRule, Operator, RuleReference};

pub struct PestConverter {}



trait FromPest {
    fn build_ygg(&self, f: impl Write, soft: bool) -> std::fmt::Result {
        let _ = soft;
        let _ = f;
        unimplemented!()
    }
}


impl Default for PestConverter {
    fn default() -> Self {
        Self {}
    }
}

impl PestConverter {
    pub fn parse_pest(&self, text: &str) -> GrammarInfo {
        let (_, rules) = parse_and_optimize(text).unwrap();
        let mut info = GrammarInfo::default();
        for (index, rule) in rules.iter().enumerate() {
            let out = self.visit_rule(rule, index);
            info.rules.insert(out.name.clone(), out);
        }
        return info;
    }
}

impl PestConverter {
    fn visit_rule(&self, rule: &OptimizedRule, index: usize) -> GrammarRule {
        let name = rule.name.clone();
        let entry = index == 0;
        let atomic = match rule.ty {
            RuleType::Atomic => { true }
            RuleType::CompoundAtomic => { true }
            _ => false
        };
        let body = self.visit_expr(&rule.expr, atomic);

        GrammarRule {
            name,
            r#type: "".to_string(),
            document: "".to_string(),
            derives: Default::default(),
            auto_inline: false,
            auto_boxed: false,
            entry,
            union: false,
            force_export: false,
            body,
            range: Default::default(),
        }
    }
    fn visit_expr(&self, expr: &OptimizedExpr, atomic: bool) -> ExpressionNode {
        match expr {
            OptimizedExpr::Str(s) => {
                DataKind::String(s.to_owned()).into()
            }
            OptimizedExpr::Insens(_) => { unreachable!() }
            OptimizedExpr::Range(_, _) => { unreachable!() }
            OptimizedExpr::Ident(v) => {
                RuleReference::new(v).to_node("")
            }
            OptimizedExpr::PeekSlice(_, _) => { unreachable!() }
            OptimizedExpr::PosPred(_) => { unreachable!() }
            OptimizedExpr::NegPred(_) => { unreachable!() }
            OptimizedExpr::Seq(l, r) => {
                if atomic {
                    self.visit_expr(l, atomic) & self.visit_expr(r, atomic)
                } else {
                    self.visit_expr(l, atomic) + self.visit_expr(r, atomic)
                }
            }
            OptimizedExpr::Choice(l, r) => {
                self.visit_expr(l, atomic) | self.visit_expr(r, atomic)
            }
            OptimizedExpr::Opt(v) => { self.visit_expr(v, atomic) + Operator::Optional }
            OptimizedExpr::Rep(v) => {
                self.visit_expr(v, atomic) + Operator::Repeats
            }
            OptimizedExpr::Skip(_) => { unreachable!() }
            OptimizedExpr::Push(_) => { unreachable!() }
            OptimizedExpr::RestoreOnErr(_) => { unreachable!() }
        }
    }
}

// impl FromPest for Rule {
//     fn build_ygg(&self, f: impl Write, _: bool) -> std::fmt::Result {
//         let mut soft_concat = false;
//         let kind = match self.ty {
//             RuleType::Normal => {
//                 soft_concat = true;
//                 ""
//             }
//             RuleType::Silent => {
//                 soft_concat = true;
//                 "_"
//             }
//             RuleType::Atomic => { "" }
//             RuleType::CompoundAtomic => { "" }
//             RuleType::NonAtomic => {
//                 soft_concat = true;
//                 ""
//             }
//         };
//         write!(f, "{name} {kind}= ", name = self.name, kind = kind)?;
//         FromPest::build_ygg(&self.expr, f, soft_concat);
//         write!(f, ";")
//     }
// }
//
// impl FromPest for Expr {
//     fn build_ygg(&self, f: impl Write, soft: bool) -> std::fmt::Result {
//         match self {
//             Expr::Str(s) => {
//                 f.write_str(s)
//             }
//             Expr::Insens(s) => {
//                 write!(f, "/{}/i", s)
//             }
//             Expr::Range(a, b) => {
//                 write!(f, "[{}-{}]", a, b)
//             }
//             Expr::Ident(v) => {
//                 f.write_str(v)
//             }
//             Expr::PeekSlice(a, b) => {write!(f, "unimplemented!")}
//             Expr::PosPred(a) => {write!(f, "unimplemented!")}
//             Expr::NegPred(a) => {write!(f, "unimplemented!")}
//             Expr::Seq(a, b) => {
//                 a.build_ygg(f, soft)?
//             }
//             Expr::Choice(a, b) => {write!(f, "unimplemented!")}
//             Expr::Opt(a) => {write!(f, "unimplemented!")}
//             Expr::Rep(a) => {write!(f, "unimplemented!")}
//             Expr::RepOnce(a) => {write!(f, "unimplemented!")}
//             Expr::RepExact(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMin(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMax(a, b) => {write!(f, "unimplemented!")}
//             Expr::RepMinMax(e, a, b) => {
//                 e.build_ygg(f,soft)?;
//                 write!(f, "{{{},{}}}", a,b )
//             }
//             Expr::Skip(a) => {
//                 write!(f, "unimplemented!")
//             }
//             Expr::Push(push) => {
//                 write!(f, "@push(")?;
//                 push.build_ygg(f, soft)?;
//                 write!(f, ")")?
//             }
//         }
//         Ok(())
//     }
// }
