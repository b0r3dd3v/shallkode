use nom;
use types::*;

fn make_section<'a>((name, exprs): (&'a str, Vec<Expr<'a>>)) -> Section<'a> {
    Section {
        name: name,
        exprs: exprs,
    }
}

pub fn parse(value: &str) -> Prompt {
    prompt(value).unwrap().1
}

named!(literal<&str, Expr>, map!(alt!(tag!("[") | tag!("]")), Expr::Literal));

named!(placeholder<&str, Expr>, map!(delimited!(char!('%'), nom::alpha, char!('%')), Expr::Placeholder));

named!(spec<&str, Vec<Expr> >, many0!(alt_complete!(literal | placeholder)));

named!(tagged_spec<&str, (&str, Vec<Expr>)>, separated_pair!(nom::alpha, char!(':'), spec));

named!(section<&str, Section>, map!(delimited!(char!('{'), tagged_spec, char!('}')), make_section));

named!(prompt<&str, Prompt>,
    map!(
        many0!(
            alt_complete!(
                literal     => { |literal|     TopLevelExpr::Expr(literal)     } |
                placeholder => { |placeholder| TopLevelExpr::Expr(placeholder) } |
                section     => { |section|     TopLevelExpr::Section(section)  }
            )
        ),
        |exprs| Prompt { exprs: exprs }
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(literal("[").unwrap(), ("", Expr::Literal("[")));
        assert_eq!(literal("]").unwrap(), ("", Expr::Literal("]")));
    }

    #[test]
    fn test_placeholder() {
        assert_eq!(placeholder("%hi%").unwrap(), ("", Expr::Placeholder("hi")));
    }

    #[test]
    fn test_spec() {
        assert_eq!(spec("[%hi%]").unwrap(), ("", vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]"))));
    }

    #[test]
    fn test_tagged_spec() {
        assert_eq!(tagged_spec("git:[%hi%]").unwrap(), ("", ("git", vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")))));
    }

    #[test]
    fn test_section() {
        assert_eq!(section("{git:[%hi%]}").unwrap(), ("", Section { name: "git", exprs: vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")) }));
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt("{rbenv:%version%}[]{git:[%hi%]}").unwrap(), ("", Prompt { exprs: vec![TopLevelExpr::Section(Section{ name: "rbenv", exprs: vec!(Expr::Placeholder("version")) }), TopLevelExpr::Expr(Expr::Literal("[")), TopLevelExpr::Expr(Expr::Literal("]")), TopLevelExpr::Section(Section{ name: "git", exprs: vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")) })] }));
    }
}