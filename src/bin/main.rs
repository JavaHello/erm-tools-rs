use erm_tools::core::ErmRead;
use erm_tools::core::MdOut;
use erm_tools::core::{Diff, OutDiff, TableDiff};

fn main() {
    let mut erm1 = ErmRead::new(vec!["erms/db.erm".to_owned()]);
    let mut erm2 = ErmRead::new(vec!["erms/db2.erm".to_owned()]);
    let mut diff = TableDiff::new(&mut erm1.talbes, &mut erm2.talbes);
    diff.diff();
    let mut out = MdOut::new();
    out.write(&diff.diff);
    println!("{}", out.content);
}
