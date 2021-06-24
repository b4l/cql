extern crate pest;
#[macro_use]
extern crate pest_derive;

// use pest::Parser;

// #[derive(Parser)]
// #[grammar = "../schema/cql.pest"]
// struct CqlParser;

#[cfg(test)]
mod tests {
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "../schema/cql.pest"]
    struct CqlParser;

    #[test]
    fn parse_pest() {
        let examples = [
            "landsat:scene_id = 'LC82030282019133LGN00'",
            "eo:instrument LIKE 'OLI#' WILDCARD '#'",
        ];
        for example in examples {
            println!("Query: {}", example);
            let query = CqlParser::parse(Rule::booleanValueExpression, &example)
                .expect("Failed to parse query");
            println!("{:?}", query);
        }
    }
}
