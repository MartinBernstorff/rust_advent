use std::str::Lines;

// Parse the instructions one by one
// Create "dir" entries with a parent and contents
// Turn those into a tree
// Figure out how to parse them
use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
pub struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
pub struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
struct TreeNode {
    path: Utf8PathBuf,
    size: u64,
}

#[derive(Debug)]
enum FilesystemEntry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, FilesystemEntry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| FilesystemEntry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), FilesystemEntry::Dir);

    alt((parse_file, parse_dir))(i)
}
#[derive(Debug)]
enum ParsedLine {
    Command(Command),
    Entry(FilesystemEntry),
}

fn parse_line(i: &str) -> IResult<&str, ParsedLine> {
    alt((
        map(parse_command, ParsedLine::Command),
        map(parse_entry, ParsedLine::Entry),
    ))(i)
}

fn subtree_size(tree: &Tree<TreeNode>, node: &Node<TreeNode>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += subtree_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn construct_tree() -> color_eyre::Result<Tree<TreeNode>> {
    color_eyre::install()?;

    let input = include_str!("pt1.txt");
    let parsed_lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<TreeNode>::new();

    let root = tree.insert(
        Node::new(TreeNode {
            path: "/".into(),
            size: 0,
        }),
        id_tree::InsertBehavior::AsRoot,
    )?;

    // Let the state begin
    let mut curr = root;
    for parsed_line in parsed_lines {
        println!("{parsed_line:?}");

        match parsed_line {
            ParsedLine::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, starting here
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(TreeNode {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, id_tree::InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            ParsedLine::Entry(entry) => match entry {
                FilesystemEntry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into it
                }
                FilesystemEntry::File(size, path) => {
                    let node = Node::new(TreeNode { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    Ok(tree)
}

fn main() {
    let tree = construct_tree().unwrap();

    let total_size = 70000000;
    let needed_size = 30000000;
    let current_useage =
        subtree_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap()).unwrap();

    let free_size = total_size - current_useage;

    let size_to_free = needed_size - free_size;

    let smallest_size_to_remove = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty()) // Keep only directories
        .map(|n| subtree_size(&tree, &n).unwrap())
        .filter(|&n| n > size_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();

    dbg!(smallest_size_to_remove);
}

#[cfg(test)]
mod tests {
    use camino::Utf8Path;
    use nom::{combinator::all_consuming, Finish};

    use crate::{main, parse_line};

    #[test]
    fn test_starting_input() {
        main()
    }
}
