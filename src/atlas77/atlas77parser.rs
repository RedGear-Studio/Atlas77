// Generated from Atlas77.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::atlas77listener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const COMMA:isize=1; 
		pub const SEMICOLON:isize=2; 
		pub const COLON:isize=3; 
		pub const LPAREN:isize=4; 
		pub const RPAREN:isize=5; 
		pub const LBRACE:isize=6; 
		pub const RBRACE:isize=7; 
		pub const LBRACKET:isize=8; 
		pub const RBRACKET:isize=9; 
		pub const PLUS:isize=10; 
		pub const MINUS:isize=11; 
		pub const MULT:isize=12; 
		pub const DIV:isize=13; 
		pub const MOD:isize=14; 
		pub const EQ:isize=15; 
		pub const NEQ:isize=16; 
		pub const LT:isize=17; 
		pub const GT:isize=18; 
		pub const LTE:isize=19; 
		pub const GTE:isize=20; 
		pub const ARROW:isize=21; 
		pub const NOT:isize=22; 
		pub const OR:isize=23; 
		pub const AND:isize=24; 
		pub const WS:isize=25; 
		pub const NEWLINE:isize=26; 
		pub const TRUE:isize=27; 
		pub const FALSE:isize=28; 
		pub const LET:isize=29; 
		pub const RETURN:isize=30; 
		pub const AS:isize=31; 
		pub const IF:isize=32; 
		pub const ELSE:isize=33; 
		pub const FN:isize=34; 
		pub const I32:isize=35; 
		pub const F32:isize=36; 
		pub const BOOL:isize=37; 
		pub const VOID:isize=38; 
		pub const CHAR:isize=39; 
		pub const IDENTIFIER:isize=40; 
		pub const NUMBER:isize=41; 
		pub const STRING:isize=42;
	pub const RULE_program:usize = 0; 
	pub const RULE_statement:usize = 1; 
	pub const RULE_let_statement:usize = 2; 
	pub const RULE_return_statement:usize = 3; 
	pub const RULE_expression_statement:usize = 4; 
	pub const RULE_if_statement:usize = 5; 
	pub const RULE_block:usize = 6; 
	pub const RULE_function_declaration:usize = 7; 
	pub const RULE_parameter:usize = 8; 
	pub const RULE_type:usize = 9; 
	pub const RULE_expression:usize = 10; 
	pub const RULE_assignment:usize = 11; 
	pub const RULE_logical_or:usize = 12; 
	pub const RULE_logical_and:usize = 13; 
	pub const RULE_equality:usize = 14; 
	pub const RULE_comparison:usize = 15; 
	pub const RULE_addition:usize = 16; 
	pub const RULE_multiplication:usize = 17; 
	pub const RULE_unary:usize = 18; 
	pub const RULE_call:usize = 19; 
	pub const RULE_primary:usize = 20;
	pub const ruleNames: [&'static str; 21] =  [
		"program", "statement", "let_statement", "return_statement", "expression_statement", 
		"if_statement", "block", "function_declaration", "parameter", "type", 
		"expression", "assignment", "logical_or", "logical_and", "equality", "comparison", 
		"addition", "multiplication", "unary", "call", "primary"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;40] = [
		None, Some("','"), Some("';'"), Some("':'"), Some("'('"), Some("')'"), 
		Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), Some("'+'"), Some("'-'"), 
		Some("'*'"), Some("'/'"), Some("'%'"), Some("'=='"), Some("'!='"), Some("'<'"), 
		Some("'>'"), Some("'<='"), Some("'>='"), Some("'->'"), Some("'!'"), Some("'||'"), 
		Some("'&&'"), None, None, Some("'true'"), Some("'false'"), Some("'let'"), 
		Some("'return'"), Some("'as'"), Some("'if'"), Some("'else'"), Some("'fn'"), 
		Some("'i32'"), Some("'f32'"), Some("'bool'"), Some("'void'"), Some("'char'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;43]  = [
		None, Some("COMMA"), Some("SEMICOLON"), Some("COLON"), Some("LPAREN"), 
		Some("RPAREN"), Some("LBRACE"), Some("RBRACE"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("PLUS"), Some("MINUS"), Some("MULT"), Some("DIV"), Some("MOD"), Some("EQ"), 
		Some("NEQ"), Some("LT"), Some("GT"), Some("LTE"), Some("GTE"), Some("ARROW"), 
		Some("NOT"), Some("OR"), Some("AND"), Some("WS"), Some("NEWLINE"), Some("TRUE"), 
		Some("FALSE"), Some("LET"), Some("RETURN"), Some("AS"), Some("IF"), Some("ELSE"), 
		Some("FN"), Some("I32"), Some("F32"), Some("BOOL"), Some("VOID"), Some("CHAR"), 
		Some("IDENTIFIER"), Some("NUMBER"), Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,Atlas77ParserExt<'input>, I, Atlas77ParserContextType , dyn Atlas77Listener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type Atlas77TreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, Atlas77ParserContextType , dyn Atlas77Listener<'input> + 'a>;

/// Parser for Atlas77 grammar
pub struct Atlas77Parser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				Atlas77ParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> Atlas77Parser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> Atlas77Parser<'input, I, DefaultErrorStrategy<'input,Atlas77ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for Atlas77Parser
pub trait Atlas77ParserContext<'input>:
	for<'x> Listenable<dyn Atlas77Listener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=Atlas77ParserContextType>
{}

antlr_rust::coerce_from!{ 'input : Atlas77ParserContext<'input> }

impl<'input> Atlas77ParserContext<'input> for TerminalNode<'input,Atlas77ParserContextType> {}
impl<'input> Atlas77ParserContext<'input> for ErrorNode<'input,Atlas77ParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Atlas77ParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Atlas77Listener<'input> + 'input }

pub struct Atlas77ParserContextType;
antlr_rust::tid!{Atlas77ParserContextType}

impl<'input> ParserNodeType<'input> for Atlas77ParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn Atlas77ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct Atlas77ParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> Atlas77ParserExt<'input>{
}
antlr_rust::tid! { Atlas77ParserExt<'a> }

impl<'input> TokenAware<'input> for Atlas77ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for Atlas77ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for Atlas77ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Atlas77.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(48);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LPAREN) | (1usize << LBRACE) | (1usize << MINUS) | (1usize << NOT) | (1usize << TRUE) | (1usize << FALSE) | (1usize << LET) | (1usize << RETURN))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (IF - 32)) | (1usize << (FN - 32)) | (1usize << (IDENTIFIER - 32)) | (1usize << (NUMBER - 32)) | (1usize << (STRING - 32)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(42);
				recog.statement()?;

				recog.base.set_state(44);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==NEWLINE {
					{
					recog.base.set_state(43);
					recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

					}
				}

				}
				}
				recog.base.set_state(50);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn let_statement(&self) -> Option<Rc<Let_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn return_statement(&self) -> Option<Rc<Return_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_statement(&self) -> Option<Rc<Expression_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_declaration(&self) -> Option<Rc<Function_declarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(57);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LET 
				=> {
					{
					/*InvokeRule let_statement*/
					recog.base.set_state(51);
					recog.let_statement()?;

					}
				}

			 RETURN 
				=> {
					{
					/*InvokeRule return_statement*/
					recog.base.set_state(52);
					recog.return_statement()?;

					}
				}

			 LPAREN | MINUS | NOT | TRUE | FALSE | IDENTIFIER | NUMBER | STRING 
				=> {
					{
					/*InvokeRule expression_statement*/
					recog.base.set_state(53);
					recog.expression_statement()?;

					}
				}

			 IF 
				=> {
					{
					/*InvokeRule if_statement*/
					recog.base.set_state(54);
					recog.if_statement()?;

					}
				}

			 LBRACE 
				=> {
					{
					/*InvokeRule block*/
					recog.base.set_state(55);
					recog.block()?;

					}
				}

			 FN 
				=> {
					{
					/*InvokeRule function_declaration*/
					recog.base.set_state(56);
					recog.function_declaration()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- let_statement ----------------
pub type Let_statementContextAll<'input> = Let_statementContext<'input>;


pub type Let_statementContext<'input> = BaseParserRuleContext<'input,Let_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Let_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Let_statementContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Let_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_let_statement(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_let_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Let_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_let_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_let_statement }
}
antlr_rust::tid!{Let_statementContextExt<'a>}

impl<'input> Let_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Let_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Let_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Let_statementContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Let_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LET
/// Returns `None` if there is no child corresponding to token LET
fn LET(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LET, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn type(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Let_statementContextAttrs<'input> for Let_statementContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn let_statement(&mut self,)
	-> Result<Rc<Let_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Let_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_let_statement);
        let mut _localctx: Rc<Let_statementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(59);
			recog.base.match_token(LET,&mut recog.err_handler)?;

			recog.base.set_state(60);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(63);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COLON {
				{
				recog.base.set_state(61);
				recog.base.match_token(COLON,&mut recog.err_handler)?;

				/*InvokeRule type*/
				recog.base.set_state(62);
				recog.type()?;

				}
			}

			recog.base.set_state(67);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EQ {
				{
				recog.base.set_state(65);
				recog.base.match_token(EQ,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(66);
				recog.expression()?;

				}
			}

			recog.base.set_state(69);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- return_statement ----------------
pub type Return_statementContextAll<'input> = Return_statementContext<'input>;


pub type Return_statementContext<'input> = BaseParserRuleContext<'input,Return_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Return_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Return_statementContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Return_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_return_statement(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_return_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Return_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_return_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_return_statement }
}
antlr_rust::tid!{Return_statementContextExt<'a>}

impl<'input> Return_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Return_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Return_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Return_statementContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Return_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> Return_statementContextAttrs<'input> for Return_statementContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn return_statement(&mut self,)
	-> Result<Rc<Return_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Return_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_return_statement);
        let mut _localctx: Rc<Return_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(71);
			recog.base.match_token(RETURN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(72);
			recog.expression()?;

			recog.base.set_state(73);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression_statement ----------------
pub type Expression_statementContextAll<'input> = Expression_statementContext<'input>;


pub type Expression_statementContext<'input> = BaseParserRuleContext<'input,Expression_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Expression_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Expression_statementContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Expression_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression_statement(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_expression_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expression_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression_statement }
}
antlr_rust::tid!{Expression_statementContextExt<'a>}

impl<'input> Expression_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expression_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expression_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Expression_statementContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Expression_statementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> Expression_statementContextAttrs<'input> for Expression_statementContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression_statement(&mut self,)
	-> Result<Rc<Expression_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expression_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_expression_statement);
        let mut _localctx: Rc<Expression_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(75);
			recog.expression()?;

			recog.base.set_state(76);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_statement ----------------
pub type If_statementContextAll<'input> = If_statementContext<'input>;


pub type If_statementContext<'input> = BaseParserRuleContext<'input,If_statementContextExt<'input>>;

#[derive(Clone)]
pub struct If_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for If_statementContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for If_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_statement(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_if_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr_rust::tid!{If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait If_statementContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<If_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}

}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_statement(&mut self,)
	-> Result<Rc<If_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(78);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			recog.base.set_state(79);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(80);
			recog.expression()?;

			recog.base.set_state(81);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			/*InvokeRule statement*/
			recog.base.set_state(82);
			recog.statement()?;

			recog.base.set_state(85);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(83);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(84);
					recog.statement()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(87);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(91);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(88);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(93);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(100);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LPAREN) | (1usize << LBRACE) | (1usize << MINUS) | (1usize << NOT) | (1usize << TRUE) | (1usize << FALSE) | (1usize << LET) | (1usize << RETURN))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (IF - 32)) | (1usize << (FN - 32)) | (1usize << (IDENTIFIER - 32)) | (1usize << (NUMBER - 32)) | (1usize << (STRING - 32)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(94);
				recog.statement()?;

				recog.base.set_state(96);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==NEWLINE {
					{
					recog.base.set_state(95);
					recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

					}
				}

				}
				}
				recog.base.set_state(102);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(103);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_declaration ----------------
