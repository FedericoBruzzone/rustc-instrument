//! A Rustc plugin that prints out the name of all items in a crate.

#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

// use rustc_middle::ty::TyCtxt;
use rustc_ast::{
    ast::*,
    visit::{self, *},
};
use rustc_span::symbol::*;
use rustc_span::Span;

use clap::Parser;
use rustc_instrument::{CrateFilter, RustcPlugin, RustcPluginArgs, Utf8Path};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, env, process::Command};

#[cfg(feature = "test1")]
fn test1() {}
#[cfg(feature = "test2")]
fn test2() {}
#[cfg(all(feature = "test1", not(feature = "test2")))]
fn test3() {}

// This struct is the plugin provided to the rustc_plugin framework,
// and it must be exported for use by the CLI/driver binaries.
pub struct PrintAst;

impl RustcPlugin for PrintAst {
    type Args = PrintAstArgs;

    fn version(&self) -> Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> Cow<'static, str> {
        "print-hir-ast-driver".into()
    }

    // In the CLI, we ask Clap to parse arguments and also specify a CrateFilter.
    // If one of the CLI arguments was a specific file to analyze, then you
    // could provide a different filter.
    fn args(&self, _target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args> {
        let args = PrintAstArgs::parse_from(env::args().skip(1));
        let filter = CrateFilter::AllCrates;
        RustcPluginArgs { args, filter }
    }

    // Pass Cargo arguments (like --feature) from the top-level CLI to Cargo.
    fn modify_cargo(&self, cargo: &mut Command, args: &Self::Args) {
        // Add --features test1 to the cargo command.
        // cargo.arg("--features").arg("test2");

        // Enable all features.
        // cargo.arg("--all-features");

        cargo.args(&args.cargo_args);
    }

    // In the driver, we use the Rustc API to start a compiler session
    // for the arguments given to us by rustc_plugin.
    fn run(
        self,
        compiler_args: Vec<String>,
        plugin_args: Self::Args,
    ) -> rustc_interface::interface::Result<()> {
        let mut callbacks = PrintAstCallbacks { _args: plugin_args };
        let compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
        compiler.run()
    }
}

// To parse CLI arguments, we use Clap for this example. But that
// detail is up to you.
#[derive(Parser, Serialize, Deserialize, Default)]
pub struct PrintAstArgs {
    #[clap(last = true)]
    cargo_args: Vec<String>,
}

struct PrintAstCallbacks {
    _args: PrintAstArgs,
}

impl rustc_driver::Callbacks for PrintAstCallbacks {
    // At the top-level, the Rustc API uses an event-based interface for
    // accessing the compiler at different stages of compilation. In this callback,
    // all the type-checking has completed.
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        _queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        // We extract a key data structure, the `TyCtxt`, which is all we need
        // for our simple task of printing out item names.
        // queries
        //   .global_ctxt()
        //   .unwrap()
        //   .enter(|tcx| print_all_items(tcx, &self.args));

        // Note that you should generally allow compilation to continue. If
        // your plugin is being invoked on a dependency, then you need to ensure
        // the dependency is type-checked (its .rmeta file is emitted into target/)
        // so that its dependents can read the compiler outputs.
        rustc_driver::Compilation::Continue
    }

    fn after_crate_root_parsing<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        let krate_res = queries.parse().unwrap();
        let krate = &(*krate_res.borrow());
        // println!("Crate: {krate:#?}");

        let collector = &mut CollectVisitor;
        let _ = visit::walk_crate(collector, krate);

        rustc_driver::Compilation::Continue
    }
}

struct CollectVisitor;

