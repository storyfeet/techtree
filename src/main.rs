use lazy_conf::Loader;

fn main() -> Result<(), String> {
    let mut cfg = lazy_conf::config("-c", &["{PWD}/conf.lz", "{HOME}/.config/techtree/conf.lz"])
        .map_err(|_| "args not clear".to_string())?;

    let dip = cfg
        .grab()
        .fg("-d")
        .cf("diplomacy.cards")
        .help("Diplomacy Card Location")
        .s_local();

    if cfg.help("Diplomacy") {
        return Ok(());
    }

    let dip = lazy_conf::LzList::load(dip.ok_or("No Diplomacy supplied")?)
        .map_err(|_| "could not load diplomacy file")?;

    for i in dip.items {
        println!("{:?}", i);
    }

    Ok(())
}
