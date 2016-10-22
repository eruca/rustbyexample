use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
	let users = init_data();

	match compute_nearest_neighbor("Hailey", &users){
		None => {println!("find nothing");},
		Some(v) => {println!("{:?}", v);}
	}
}

fn manhattan(m1: &HashMap<&'static str,f64>, m2: &HashMap<&'static str,f64>) -> f64 {
	m1.iter().filter_map(|(k, &v)| {
		match m2.get(k) {
			Some(&v2) => Some((v2 - v).abs()),
			None => None,
		}
	}).sum()
}

#[test]
fn test_manhattan(){
	let users = init_data();
	let m1 = users.get("Hailey").unwrap();
	let m2 = users.get("Veronica").unwrap();

	assert_eq!(manhattan(&m1, &m2), 2.0);

	let m2 = users.get("Jordyn").unwrap();
	assert_eq!(manhattan(&m1, &m2), 7.5);
}

fn compute_nearest_neighbor(name:&str, users:&HashMap<&'static str, HashMap<&'static str,f64>>) -> Option<Vec<(&'static str, f64)>> {
	let mut output = users.iter().fold(Vec::new(), |mut output, (&k, map)|{
		if k != name {
			let distance = manhattan(users.get(name).unwrap(), map);
			output.push((k,distance));
		}

		output
	});

	if output.len() == 0 {
		None
	} else {
		output.sort_by(|a, b|{
			// a.1.partial_cmp(&b.1)
			match a.1.partial_cmp(&b.1) {
				Some(Ordering::Equal) => a.0.cmp(&b.0),
				Some(x) => x,
				None => Ordering::Less,
			}
		});
		Some(output)
	}
}

fn init_data() -> Box<HashMap<&'static str, HashMap<&'static str,f64>>> {
	let mut map = Box::new(HashMap::new());

	let mut map_0 = HashMap::new();
	let mut map_1 = HashMap::new();
	let mut map_2 = HashMap::new();
	let mut map_3 = HashMap::new();
	let mut map_4 = HashMap::new();
	let mut map_5 = HashMap::new();
	let mut map_6 = HashMap::new();
	let mut map_7 = HashMap::new();

	map_0.insert("Blues Traveler", 3.5);
	map_0.insert("Broken Bells", 2.0);
	map_0.insert("Norah Jones", 4.5);
	map_0.insert("Phoenix", 5.0);
	map_0.insert("Slightly Stoopid", 1.5);
	map_0.insert("The Strokes", 2.5);
	map_0.insert("Vampire Weekend", 2.0);

	map_1.insert("Blues Traveler", 2.0);
	map_1.insert("Broken Bells", 3.5);
	map_1.insert("Deadmau5", 4.0);
	map_1.insert("Phoenix", 2.0);
	map_1.insert("Slightly Stoopid", 3.5);
	map_1.insert("Vampire Weekend", 3.0);

	map_2.insert("Blues Traveler", 5.0);
	map_2.insert("Broken Bells", 3.0);
	map_2.insert("Deadmau5", 1.0);
	map_2.insert("Norah Jones", 3.0);
	map_2.insert("Phoenix", 5.0);
	map_2.insert("Slightly Stoopid", 1.0);

	map_3.insert("Blues Traveler",3.0);
	map_3.insert("Broken Bells", 4.0);
	map_3.insert("Deadmau5", 4.5);
	map_3.insert("Phoenix", 3.0);
	map_3.insert("Slightly Stoopid", 4.5);
	map_3.insert("The Strokes", 4.0);
	map_3.insert("Vampire Weekend", 2.0);

	map_4.insert("Broken Bells", 4.0);
	map_4.insert("Deadmau5", 1.0);
	map_4.insert("Norah Jones", 4.0);
	map_4.insert("The Strokes", 4.0);
	map_4.insert("Vampire Weekend", 1.0);

	map_5.insert("Broken Bells", 4.5);
	map_5.insert("Deadmau5", 4.0);
	map_5.insert("Norah Jones", 5.0);
	map_5.insert("Phoenix", 5.0);
	map_5.insert("Slightly Stoopid",4.5);
	map_5.insert("The Strokes", 4.0);
	map_5.insert("Vampire Weekend", 4.0);

	map_6.insert("Blues Traveler",5.0);
	map_6.insert("Broken Bells",2.0);
	map_6.insert("Norah Jones", 3.0);
	map_6.insert("Phoenix", 5.0);
	map_6.insert("Slightly Stoopid", 4.0);
	map_6.insert("The Strokes", 5.0);

	map_7.insert("Blues Traveler",3.0);
	map_7.insert("Norah Jones", 5.0);
	map_7.insert("Phoenix", 4.0);
	map_7.insert("Slightly Stoopid", 2.5);
	map_7.insert("The Strokes", 3.0);

	map.insert("Angelica", map_0);
	map.insert("Bill", map_1);
	map.insert("Chan", map_2);
	map.insert("Dan", map_3);
	map.insert("Hailey", map_4);
	map.insert("Jordyn", map_5);
	map.insert("Sam", map_6);
	map.insert("Veronica", map_7);

	map
}