// Visit in pre-order
impl<'ast> Visitor<'ast> for CollectVisitor {
    fn visit_ident(&mut self, _ident: Ident) {}
    fn visit_foreign_item(&mut self, i: &'ast ForeignItem) {
        walk_foreign_item(self, i)
    }
    fn visit_item(&mut self, i: &'ast Item) {
        walk_item(self, i)
    }
    fn visit_local(&mut self, l: &'ast Local) {
        walk_local(self, l)
    }
    fn visit_block(&mut self, b: &'ast Block) {
        walk_block(self, b)
    }
    fn visit_stmt(&mut self, s: &'ast Stmt) {
        walk_stmt(self, s)
    }
    fn visit_param(&mut self, param: &'ast Param) {
        walk_param(self, param)
    }
    fn visit_arm(&mut self, a: &'ast Arm) {
        walk_arm(self, a)
    }
    fn visit_pat(&mut self, p: &'ast Pat) {
        walk_pat(self, p)
    }
    fn visit_anon_const(&mut self, c: &'ast AnonConst) {
        walk_anon_const(self, c)
    }
    fn visit_expr(&mut self, ex: &'ast Expr) {
        walk_expr(self, ex)
    }
    /// This method is a hack to workaround unstable of `stmt_expr_attributes`.
    /// It can be removed once that feature is stabilized.
    fn visit_method_receiver_expr(&mut self, ex: &'ast Expr) {
        self.visit_expr(ex)
    }
    fn visit_expr_post(&mut self, _ex: &'ast Expr) {}
    fn visit_ty(&mut self, t: &'ast Ty) {
        walk_ty(self, t)
    }
    fn visit_generic_param(&mut self, param: &'ast GenericParam) {
        walk_generic_param(self, param)
    }
    fn visit_generics(&mut self, g: &'ast Generics) {
        walk_generics(self, g)
    }
    fn visit_closure_binder(&mut self, b: &'ast ClosureBinder) {
        walk_closure_binder(self, b)
    }
    fn visit_where_predicate(&mut self, p: &'ast WherePredicate) {
        walk_where_predicate(self, p)
    }
    fn visit_fn(&mut self, fk: FnKind<'ast>, _: Span, _: NodeId) {
        walk_fn(self, fk)
    }
    fn visit_assoc_item(&mut self, i: &'ast AssocItem, ctxt: AssocCtxt) {
        walk_assoc_item(self, i, ctxt)
    }
    fn visit_trait_ref(&mut self, t: &'ast TraitRef) {
        walk_trait_ref(self, t)
    }
    fn visit_param_bound(&mut self, bounds: &'ast GenericBound, _ctxt: BoundKind) {
        walk_param_bound(self, bounds)
    }
    fn visit_poly_trait_ref(&mut self, t: &'ast PolyTraitRef) {
        walk_poly_trait_ref(self, t)
    }
    fn visit_variant_data(&mut self, s: &'ast VariantData) {
        walk_struct_def(self, s)
    }
    fn visit_field_def(&mut self, s: &'ast FieldDef) {
        walk_field_def(self, s)
    }
    fn visit_enum_def(&mut self, enum_definition: &'ast EnumDef) {
        walk_enum_def(self, enum_definition)
    }
    fn visit_variant(&mut self, v: &'ast Variant) {
        walk_variant(self, v)
    }
    fn visit_variant_discr(&mut self, discr: &'ast AnonConst) {
        self.visit_anon_const(discr);
    }
    fn visit_label(&mut self, label: &'ast Label) {
        walk_label(self, label)
    }
    fn visit_lifetime(&mut self, lifetime: &'ast Lifetime, _: LifetimeCtxt) {
        walk_lifetime(self, lifetime)
    }
    fn visit_mac_call(&mut self, mac: &'ast MacCall) {
        walk_mac(self, mac)
    }
    fn visit_mac_def(&mut self, _mac: &'ast MacroDef, _id: NodeId) {
        // Nothing to do
    }
    fn visit_path(&mut self, path: &'ast Path, _id: NodeId) {
        walk_path(self, path)
    }
    fn visit_use_tree(&mut self, use_tree: &'ast UseTree, id: NodeId, _nested: bool) {
        walk_use_tree(self, use_tree, id)
    }
    fn visit_path_segment(&mut self, path_segment: &'ast PathSegment) {
        walk_path_segment(self, path_segment)
    }
    fn visit_generic_args(&mut self, generic_args: &'ast GenericArgs) {
        walk_generic_args(self, generic_args)
    }
    fn visit_generic_arg(&mut self, generic_arg: &'ast GenericArg) {
        walk_generic_arg(self, generic_arg)
    }
    fn visit_assoc_constraint(&mut self, constraint: &'ast AssocConstraint) {
        walk_assoc_constraint(self, constraint)
    }
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        // Macro attributes are here
        walk_attribute(self, attr)
    }
    fn visit_vis(&mut self, vis: &'ast Visibility) {
        walk_vis(self, vis)
    }
    fn visit_fn_ret_ty(&mut self, ret_ty: &'ast FnRetTy) {
        walk_fn_ret_ty(self, ret_ty)
    }
    fn visit_fn_header(&mut self, _header: &'ast FnHeader) {
        // Nothing to do
    }
    fn visit_expr_field(&mut self, f: &'ast ExprField) {
        walk_expr_field(self, f)
    }
    fn visit_pat_field(&mut self, fp: &'ast PatField) {
        walk_pat_field(self, fp)
    }
    fn visit_crate(&mut self, krate: &'ast Crate) {
        walk_crate(self, krate)
    }
    fn visit_inline_asm(&mut self, asm: &'ast InlineAsm) {
        walk_inline_asm(self, asm)
    }
    fn visit_format_args(&mut self, fmt: &'ast FormatArgs) {
        walk_format_args(self, fmt)
    }
    fn visit_inline_asm_sym(&mut self, sym: &'ast InlineAsmSym) {
        walk_inline_asm_sym(self, sym)
    }
    fn visit_capture_by(&mut self, _capture_by: &'ast CaptureBy) {
        // Nothing to do
    }
}

// The core of our analysis. It doesn't do much, just access some methods on the `TyCtxt`.
// I recommend reading the Rustc Development Guide to better understand which compiler APIs
// are relevant to whatever task you have.
// fn _print_all_items(tcx: TyCtxt, args: &PrintAllItemsPluginArgs) {
//     let hir = tcx.hir();
//     // for maybe_owner in hir.krate().owners.raw.iter() {
//     //   let owner = maybe_owner.as_owner();
//     //   if let Some(owner) = owner {
//     //     println!("owner: {:#?}", owner);
//     //   }
//     // }
//     for item_id in hir.items() {
//         let item = hir.item(item_id);
//         let mut msg = format!(
//             "There is an item \"{}\" of type \"{}\"",
//             item.ident,
//             item.kind.descr()
//         );
//         if args.allcaps {
//             msg = msg.to_uppercase();
//         }
//         println!("{msg}");
//     }
// }
