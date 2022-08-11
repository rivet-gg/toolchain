use tabled::{Table, Tabled};

pub fn table<T>(iter: impl IntoIterator<Item = T>) 
where
	T: Tabled,
{
	println!("{}", Table::new(iter).with(tabled::Style::rounded()));
}
