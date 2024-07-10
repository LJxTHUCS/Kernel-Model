use super::ast::*;
use super::lexer::*;
use crate::error::Error;
use nom::bytes::complete::take;
use nom::combinator::{eof, map, verify};
use nom::error::{Error as NomError, ErrorKind};
use nom::multi::{many0, separated_list1};
use nom::sequence::tuple;
use nom::Err;
use nom::Finish;

pub type IResult<'a, Output> = nom::IResult<Tokens<'a>, Output>;

pub fn parse_kml(input: Tokens) -> Result<Model, Error> {
    let result = model(input).finish();
    match result {
        Ok((_, model)) => Ok(model),
        Err(_) => Err(Error::Syntax),
    }
}

fn model(input: Tokens) -> IResult<Model> {
    map(
        tuple((many0(event_def), kernel_def, eof)),
        |(event_defs, kernel_def, _)| Model {
            event_defs,
            kernel_def,
        },
    )(input)
}

fn event_def(input: Tokens) -> IResult<EventDef> {
    map(
        tuple((
            match_token(TokenKind::Event),
            identifier,
            match_token(TokenKind::LBrace),
            many0(event_action),
            match_token(TokenKind::RBrace),
        )),
        |(_, identifier, _, actions, _)| EventDef {
            name: identifier,
            body: actions,
        },
    )(input)
}

fn event_action(input: Tokens) -> IResult<EventAction> {
    let (input, token) = take(1usize)(input)?;
    let token = &token.tok[0];
    if token.kind.is_action() {
        Ok((input, token.kind.into()))
    } else {
        Err(Err::Error(NomError::new(input, ErrorKind::Tag)))
    }
}

fn kernel_def(input: Tokens) -> IResult<KernelDef> {
    map(
        tuple((
            match_token(TokenKind::Kernel),
            match_token(TokenKind::LBrace),
            many0(kernel_config),
            match_token(TokenKind::RBrace),
        )),
        |(_, _, configs, _)| KernelDef { configs },
    )(input)
}

fn kernel_config(input: Tokens) -> IResult<KernelConfig> {
    let (input, token) = take(1usize)(input)?;
    let token = &token.tok[0];
    match token.kind {
        TokenKind::Events => {
            event_config(input).map(|(input, events)| (input, KernelConfig::Events(events)))
        }
        TokenKind::Scheduler => scheduler_config(input)
            .map(|(input, scheduler)| (input, KernelConfig::Scheduler(scheduler))),
        _ => Err(Err::Error(NomError::new(input, ErrorKind::Tag))),
    }
}

fn event_config(input: Tokens) -> IResult<Vec<Identifier>> {
    map(
        tuple((
            match_token(TokenKind::Eq),
            match_token(TokenKind::LBracket),
            identifiers,
            match_token(TokenKind::RBracket),
        )),
        |(_, _, events, _)| events,
    )(input)
}

fn scheduler_config(input: Tokens) -> IResult<SchedulerType> {
    map(
        tuple((match_token(TokenKind::Eq), scheduler_type)),
        |(_, sched_type)| sched_type,
    )(input)
}

fn scheduler_type(input: Tokens) -> IResult<SchedulerType> {
    let (input, token) = take(1usize)(input)?;
    let token = &token.tok[0];
    match token.kind {
        TokenKind::Fifo => Ok((input, SchedulerType::Fifo)),
        _ => Err(Err::Error(NomError::new(input, ErrorKind::Tag))),
    }
}

// Utilities
fn match_token(kind: TokenKind) -> impl FnMut(Tokens) -> IResult<Tokens> {
    move |i| verify(take(1usize), |t: &Tokens| t.tok[0].kind == kind)(i)
}

// Low-level terminals
fn identifier(input: Tokens) -> IResult<Identifier> {
    let (input, token) = take(1usize)(input)?;
    let token = &token.tok[0];
    match token.kind {
        TokenKind::Identifier => Ok((input, Identifier(token.text().to_string()))),
        _ => Err(Err::Error(NomError::new(input, ErrorKind::Tag))),
    }
}

fn identifiers(input: Tokens) -> IResult<Vec<Identifier>> {
    separated_list1(match_token(TokenKind::Comma), identifier)(input)
}
