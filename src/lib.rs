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
            "landsat:wrs_path IN ('153','154','155')",
            "eq:cloud_cover < 0.1 AND landsat:wrs_row=28 AND landsat:wrs_path=203",
            "ro:cloud_cover=0.1 OR ro:cloud_cover=0.2",
            "ro:cloud_cover IN (0.1,0.2)",
            "eo:cloud_cover BETWEEN 0.1 AND 0.2
                AND landsat:wrs_row=28
                AND landsat:wrs_path=203",
            "eo:cloud_cover >= 0.1 AND eo:cloud_cover <= 0.2
                AND landsat:wrs_row=28
                AND landsat:wrs_path=203",
            "eo_instruments LIKE 'OLI%' AND INTERSECTS(footprint,POLYGON((43.5845 -79.5442, 43.6079 -79.4893, 43.5677 -79.4632, 43.6129 -79.3925, 43.6223 -79.3238, 43.6576 -79.3163, 43.7945 -79.1178, 43.8144 -79.1542, 43.8555 -79.1714, 43.7509 -79.6390, 43.5845 -79.5442)))",
            "beamMode='ScanSAR Narrow' AND
                swathDirection='ascending' AND
                polarization='HH+VV+HV+VH' AND
                intersects(geometry,POLYGON((-77.117938 38.936860,-77.040604 39.995648,-76.910536 38.892912,-77.039359 38.791753,-77.047906 38.841462,-77.034183 38.840655,-77.033142 38.857490,-77.117938 38.936860)))",
            "floors>5",
            "taxes <= 500",
            "owner LIKE '%Jones%'",
            "owner LIKE 'Mike%'",
            "owner NOT LIKE '%Mike%'",
            "swimming_pool = true",
            "floors>5 AND swimming_pool=true",
            "swimming_pool=true AND (floors>5 OR
                material LIKE 'brick%' OR
                material LIKE '%brick')",
            "(floors>5 AND material='brick') OR swimming_pool=true",
            "NOT (floors<5) OR swimming_pool=true",
            "(owner LIKE 'mike%' OR owner LIKE 'Mike%') AND floors<4",
            "built BEFORE 2015-01-01",
            "built AFTER 2012-06-05",
            "updated DURING 2017-06-10T07:30:00Z/2017-06-11T10:30:00Z",
            "WITHIN(location,ENVELOPE(-118,33.8,-117.9,34))",
            "INTERSECTS(geometry,POLYGON((-10.0 -10.0,10.0 -10.0,10.0 10.0,-10.0 -10.0)))",
            "floors>5 AND WITHIN(geometry,ENVELOPE(-118,33.8,-117.9,34))",
            ];
        for example in &examples {
            println!("Query: {}", example);
            let query = CqlParser::parse(Rule::cql, &example)
                .expect("Failed to parse query");
            println!("{:#?}", query);
        }
    }
}
