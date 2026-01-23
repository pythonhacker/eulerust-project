use euler_lib::{data_file, Grid};

// Maximum product in a Grid.
fn main() {

    let mut mygrid = Grid::new(20, 20);
	let fpath = data_file("problem11", "grid.txt");
    mygrid.read(fpath.display().to_string());

    let mut prod;
    let mut largest = 0;

    for x in 0..mygrid.width() {
        for y in 0..mygrid.height() {
            prod = mygrid.get_max_product(x, y);
            if prod > largest {
                largest = prod;
            }
        }
    }

    println!("{}", largest);
}
