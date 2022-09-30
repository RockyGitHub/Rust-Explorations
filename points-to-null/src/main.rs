fn main() {
    println!("Hello, world!");
    let mut my_vec = Vec::new();
    
    let service = ServiceReportContext {
        uuid: [5u8; 16],
    };

    let service_pointer = Box::into_raw(Box::new(service));
    my_vec.push(service_pointer);

    {
        let a_thing = unsafe { Box::from_raw(service_pointer)};
    }

    println!("{}", my_vec[0].is_null());
    println!("{:?}", unsafe { Box::from_raw(my_vec[0]) }); // double free error


}

#[repr(C)]
#[derive(Debug)]
pub struct ServiceReportContext {
    pub uuid: [u8; 16],
}
