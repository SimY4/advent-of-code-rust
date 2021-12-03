use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Route(String, String, u8);

fn parse_line(line: &str) -> Route {
    match line.split_whitespace().collect::<Vec<&str>>()[..5] {
        [from, "to", to, "=", distance] => Route(
            from.to_string(),
            to.to_string(),
            distance.parse::<u8>().unwrap(),
        ),
        _ => unreachable!(),
    }
}

fn routes(input: &str) -> HashMap<String, Vec<Route>> {
    input
        .lines()
        .flat_map(|line| {
            let route = parse_line(line);
            let Route(from, to, distance) = route.clone();
            vec![route, Route(to, from, distance)]
        })
        .fold(
            &mut HashMap::new(),
            |acc: &mut HashMap<String, Vec<Route>>, route| {
                let Route(from, _, _) = route.clone();
                let vec = acc.entry(from).or_insert(vec![]);
                vec.push(route);
                acc
            },
        )
        .clone()
}

fn expand_path(path: Vec<Route>, rest: HashMap<String, Vec<Route>>) -> Vec<Vec<Route>> {
    match rest
        .get(&path[0].1)
        .map(|next_routes| {
            next_routes
                .iter()
                .filter(|next_route| path.iter().all(|r| r.0 != next_route.1))
                .cloned()
                .collect::<Vec<Route>>()
        })
        .filter(|next_routes| next_routes.len() > 0)
    {
        Some(next_routes) => next_routes
            .iter()
            .flat_map(|next_route| {
                let new_path = vec![vec![next_route.clone()], path.clone()].concat();
                let mut new_rest = rest.clone();
                new_rest.remove(&path[0].1);
                expand_path(new_path, new_rest)
            })
            .collect::<Vec<Vec<Route>>>(),
        None => vec![path],
    }
}

fn paths(routes: HashMap<String, Vec<Route>>) -> Vec<Vec<Route>> {
    routes
        .values()
        .flatten()
        .cloned()
        .flat_map(|initial_route| {
            let mut rest = routes.clone();
            rest.remove(&initial_route.0);
            expand_path(vec![initial_route], rest)
        })
        .filter(|path| path.len() >= (routes.len() / 2) - 1)
        .collect::<Vec<Vec<Route>>>()
}

pub fn solve(input: &str) -> Option<u32> {
    let routes = routes(input);
    paths(routes)
        .iter()
        .map(|path| path.iter().map(|route| route.2 as u32).sum::<u32>())
        .min()
}

pub fn solve2(input: &str) -> Option<u32> {
    let routes = routes(input);
    paths(routes)
        .iter()
        .map(|path| path.iter().map(|route| route.2 as u32).sum::<u32>())
        .max()
}

pub const INPUT: &str = "AlphaCentauri to Snowdin = 66\n\
                         AlphaCentauri to Tambi = 28\n\
                         AlphaCentauri to Faerun = 60\n\
                         AlphaCentauri to Norrath = 34\n\
                         AlphaCentauri to Straylight = 34\n\
                         AlphaCentauri to Tristram = 3\n\
                         AlphaCentauri to Arbre = 108\n\
                         Snowdin to Tambi = 22\n\
                         Snowdin to Faerun = 12\n\
                         Snowdin to Norrath = 91\n\
                         Snowdin to Straylight = 121\n\
                         Snowdin to Tristram = 111\n\
                         Snowdin to Arbre = 71\n\
                         Tambi to Faerun = 39\n\
                         Tambi to Norrath = 113\n\
                         Tambi to Straylight = 130\n\
                         Tambi to Tristram = 35\n\
                         Tambi to Arbre = 40\n\
                         Faerun to Norrath = 63\n\
                         Faerun to Straylight = 21\n\
                         Faerun to Tristram = 57\n\
                         Faerun to Arbre = 83\n\
                         Norrath to Straylight = 9\n\
                         Norrath to Tristram = 50\n\
                         Norrath to Arbre = 60\n\
                         Straylight to Tristram = 27\n\
                         Straylight to Arbre = 81\n\
                         Tristram to Arbre = 90";