pub type Function_declarationContextAll<'input> = Function_declarationContext<'input>;


pub type Function_declarationContext<'input> = BaseParserRuleContext<'input,Function_declarationContextExt<'input>>;

#[derive(Clone)]
pub struct Function_declarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Function_declarationContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Function_declarationContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_declaration(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_function_declaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Function_declarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_declaration }
}
antlr_rust::tid!{Function_declarationContextExt<'a>}

impl<'input> Function_declarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_declarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_declarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_declarationContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Function_declarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FN
/// Returns `None` if there is no child corresponding to token FN
fn FN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(FN, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn type(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Function_declarationContextAttrs<'input> for Function_declarationContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_declaration(&mut self,)
	-> Result<Rc<Function_declarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_declarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_function_declaration);
        let mut _localctx: Rc<Function_declarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(105);
			recog.base.match_token(FN,&mut recog.err_handler)?;

			recog.base.set_state(106);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(107);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(116);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule parameter*/
				recog.base.set_state(108);
				recog.parameter()?;

				recog.base.set_state(113);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(109);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule parameter*/
					recog.base.set_state(110);
					recog.parameter()?;

					}
					}
					recog.base.set_state(115);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(118);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(121);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ARROW {
				{
				recog.base.set_state(119);
				recog.base.match_token(ARROW,&mut recog.err_handler)?;

				/*InvokeRule type*/
				recog.base.set_state(120);
				recog.type()?;

				}
			}

			/*InvokeRule block*/
			recog.base.set_state(123);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_parameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn type(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(125);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(126);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule type*/
			recog.base.set_state(127);
			recog.type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type ----------------
pub type TypeContextAll<'input> = TypeContext<'input>;


pub type TypeContext<'input> = BaseParserRuleContext<'input,TypeContextExt<'input>>;

#[derive(Clone)]
pub struct TypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for TypeContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for TypeContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type }
}
antlr_rust::tid!{TypeContextExt<'a>}

impl<'input> TypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<TypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token I32
/// Returns `None` if there is no child corresponding to token I32
fn I32(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(I32, 0)
}
/// Retrieves first TerminalNode corresponding to token F32
/// Returns `None` if there is no child corresponding to token F32
fn F32(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(F32, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}
/// Retrieves first TerminalNode corresponding to token VOID
/// Returns `None` if there is no child corresponding to token VOID
fn VOID(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(VOID, 0)
}
/// Retrieves first TerminalNode corresponding to token CHAR
/// Returns `None` if there is no child corresponding to token CHAR
fn CHAR(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(CHAR, 0)
}

}

impl<'input> TypeContextAttrs<'input> for TypeContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type(&mut self,)
	-> Result<Rc<TypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_type);
        let mut _localctx: Rc<TypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(129);
			_la = recog.base.input.la(1);
			if { !(((((_la - 35)) & !0x3f) == 0 && ((1usize << (_la - 35)) & ((1usize << (I32 - 35)) | (1usize << (F32 - 35)) | (1usize << (BOOL - 35)) | (1usize << (VOID - 35)) | (1usize << (CHAR - 35)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logical_or(&self) -> Option<Rc<Logical_orContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(133);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule assignment*/
					recog.base.set_state(131);
					recog.assignment()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule logical_or*/
					recog.base.set_state(132);
					recog.logical_or()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignment ----------------
pub type AssignmentContextAll<'input> = AssignmentContext<'input>;


pub type AssignmentContext<'input> = BaseParserRuleContext<'input,AssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for AssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for AssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignment(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_assignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment }
}
antlr_rust::tid!{AssignmentContextExt<'a>}

impl<'input> AssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<AssignmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn logical_or(&self) -> Option<Rc<Logical_orContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment(&mut self,)
	-> Result<Rc<AssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(139);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(135);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(136);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					/*InvokeRule assignment*/
					recog.base.set_state(137);
					recog.assignment()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule logical_or*/
					recog.base.set_state(138);
					recog.logical_or()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logical_or ----------------
pub type Logical_orContextAll<'input> = Logical_orContext<'input>;


pub type Logical_orContext<'input> = BaseParserRuleContext<'input,Logical_orContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_orContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Logical_orContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Logical_orContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_or(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_logical_or(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Logical_orContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_or }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_or }
}
antlr_rust::tid!{Logical_orContextExt<'a>}

impl<'input> Logical_orContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_orContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_orContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_orContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Logical_orContextExt<'input>>{

fn logical_and_all(&self) ->  Vec<Rc<Logical_andContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logical_and(&self, i: usize) -> Option<Rc<Logical_andContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> Logical_orContextAttrs<'input> for Logical_orContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_or(&mut self,)
	-> Result<Rc<Logical_orContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_orContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_logical_or);
        let mut _localctx: Rc<Logical_orContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logical_and*/
			recog.base.set_state(141);
			recog.logical_and()?;

			recog.base.set_state(146);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OR {
				{
				{
				recog.base.set_state(142);
				recog.base.match_token(OR,&mut recog.err_handler)?;

				/*InvokeRule logical_and*/
				recog.base.set_state(143);
				recog.logical_and()?;

				}
				}
				recog.base.set_state(148);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logical_and ----------------
pub type Logical_andContextAll<'input> = Logical_andContext<'input>;


pub type Logical_andContext<'input> = BaseParserRuleContext<'input,Logical_andContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_andContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for Logical_andContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for Logical_andContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_and(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_logical_and(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Logical_andContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_and }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_and }
}
antlr_rust::tid!{Logical_andContextExt<'a>}

impl<'input> Logical_andContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_andContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_andContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_andContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<Logical_andContextExt<'input>>{

fn equality_all(&self) ->  Vec<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equality(&self, i: usize) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}

}

impl<'input> Logical_andContextAttrs<'input> for Logical_andContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_and(&mut self,)
	-> Result<Rc<Logical_andContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_andContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_logical_and);
        let mut _localctx: Rc<Logical_andContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule equality*/
			recog.base.set_state(149);
			recog.equality()?;

			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AND {
				{
				{
				recog.base.set_state(150);
				recog.base.match_token(AND,&mut recog.err_handler)?;

				/*InvokeRule equality*/
				recog.base.set_state(151);
				recog.equality()?;

				}
				}
				recog.base.set_state(156);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equality ----------------
pub type EqualityContextAll<'input> = EqualityContext<'input>;


pub type EqualityContext<'input> = BaseParserRuleContext<'input,EqualityContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for EqualityContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for EqualityContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equality(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_equality(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equality }
}
antlr_rust::tid!{EqualityContextExt<'a>}

impl<'input> EqualityContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<EqualityContextExt<'input>>{

fn comparison_all(&self) ->  Vec<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comparison(&self, i: usize) -> Option<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token EQ in current rule
fn EQ_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EQ, starting from 0.
/// Returns `None` if number of children corresponding to token EQ is less or equal than `i`.
fn EQ(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(EQ, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEQ in current rule
fn NEQ_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEQ, starting from 0.
/// Returns `None` if number of children corresponding to token NEQ is less or equal than `i`.
fn NEQ(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(NEQ, i)
}

}

impl<'input> EqualityContextAttrs<'input> for EqualityContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equality(&mut self,)
	-> Result<Rc<EqualityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_equality);
        let mut _localctx: Rc<EqualityContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule comparison*/
			recog.base.set_state(157);
			recog.comparison()?;

			recog.base.set_state(162);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==EQ || _la==NEQ {
				{
				{
				recog.base.set_state(158);
				_la = recog.base.input.la(1);
				if { !(_la==EQ || _la==NEQ) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule comparison*/
				recog.base.set_state(159);
				recog.comparison()?;

				}
				}
				recog.base.set_state(164);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comparison ----------------
pub type ComparisonContextAll<'input> = ComparisonContext<'input>;


pub type ComparisonContext<'input> = BaseParserRuleContext<'input,ComparisonContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for ComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for ComparisonContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparison(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_comparison(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparison }
}
antlr_rust::tid!{ComparisonContextExt<'a>}

impl<'input> ComparisonContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<ComparisonContextExt<'input>>{

fn addition_all(&self) ->  Vec<Rc<AdditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn addition(&self, i: usize) -> Option<Rc<AdditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LT in current rule
fn LT_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LT, starting from 0.
/// Returns `None` if number of children corresponding to token LT is less or equal than `i`.
fn LT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GT in current rule
fn GT_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GT, starting from 0.
/// Returns `None` if number of children corresponding to token GT is less or equal than `i`.
fn GT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(GT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LTE in current rule
fn LTE_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LTE, starting from 0.
/// Returns `None` if number of children corresponding to token LTE is less or equal than `i`.
fn LTE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LTE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GTE in current rule
fn GTE_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GTE, starting from 0.
/// Returns `None` if number of children corresponding to token GTE is less or equal than `i`.
fn GTE(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(GTE, i)
}

}

impl<'input> ComparisonContextAttrs<'input> for ComparisonContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparison(&mut self,)
	-> Result<Rc<ComparisonContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_comparison);
        let mut _localctx: Rc<ComparisonContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule addition*/
			recog.base.set_state(165);
			recog.addition()?;

			recog.base.set_state(170);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LT) | (1usize << GT) | (1usize << LTE) | (1usize << GTE))) != 0) {
				{
				{
				recog.base.set_state(166);
				_la = recog.base.input.la(1);
				if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LT) | (1usize << GT) | (1usize << LTE) | (1usize << GTE))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule addition*/
				recog.base.set_state(167);
				recog.addition()?;

				}
				}
				recog.base.set_state(172);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- addition ----------------
pub type AdditionContextAll<'input> = AdditionContext<'input>;


pub type AdditionContext<'input> = BaseParserRuleContext<'input,AdditionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for AdditionContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for AdditionContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_addition(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_addition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AdditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_addition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_addition }
}
antlr_rust::tid!{AdditionContextExt<'a>}

impl<'input> AdditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditionContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<AdditionContextExt<'input>>{

fn multiplication_all(&self) ->  Vec<Rc<MultiplicationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplication(&self, i: usize) -> Option<Rc<MultiplicationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PLUS in current rule
fn PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token PLUS is less or equal than `i`.
fn PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(PLUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
/// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(MINUS, i)
}

}

impl<'input> AdditionContextAttrs<'input> for AdditionContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn addition(&mut self,)
	-> Result<Rc<AdditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_addition);
        let mut _localctx: Rc<AdditionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplication*/
			recog.base.set_state(173);
			recog.multiplication()?;

			recog.base.set_state(178);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PLUS || _la==MINUS {
				{
				{
				recog.base.set_state(174);
				_la = recog.base.input.la(1);
				if { !(_la==PLUS || _la==MINUS) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule multiplication*/
				recog.base.set_state(175);
				recog.multiplication()?;

				}
				}
				recog.base.set_state(180);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiplication ----------------
pub type MultiplicationContextAll<'input> = MultiplicationContext<'input>;


pub type MultiplicationContext<'input> = BaseParserRuleContext<'input,MultiplicationContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for MultiplicationContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for MultiplicationContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiplication(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_multiplication(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MultiplicationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplication }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplication }
}
antlr_rust::tid!{MultiplicationContextExt<'a>}

impl<'input> MultiplicationContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplicationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicationContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<MultiplicationContextExt<'input>>{

fn unary_all(&self) ->  Vec<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unary(&self, i: usize) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token MULT in current rule
fn MULT_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MULT, starting from 0.
/// Returns `None` if number of children corresponding to token MULT is less or equal than `i`.
fn MULT(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(MULT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DIV in current rule
fn DIV_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DIV, starting from 0.
/// Returns `None` if number of children corresponding to token DIV is less or equal than `i`.
fn DIV(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(DIV, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MOD in current rule
fn MOD_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MOD, starting from 0.
/// Returns `None` if number of children corresponding to token MOD is less or equal than `i`.
fn MOD(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(MOD, i)
}

}

impl<'input> MultiplicationContextAttrs<'input> for MultiplicationContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplication(&mut self,)
	-> Result<Rc<MultiplicationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_multiplication);
        let mut _localctx: Rc<MultiplicationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unary*/
			recog.base.set_state(181);
			recog.unary()?;

			recog.base.set_state(186);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << MULT) | (1usize << DIV) | (1usize << MOD))) != 0) {
				{
				{
				recog.base.set_state(182);
				_la = recog.base.input.la(1);
				if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << MULT) | (1usize << DIV) | (1usize << MOD))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule unary*/
				recog.base.set_state(183);
				recog.unary()?;

				}
				}
				recog.base.set_state(188);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unary ----------------
pub type UnaryContextAll<'input> = UnaryContext<'input>;


pub type UnaryContext<'input> = BaseParserRuleContext<'input,UnaryContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for UnaryContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for UnaryContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unary(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_unary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for UnaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}
antlr_rust::tid!{UnaryContextExt<'a>}

impl<'input> UnaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<UnaryContextExt<'input>>{

fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
fn call(&self) -> Option<Rc<CallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnaryContextAttrs<'input> for UnaryContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary(&mut self,)
	-> Result<Rc<UnaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_unary);
        let mut _localctx: Rc<UnaryContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(192);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MINUS | NOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(189);
					_la = recog.base.input.la(1);
					if { !(_la==MINUS || _la==NOT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule unary*/
					recog.base.set_state(190);
					recog.unary()?;

					}
				}

			 LPAREN | TRUE | FALSE | IDENTIFIER | NUMBER | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule call*/
					recog.base.set_state(191);
					recog.call()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- call ----------------
pub type CallContextAll<'input> = CallContext<'input>;


pub type CallContext<'input> = BaseParserRuleContext<'input,CallContextExt<'input>>;

#[derive(Clone)]
pub struct CallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for CallContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for CallContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_call(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_call(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_call }
}
antlr_rust::tid!{CallContextExt<'a>}

impl<'input> CallContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CallContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<CallContextExt<'input>>{

fn primary(&self) -> Option<Rc<PrimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LPAREN in current rule
fn LPAREN_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LPAREN, starting from 0.
/// Returns `None` if number of children corresponding to token LPAREN is less or equal than `i`.
fn LPAREN(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RPAREN in current rule
fn RPAREN_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RPAREN, starting from 0.
/// Returns `None` if number of children corresponding to token RPAREN is less or equal than `i`.
fn RPAREN(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,Atlas77ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> CallContextAttrs<'input> for CallContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn call(&mut self,)
	-> Result<Rc<CallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_call);
        let mut _localctx: Rc<CallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule primary*/
			recog.base.set_state(194);
			recog.primary()?;

			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LPAREN {
				{
				{
				recog.base.set_state(195);
				recog.base.match_token(LPAREN,&mut recog.err_handler)?;

				recog.base.set_state(204);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LPAREN) | (1usize << MINUS) | (1usize << NOT) | (1usize << TRUE) | (1usize << FALSE))) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & ((1usize << (IDENTIFIER - 40)) | (1usize << (NUMBER - 40)) | (1usize << (STRING - 40)))) != 0) {
					{
					/*InvokeRule expression*/
					recog.base.set_state(196);
					recog.expression()?;

					recog.base.set_state(201);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(197);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(198);
						recog.expression()?;

						}
						}
						recog.base.set_state(203);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(206);
				recog.base.match_token(RPAREN,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(211);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primary ----------------
pub type PrimaryContextAll<'input> = PrimaryContext<'input>;


pub type PrimaryContext<'input> = BaseParserRuleContext<'input,PrimaryContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> Atlas77ParserContext<'input> for PrimaryContext<'input>{}

impl<'input,'a> Listenable<dyn Atlas77Listener<'input> + 'a> for PrimaryContext<'input>{
		fn enter(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primary(self);
		}fn exit(&self,listener: &mut (dyn Atlas77Listener<'input> + 'a)) {
			listener.exit_primary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PrimaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = Atlas77ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}
antlr_rust::tid!{PrimaryContextExt<'a>}

impl<'input> PrimaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn Atlas77ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryContextAttrs<'input>: Atlas77ParserContext<'input> + BorrowMut<PrimaryContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,Atlas77ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> PrimaryContextAttrs<'input> for PrimaryContext<'input>{}

impl<'input, I, H> Atlas77Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary(&mut self,)
	-> Result<Rc<PrimaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_primary);
        let mut _localctx: Rc<PrimaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(221);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 TRUE 
				=> {
					{
					recog.base.set_state(212);
					recog.base.match_token(TRUE,&mut recog.err_handler)?;

					}
				}

			 FALSE 
				=> {
					{
					recog.base.set_state(213);
					recog.base.match_token(FALSE,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					{
					recog.base.set_state(214);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 STRING 
				=> {
					{
					recog.base.set_state(215);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(216);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 LPAREN 
				=> {
					{
					recog.base.set_state(217);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(218);
					recog.expression()?;

					recog.base.set_state(219);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x2c\u{e2}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x03\x02\x03\x02\
	\x05\x02\x2f\x0a\x02\x07\x02\x31\x0a\x02\x0c\x02\x0e\x02\x34\x0b\x02\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x3c\x0a\x03\x03\x04\
	\x03\x04\x03\x04\x03\x04\x05\x04\x42\x0a\x04\x03\x04\x03\x04\x05\x04\x46\
	\x0a\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\
	\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\
	\x58\x0a\x07\x03\x08\x03\x08\x07\x08\x5c\x0a\x08\x0c\x08\x0e\x08\x5f\x0b\
	\x08\x03\x08\x03\x08\x05\x08\x63\x0a\x08\x07\x08\x65\x0a\x08\x0c\x08\x0e\
	\x08\x68\x0b\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\
	\x03\x09\x07\x09\x72\x0a\x09\x0c\x09\x0e\x09\x75\x0b\x09\x05\x09\x77\x0a\
	\x09\x03\x09\x03\x09\x03\x09\x05\x09\x7c\x0a\x09\x03\x09\x03\x09\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x05\x0c\u{88}\x0a\
	\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{8e}\x0a\x0d\x03\x0e\x03\x0e\
	\x03\x0e\x07\x0e\u{93}\x0a\x0e\x0c\x0e\x0e\x0e\u{96}\x0b\x0e\x03\x0f\x03\
	\x0f\x03\x0f\x07\x0f\u{9b}\x0a\x0f\x0c\x0f\x0e\x0f\u{9e}\x0b\x0f\x03\x10\
	\x03\x10\x03\x10\x07\x10\u{a3}\x0a\x10\x0c\x10\x0e\x10\u{a6}\x0b\x10\x03\
	\x11\x03\x11\x03\x11\x07\x11\u{ab}\x0a\x11\x0c\x11\x0e\x11\u{ae}\x0b\x11\
	\x03\x12\x03\x12\x03\x12\x07\x12\u{b3}\x0a\x12\x0c\x12\x0e\x12\u{b6}\x0b\
	\x12\x03\x13\x03\x13\x03\x13\x07\x13\u{bb}\x0a\x13\x0c\x13\x0e\x13\u{be}\
	\x0b\x13\x03\x14\x03\x14\x03\x14\x05\x14\u{c3}\x0a\x14\x03\x15\x03\x15\x03\
	\x15\x03\x15\x03\x15\x07\x15\u{ca}\x0a\x15\x0c\x15\x0e\x15\u{cd}\x0b\x15\
	\x05\x15\u{cf}\x0a\x15\x03\x15\x07\x15\u{d2}\x0a\x15\x0c\x15\x0e\x15\u{d5}\
	\x0b\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\
	\x03\x16\x05\x16\u{e0}\x0a\x16\x03\x16\x02\x02\x17\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x02\x08\x03\
	\x02\x25\x29\x03\x02\x11\x12\x03\x02\x13\x16\x03\x02\x0c\x0d\x03\x02\x0e\
	\x10\x04\x02\x0d\x0d\x18\x18\x02\u{ed}\x02\x32\x03\x02\x02\x02\x04\x3b\x03\
	\x02\x02\x02\x06\x3d\x03\x02\x02\x02\x08\x49\x03\x02\x02\x02\x0a\x4d\x03\
	\x02\x02\x02\x0c\x50\x03\x02\x02\x02\x0e\x59\x03\x02\x02\x02\x10\x6b\x03\
	\x02\x02\x02\x12\x7f\x03\x02\x02\x02\x14\u{83}\x03\x02\x02\x02\x16\u{87}\
	\x03\x02\x02\x02\x18\u{8d}\x03\x02\x02\x02\x1a\u{8f}\x03\x02\x02\x02\x1c\
	\u{97}\x03\x02\x02\x02\x1e\u{9f}\x03\x02\x02\x02\x20\u{a7}\x03\x02\x02\x02\
	\x22\u{af}\x03\x02\x02\x02\x24\u{b7}\x03\x02\x02\x02\x26\u{c2}\x03\x02\x02\
	\x02\x28\u{c4}\x03\x02\x02\x02\x2a\u{df}\x03\x02\x02\x02\x2c\x2e\x05\x04\
	\x03\x02\x2d\x2f\x07\x1c\x02\x02\x2e\x2d\x03\x02\x02\x02\x2e\x2f\x03\x02\
	\x02\x02\x2f\x31\x03\x02\x02\x02\x30\x2c\x03\x02\x02\x02\x31\x34\x03\x02\
	\x02\x02\x32\x30\x03\x02\x02\x02\x32\x33\x03\x02\x02\x02\x33\x03\x03\x02\
	\x02\x02\x34\x32\x03\x02\x02\x02\x35\x3c\x05\x06\x04\x02\x36\x3c\x05\x08\
	\x05\x02\x37\x3c\x05\x0a\x06\x02\x38\x3c\x05\x0c\x07\x02\x39\x3c\x05\x0e\
	\x08\x02\x3a\x3c\x05\x10\x09\x02\x3b\x35\x03\x02\x02\x02\x3b\x36\x03\x02\
	\x02\x02\x3b\x37\x03\x02\x02\x02\x3b\x38\x03\x02\x02\x02\x3b\x39\x03\x02\
	\x02\x02\x3b\x3a\x03\x02\x02\x02\x3c\x05\x03\x02\x02\x02\x3d\x3e\x07\x1f\
	\x02\x02\x3e\x41\x07\x2a\x02\x02\x3f\x40\x07\x05\x02\x02\x40\x42\x05\x14\
	\x0b\x02\x41\x3f\x03\x02\x02\x02\x41\x42\x03\x02\x02\x02\x42\x45\x03\x02\
	\x02\x02\x43\x44\x07\x11\x02\x02\x44\x46\x05\x16\x0c\x02\x45\x43\x03\x02\
	\x02\x02\x45\x46\x03\x02\x02\x02\x46\x47\x03\x02\x02\x02\x47\x48\x07\x04\
	\x02\x02\x48\x07\x03\x02\x02\x02\x49\x4a\x07\x20\x02\x02\x4a\x4b\x05\x16\
	\x0c\x02\x4b\x4c\x07\x04\x02\x02\x4c\x09\x03\x02\x02\x02\x4d\x4e\x05\x16\
	\x0c\x02\x4e\x4f\x07\x04\x02\x02\x4f\x0b\x03\x02\x02\x02\x50\x51\x07\x22\
	\x02\x02\x51\x52\x07\x06\x02\x02\x52\x53\x05\x16\x0c\x02\x53\x54\x07\x07\
	\x02\x02\x54\x57\x05\x04\x03\x02\x55\x56\x07\x23\x02\x02\x56\x58\x05\x04\
	\x03\x02\x57\x55\x03\x02\x02\x02\x57\x58\x03\x02\x02\x02\x58\x0d\x03\x02\
	\x02\x02\x59\x5d\x07\x08\x02\x02\x5a\x5c\x07\x1c\x02\x02\x5b\x5a\x03\x02\
	\x02\x02\x5c\x5f\x03\x02\x02\x02\x5d\x5b\x03\x02\x02\x02\x5d\x5e\x03\x02\
	\x02\x02\x5e\x66\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\x02\x60\x62\x05\x04\
	\x03\x02\x61\x63\x07\x1c\x02\x02\x62\x61\x03\x02\x02\x02\x62\x63\x03\x02\
	\x02\x02\x63\x65\x03\x02\x02\x02\x64\x60\x03\x02\x02\x02\x65\x68\x03\x02\
	\x02\x02\x66\x64\x03\x02\x02\x02\x66\x67\x03\x02\x02\x02\x67\x69\x03\x02\
	\x02\x02\x68\x66\x03\x02\x02\x02\x69\x6a\x07\x09\x02\x02\x6a\x0f\x03\x02\
	\x02\x02\x6b\x6c\x07\x24\x02\x02\x6c\x6d\x07\x2a\x02\x02\x6d\x76\x07\x06\
	\x02\x02\x6e\x73\x05\x12\x0a\x02\x6f\x70\x07\x03\x02\x02\x70\x72\x05\x12\
	\x0a\x02\x71\x6f\x03\x02\x02\x02\x72\x75\x03\x02\x02\x02\x73\x71\x03\x02\
	\x02\x02\x73\x74\x03\x02\x02\x02\x74\x77\x03\x02\x02\x02\x75\x73\x03\x02\
	\x02\x02\x76\x6e\x03\x02\x02\x02\x76\x77\x03\x02\x02\x02\x77\x78\x03\x02\
	\x02\x02\x78\x7b\x07\x07\x02\x02\x79\x7a\x07\x17\x02\x02\x7a\x7c\x05\x14\
	\x0b\x02\x7b\x79\x03\x02\x02\x02\x7b\x7c\x03\x02\x02\x02\x7c\x7d\x03\x02\
	\x02\x02\x7d\x7e\x05\x0e\x08\x02\x7e\x11\x03\x02\x02\x02\x7f\u{80}\x07\x2a\
	\x02\x02\u{80}\u{81}\x07\x05\x02\x02\u{81}\u{82}\x05\x14\x0b\x02\u{82}\x13\
	\x03\x02\x02\x02\u{83}\u{84}\x09\x02\x02\x02\u{84}\x15\x03\x02\x02\x02\u{85}\
	\u{88}\x05\x18\x0d\x02\u{86}\u{88}\x05\x1a\x0e\x02\u{87}\u{85}\x03\x02\x02\
	\x02\u{87}\u{86}\x03\x02\x02\x02\u{88}\x17\x03\x02\x02\x02\u{89}\u{8a}\x07\
	\x2a\x02\x02\u{8a}\u{8b}\x07\x11\x02\x02\u{8b}\u{8e}\x05\x18\x0d\x02\u{8c}\
	\u{8e}\x05\x1a\x0e\x02\u{8d}\u{89}\x03\x02\x02\x02\u{8d}\u{8c}\x03\x02\x02\
	\x02\u{8e}\x19\x03\x02\x02\x02\u{8f}\u{94}\x05\x1c\x0f\x02\u{90}\u{91}\x07\
	\x19\x02\x02\u{91}\u{93}\x05\x1c\x0f\x02\u{92}\u{90}\x03\x02\x02\x02\u{93}\
	\u{96}\x03\x02\x02\x02\u{94}\u{92}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\
	\x02\u{95}\x1b\x03\x02\x02\x02\u{96}\u{94}\x03\x02\x02\x02\u{97}\u{9c}\x05\
	\x1e\x10\x02\u{98}\u{99}\x07\x1a\x02\x02\u{99}\u{9b}\x05\x1e\x10\x02\u{9a}\
	\u{98}\x03\x02\x02\x02\u{9b}\u{9e}\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\
	\x02\u{9c}\u{9d}\x03\x02\x02\x02\u{9d}\x1d\x03\x02\x02\x02\u{9e}\u{9c}\x03\
	\x02\x02\x02\u{9f}\u{a4}\x05\x20\x11\x02\u{a0}\u{a1}\x09\x03\x02\x02\u{a1}\
	\u{a3}\x05\x20\x11\x02\u{a2}\u{a0}\x03\x02\x02\x02\u{a3}\u{a6}\x03\x02\x02\
	\x02\u{a4}\u{a2}\x03\x02\x02\x02\u{a4}\u{a5}\x03\x02\x02\x02\u{a5}\x1f\x03\
	\x02\x02\x02\u{a6}\u{a4}\x03\x02\x02\x02\u{a7}\u{ac}\x05\x22\x12\x02\u{a8}\
	\u{a9}\x09\x04\x02\x02\u{a9}\u{ab}\x05\x22\x12\x02\u{aa}\u{a8}\x03\x02\x02\
	\x02\u{ab}\u{ae}\x03\x02\x02\x02\u{ac}\u{aa}\x03\x02\x02\x02\u{ac}\u{ad}\
	\x03\x02\x02\x02\u{ad}\x21\x03\x02\x02\x02\u{ae}\u{ac}\x03\x02\x02\x02\u{af}\
	\u{b4}\x05\x24\x13\x02\u{b0}\u{b1}\x09\x05\x02\x02\u{b1}\u{b3}\x05\x24\x13\
	\x02\u{b2}\u{b0}\x03\x02\x02\x02\u{b3}\u{b6}\x03\x02\x02\x02\u{b4}\u{b2}\
	\x03\x02\x02\x02\u{b4}\u{b5}\x03\x02\x02\x02\u{b5}\x23\x03\x02\x02\x02\u{b6}\
	\u{b4}\x03\x02\x02\x02\u{b7}\u{bc}\x05\x26\x14\x02\u{b8}\u{b9}\x09\x06\x02\
	\x02\u{b9}\u{bb}\x05\x26\x14\x02\u{ba}\u{b8}\x03\x02\x02\x02\u{bb}\u{be}\
	\x03\x02\x02\x02\u{bc}\u{ba}\x03\x02\x02\x02\u{bc}\u{bd}\x03\x02\x02\x02\
	\u{bd}\x25\x03\x02\x02\x02\u{be}\u{bc}\x03\x02\x02\x02\u{bf}\u{c0}\x09\x07\
	\x02\x02\u{c0}\u{c3}\x05\x26\x14\x02\u{c1}\u{c3}\x05\x28\x15\x02\u{c2}\u{bf}\
	\x03\x02\x02\x02\u{c2}\u{c1}\x03\x02\x02\x02\u{c3}\x27\x03\x02\x02\x02\u{c4}\
	\u{d3}\x05\x2a\x16\x02\u{c5}\u{ce}\x07\x06\x02\x02\u{c6}\u{cb}\x05\x16\x0c\
	\x02\u{c7}\u{c8}\x07\x03\x02\x02\u{c8}\u{ca}\x05\x16\x0c\x02\u{c9}\u{c7}\
	\x03\x02\x02\x02\u{ca}\u{cd}\x03\x02\x02\x02\u{cb}\u{c9}\x03\x02\x02\x02\
	\u{cb}\u{cc}\x03\x02\x02\x02\u{cc}\u{cf}\x03\x02\x02\x02\u{cd}\u{cb}\x03\
	\x02\x02\x02\u{ce}\u{c6}\x03\x02\x02\x02\u{ce}\u{cf}\x03\x02\x02\x02\u{cf}\
	\u{d0}\x03\x02\x02\x02\u{d0}\u{d2}\x07\x07\x02\x02\u{d1}\u{c5}\x03\x02\x02\
	\x02\u{d2}\u{d5}\x03\x02\x02\x02\u{d3}\u{d1}\x03\x02\x02\x02\u{d3}\u{d4}\
	\x03\x02\x02\x02\u{d4}\x29\x03\x02\x02\x02\u{d5}\u{d3}\x03\x02\x02\x02\u{d6}\
	\u{e0}\x07\x1d\x02\x02\u{d7}\u{e0}\x07\x1e\x02\x02\u{d8}\u{e0}\x07\x2b\x02\
	\x02\u{d9}\u{e0}\x07\x2c\x02\x02\u{da}\u{e0}\x07\x2a\x02\x02\u{db}\u{dc}\
	\x07\x06\x02\x02\u{dc}\u{dd}\x05\x16\x0c\x02\u{dd}\u{de}\x07\x07\x02\x02\
	\u{de}\u{e0}\x03\x02\x02\x02\u{df}\u{d6}\x03\x02\x02\x02\u{df}\u{d7}\x03\
	\x02\x02\x02\u{df}\u{d8}\x03\x02\x02\x02\u{df}\u{d9}\x03\x02\x02\x02\u{df}\
	\u{da}\x03\x02\x02\x02\u{df}\u{db}\x03\x02\x02\x02\u{e0}\x2b\x03\x02\x02\
	\x02\x1b\x2e\x32\x3b\x41\x45\x57\x5d\x62\x66\x73\x76\x7b\u{87}\u{8d}\u{94}\
	\u{9c}\u{a4}\u{ac}\u{b4}\u{bc}\u{c2}\u{cb}\u{ce}\u{d3}\u{df}";

