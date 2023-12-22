use crate::atom::app::App;
use super::Ready;

impl Ready<(), ()> for App {
    fn ready(&mut self, _: ()) -> () {
        println!("App is ready");
            
    }
}
