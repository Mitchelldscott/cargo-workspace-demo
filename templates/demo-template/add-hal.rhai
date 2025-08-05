let hal = values.hal_crate;

print("Adding HAL crate: " + hal);
let result = system::command("cargo", ["add", hal]);

print(result);