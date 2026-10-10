let crate_type = variable::get("crate_type");
debug(`crate_type: ${crate_type}`);

let license = variable::get("license").to_upper();
while switch license {
  "APACHE" => {
    file::delete("LICENSE-MIT");
    file::rename("LICENSE-APACHE", "LICENSE");
    false
  }
  "MIT" => {
    file::delete("LICENSE-APACHE");
    file::rename("LICENSE-MIT", "LICENSE");
    false
  }
  _ => true,
} {
  license = variable::prompt("Select license?", "MIT", [
    "APACHE",
    "MIT",
  ]);
}
variable::set("license", license);