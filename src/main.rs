
fn main(){
    let client= osrs_rtge::Client::new("Testing if rust lib is working - @stecool on Discord".to_string());
    
    let latest = osrs_rtge::latest(&client, None).unwrap();
    let latest = serde_json::to_string(&latest).expect("couldn't to string");
    print!("{latest:#?}");

    // let timeseries = osrs_rtge::timeseries(&client, "4151", "24h").unwrap();
    // let timeseries = serde_json::to_string(&timeseries).expect("couldn't to string");
    // println!("{timeseries:#?}");

}