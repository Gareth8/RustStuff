#[path = "functions/control_flow_mod.rs"] mod control_flow_mod;
#[path = "functions/ownership_mod.rs"] mod ownership_mod;
#[path = "functions/references_and_borrowing_mod.rs"] mod references_and_borrowing_mod;
#[path = "functions/slices_mod.rs"] mod slices_mod;
#[path = "functions/structs_mod.rs"] mod structs_mod;
#[path = "functions/rectangles_mod.rs"] mod rectangles_mod;
#[path = "functions/enums_mod.rs"] mod enums_mod;

pub fn control_flow() {
    control_flow_mod::control_flow();
}
pub fn ownership() {
    ownership_mod::ownership();
}
pub fn references_and_borrowing() {
    references_and_borrowing_mod::references_and_borrowing();
}
pub fn slices() {
    slices_mod::slices();
}
pub fn structs() {
    structs_mod::structs();
}
pub fn rectangles() {
    rectangles_mod::rectangles();
}
pub fn enums() {
    enums_mod::enums();
}
pub fn control_flow_two() {
    control_flow_mod::control_flow_two();
}