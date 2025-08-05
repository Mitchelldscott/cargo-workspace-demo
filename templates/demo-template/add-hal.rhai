let hal = variable::get("hal_crate");

print("Adding HAL crate: " + hal);
print(system::command("cargo", ["add", hal]));
