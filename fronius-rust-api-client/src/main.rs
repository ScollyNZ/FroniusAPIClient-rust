use reqwest::Result;
// use serde::Deserialize;

fn main() -> Result<()> {
    //let resp = reqwest::blocking::get("http://192.168.1.85/status/powerflow")?.text()?;
    //println!("{:#?}", resp);

    let response = (Fronius24Inverter{
        ip_address : String::from("192.168.1.85")
    }).get_reading();

    // Convert the JSON string back to a Point.
    //let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    Ok(())
}

/*pub trait InverterReading {
    fn load(&self) -> f64;

    fn pv(&self) -> f64;

    fn grid(&self) -> f64;
}*/

/*pub trait Inverter {
    fn get_reading(&self) -> impl InverterReading;
}*/

/*pub struct Fronius24InverterReading {

}*/

/*impl InverterReading for Fronius24InverterReading {
    fn grid(&self) -> f64 {
        f64::from_bits(todo!())
    }
    
    fn load(&self) -> f64 {
        f64::from_bits(todo!())
    }
    
    fn pv(&self) -> f64 {
        f64::from_bits(todo!())
    }
}*/

pub struct Fronius24Inverter{
    pub ip_address:String
}

impl Fronius24Inverter {
    fn get_reading(&self) -> () {
        let resp = reqwest::blocking::get(format!("http://{}/status/powerflow", self.ip_address));
        println!("{:#?}", resp);
    }
}

/*impl Inverter for Fronius24Inverter {
    fn get_reading(&self) -> InverterReading {
        
    }
}*/


/*fn main() -> Result<()> {
    
    let response = reqwest::get("https://example.com");

    match response {
        Ok(success) => {
            // Process the successful response
        },
        Err(e) => {
            // Handle the error
            eprintln!("Request failed: {}", e);
        }
    }*/


