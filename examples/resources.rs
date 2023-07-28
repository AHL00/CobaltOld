use cobalt::resources::ResourceManager;

fn main() {
    let mut res_manager = ResourceManager::new();

    let res = res_manager.create(5);

    println!("Res: {:?}", res);
    println!("Value: {:?}", res_manager.get(&res));
}