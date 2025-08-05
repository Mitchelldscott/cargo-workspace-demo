let hal = variable::get("hal_crate");

print(system::command("ls"));

print("Adding HAL crate: " + hal);
let result = system::command("cargo", ["add", hal]);

print(result);
